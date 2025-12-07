use std::ffi::CStr;
use std::io::{self, ErrorKind};
use std::path::PathBuf;
use tokio::fs::{File, OpenOptions};

use super::MountBuilder;

const FS_TYPE: &CStr = unsafe {
    CStr::from_bytes_with_nul_unchecked(b"fuse\0")
};

macro_rules! io_error {
    ($kind:expr, $msg:literal $($args:tt)*) => {{
        return Err(std::io::Error::new(
            $kind,
            format!($msg $($args)*)
        ));
    }};
}

/// An handle to a mounted FUSE file system.
pub struct Mount {
    fuse_dev: File,
    mountpoint: PathBuf,
}

impl Mount {
    pub fn builder(
        mountpoint: impl Into<PathBuf>,
        fs_name: impl Into<String>,
    ) -> MountBuilder {
        MountBuilder::new(mountpoint, fs_name)
    }

    pub(super) async fn from_builder(builder: MountBuilder) -> io::Result<Self> {
        const FUSE_DEVICE: &str = "/dev/fuse";

        let fuse_dev = OpenOptions::new()
            .read(true)
            .write(true)
            .open(FUSE_DEVICE)
            .await;

        let fuse_dev = match fuse_dev {
            Ok(dev) => dev,
            Err(e) if e.kind() == ErrorKind::NotFound => io_error!(
                ErrorKind::NotFound,
                "FUSE device file not fount `{FUSE_DEVICE}`. Try `modprobe fuse`"
            ),
            Err(e) => return Err(e)
        };

        let mount_options = builder.into_options(&fuse_dev).await?;

        let result = unsafe {
            libc::mount(
                mount_options.source.as_ptr(),
                mount_options.target.as_ptr(),
                FS_TYPE.as_ptr(),
                mount_options.flags,
                mount_options.options.as_ptr().cast()
            )
        };

        if result == -1 {
            let err = io::Error::last_os_error();
            io_error!(err.kind(), "Failed to call mount(): {err}");
        }

        Ok(Self {
            fuse_dev,
            mountpoint: mount_options.mountpoint,
        })
    }
}
