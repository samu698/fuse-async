use std::io;
use std::ffi::CString;
use std::os::fd::AsRawFd;
use std::os::unix::ffi::OsStrExt;
use std::os::unix::fs::MetadataExt;
use std::path::PathBuf;
use std::borrow::Cow;

use tokio::fs;

use super::Mount;

macro_rules! flag_setters {
    ($(pub fn $name:ident($flag:expr);)*) => {$(
        #[inline]
        #[must_use = "A MountBuilder will do noting unless you call `.build()`"]
        pub fn $name(mut self, set: bool) -> Self {
            if set { self.flags |= $flag; }
            else { self.flags &= !($flag); }
            self
        }
    )*};
}

#[derive(Debug)]
pub struct MountBuilder {
    mountpoint: PathBuf,

    // Mount options
    fsname: CString,
    subtype: Option<CString>,
    rootmode: Option<u16>,
    default_permissions: bool,
    allow_other: bool,
    max_read: Option<usize>,

    // Mount flags
    flags: u64,
}

pub(super) struct MountOptions {
    pub(super) mountpoint: PathBuf,
    pub(super) source: CString,
    pub(super) target: CString,
    pub(super) flags: u64,
    pub(super) options: CString,
}

impl MountBuilder {
    #[must_use = "A MountBuilder will do noting unless you call `.build()`"]
    pub fn new(
        mountpoint: impl Into<PathBuf>,
        fsname: impl Into<String>,
    ) -> Self {
        // TODO: check that fsname is valid (no commas for example)
        // TODO: remove the expect
        let fsname = CString::new(fsname.into().into_bytes())
            .expect("The filesystem name cannot contain NUL bytes");

        Self {
            mountpoint: mountpoint.into(),
            fsname,
            subtype: None,
            rootmode: None,
            default_permissions: false,
            allow_other: false,
            max_read: None,
            flags: 0,
        }
    }

    #[inline]
    #[must_use = "A MountBuilder will do noting unless you call `.build()`"]
    pub fn root_mode(mut self, mode: u16) -> Self {
        self.rootmode = Some(mode);
        self
    }

    #[inline]
    #[must_use = "A MountBuilder will do noting unless you call `.build()`"]
    pub fn default_permissions(mut self, enable: bool) -> Self {
        self.default_permissions = enable;
        self
    }

    #[inline]
    #[must_use = "A MountBuilder will do noting unless you call `.build()`"]
    pub fn allow_other(mut self, allow: bool) -> Self {
        self.allow_other = allow;
        self
    }

    #[inline]
    #[must_use = "A MountBuilder will do noting unless you call `.build()`"]
    pub fn max_read(mut self, size: usize) -> Self {
        self.max_read = Some(size);
        self
    }

    #[inline]
    #[must_use = "A MountBuilder will do noting unless you call `.build()`"]
    pub fn max_read_unlimited(mut self) -> Self {
        self.max_read = None;
        self
    }

    #[inline]
    #[must_use = "A MountBuilder will do noting unless you call `.build()`"]
    pub fn subtype<'b>(
        mut self,
        subtype: impl Into<Cow<'b, str>>
    ) -> Self {
        let subtype = subtype.into().into_owned();
        // TODO: remove the expect
        let subtype = CString::new(subtype.into_bytes())
            .expect("The subtype cannot contain NUL bytes");
        self.subtype = Some(subtype);
        self
    }

    flag_setters!{
        pub fn dirsync(libc::MS_DIRSYNC);
        pub fn noatime(libc::MS_NOATIME);
        pub fn nodev(libc::MS_NODEV);
        pub fn nodiratime(libc::MS_NODIRATIME);
        pub fn noexec(libc::MS_NOEXEC);
        pub fn nosuid(libc::MS_NOSUID);
        pub fn rdonly(libc::MS_RDONLY);
        pub fn synchronous(libc::MS_SYNCHRONOUS);
    }

    pub async fn build(self) -> io::Result<Mount> {
        Mount::from_builder(self).await
    }

    pub(super) async fn into_options(
        self,
        fuse_dev: &impl AsRawFd
    ) -> io::Result<MountOptions> {
        let target = self.mountpoint
            .as_os_str()
            .as_bytes()
            .to_vec();

        // TODO: remove the expect
        let target = CString::new(target)
            .expect("The mountpoint cannot contain NUL bytes");

        let rootmode = match self.rootmode {
            Some(mode) => mode,
            None => fs::metadata(&self.mountpoint).await?
                .mode() as u16
        };
        let uid = unsafe { libc::getuid() };
        let gid = unsafe { libc::getgid() };
        let fd = fuse_dev.as_raw_fd();

        let mut options = format!("fd={fd},rootmode={rootmode:o},user_id={uid},group_id={gid},fsname=")
            .into_bytes();

        options.extend_from_slice(self.fsname.as_bytes());

        if let Some(subtype) = self.subtype {
            options.extend_from_slice(b",subtype=");
            options.extend_from_slice(subtype.as_bytes());
        }

        if self.default_permissions {
            options.extend_from_slice(b",default_permissions");
        }

        if self.allow_other {
            options.extend_from_slice(b",allow_other");
        }

        if let Some(size) = self.max_read {
            options.extend_from_slice(format!("max_read={size}").as_bytes());
        }
        
        // SAFETY: The genereted options string cannot contain NUL bytes, 
        //         because it is built only from static byte strings, CStrings,
        //         and stringified numbers
        let options = unsafe { CString::from_vec_unchecked(options) };

        Ok(MountOptions {
            mountpoint: self.mountpoint,
            source: self.fsname,
            target,
            flags: self.flags,
            options,
        })
    }
}
