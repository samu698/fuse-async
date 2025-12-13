#![warn(non_camel_case_types)]

use zerocopy::{KnownLayout, Immutable, FromBytes, IntoBytes};
use bitflags::bitflags;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[derive(KnownLayout, Immutable, FromBytes, IntoBytes)]
pub struct GetattrFlags(u32);

#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[derive(KnownLayout, Immutable, FromBytes, IntoBytes)]
pub struct AttrFlags(u32);

#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[derive(KnownLayout, Immutable, FromBytes, IntoBytes)]
pub struct SetattrValid(u32);

#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[derive(KnownLayout, Immutable, FromBytes, IntoBytes)]
pub struct OpenInFlags(u32);

#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[derive(KnownLayout, Immutable, FromBytes, IntoBytes)]
pub struct OpenOutFlags(u32);

#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[derive(KnownLayout, Immutable, FromBytes, IntoBytes)]
pub struct ReadFlags(u32);

#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[derive(KnownLayout, Immutable, FromBytes, IntoBytes)]
pub struct WriteFlags(u32);

#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[derive(KnownLayout, Immutable, FromBytes, IntoBytes)]
pub struct ReleaseFlags(u32);

#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[derive(KnownLayout, Immutable, FromBytes, IntoBytes)]
pub struct FsyncFlags(u32);

#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[derive(KnownLayout, Immutable, FromBytes, IntoBytes)]
pub struct SetxattrFlags(u32);

#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[derive(KnownLayout, Immutable, FromBytes, IntoBytes)]
pub struct InitFlags(u32);

#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[derive(KnownLayout, Immutable, FromBytes, IntoBytes)]
pub struct InitFlags2(u32);

#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[derive(KnownLayout, Immutable, FromBytes, IntoBytes)]
pub struct LockFlags(u32);

#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[derive(KnownLayout, Immutable, FromBytes, IntoBytes)]
pub struct IoctlFlags(u32);

#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[derive(KnownLayout, Immutable, FromBytes, IntoBytes)]
pub struct PollFlags(u32);

#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[derive(KnownLayout, Immutable, FromBytes, IntoBytes)]
pub struct SetupMappingFlags(u64);

bitflags! {
    impl GetattrFlags: u32 {
        const FUSE_GETATTR_FH = 1 << 0;
    }

    impl AttrFlags: u32 {
        const FUSE_ATTR_SUBMOUNT = 1 << 0;
        const FUSE_ATTR_DAX = 1 << 1;
    }

    impl SetattrValid: u32 {
        const FATTR_MODE = 1 << 0;
        const FATTR_UID = 1 << 1;
        const FATTR_GID = 1 << 2;
        const FATTR_SIZE = 1 << 3;
        const FATTR_ATIME = 1 << 4;
        const FATTR_MTIME = 1 << 5;
        const FATTR_FH = 1 << 6;
        const FATTR_ATIME_NOW = 1 << 7;
        const FATTR_MTIME_NOW = 1 << 8;
        const FATTR_LOCKOWNER = 1 << 9;
        const FATTR_CTIME = 1 << 10;
        const FATTR_KILL_SUIDGID = 1 << 11;
    }

    impl OpenInFlags: u32 {
        const FUSE_OPEN_KILL_SUIDGID = 1 << 0;
    }

    impl OpenOutFlags: u32 {
        const FOPEN_DIRECT_IO = 1 << 0;
        const FOPEN_KEEP_CACHE = 1 << 1;
        const FOPEN_NONSEEKABLE = 1 << 2;
        const FOPEN_CACHE_DIR = 1 << 3;
        const FOPEN_STREAM = 1 << 4;
        const FOPEN_NOFLUSH = 1 << 5;
        const FOPEN_PARALLEL_DIRECT_WRITES = 1 << 6;
        const FOPEN_PASSTHROUGH = 1 << 7;
    }

    impl ReadFlags: u32 {
        const FUSE_READ_LOCKOWNER = 1 << 1;
    }

    impl WriteFlags: u32 {
        const FUSE_WRITE_CACHE = 1 << 0;
        const FUSE_WRITE_LOCKOWNER = 1 << 1;
        const FUSE_WRITE_KILL_SUIDGID = 1 << 2;
    }

    impl ReleaseFlags: u32 {
        const FUSE_RELEASE_FLUSH = 1 << 0;
        const FUSE_RELEASE_FLOCK_UNLOCK = 1 << 1;
    }

    impl FsyncFlags: u32 {
        const FUSE_FSYNC_FDATASYNC = 1 << 0;
    }

    impl SetxattrFlags: u32 {
        const FUSE_SETXATTR_ACL_KILL_SGID = 1 << 0;
    }

    impl InitFlags: u32 {
        const FUSE_ASYNC_READ = 1 << 0;
        const FUSE_POSIX_LOCKS = 1 << 1;
        const FUSE_FILE_OPS = 1 << 2;
        const FUSE_ATOMIC_O_TRUNC = 1 << 3;
        const FUSE_EXPORT_SUPPORT = 1 << 4;
        const FUSE_BIG_WRITES = 1 << 5;
        const FUSE_DONT_MASK = 1 << 6;
        const FUSE_SPLICE_WRITE = 1 << 7;
        const FUSE_SPLICE_MOVE = 1 << 8;
        const FUSE_SPLICE_READ = 1 << 9;
        const FUSE_FLOCK_LOCKS = 1 << 10;
        const FUSE_HAS_IOCTL_DIR = 1 << 11;
        const FUSE_AUTO_INVAL_DATA = 1 << 12;
        const FUSE_DO_READDIRPLUS = 1 << 13;
        const FUSE_READDIRPLUS_AUTO = 1 << 14;
        const FUSE_ASYNC_DIO = 1 << 15;
        const FUSE_WRITEBACK_CACHE = 1 << 16;
        const FUSE_NO_OPEN_SUPPORT = 1 << 17;
        const FUSE_PARALLEL_DIROPS = 1 << 18;
        const FUSE_HANDLE_KILLPRIV = 1 << 19;
        const FUSE_POSIX_ACL = 1 << 20;
        const FUSE_ABORT_ERROR = 1 << 21;
        const FUSE_MAX_PAGES = 1 << 22;
        const FUSE_CACHE_SYMLINKS = 1 << 23;
        const FUSE_NO_OPENDIR_SUPPORT = 1 << 24;
        const FUSE_EXPLICIT_INVAL_DATA = 1 << 25;
        const FUSE_MAP_ALIGNMENT = 1 << 26;
        const FUSE_SUBMOUNTS = 1 << 27;
        const FUSE_HANDLE_KILLPRIV_V2 = 1 << 28;
        const FUSE_SETXATTR_EXT = 1 << 29;
        const FUSE_INIT_EXT = 1 << 30;
    }

    impl InitFlags2: u32 {
        const FUSE_SECURITY_CTX = 1 << 0;
        const FUSE_HAS_INODE_DAX = 1 << 1;
        const FUSE_CREATE_SUPP_GROUP = 1 << 2;
        const FUSE_HAS_EXPIRE_ONLY = 1 << 3;
        const FUSE_DIRECT_IO_ALLOW_MMAP = 1 << 4;
        const FUSE_PASSTHROUGH = 1 << 5;
        const FUSE_NO_EXPORT_SUPPORT = 1 << 6;
        const FUSE_HAS_RESEND = 1 << 7;
        const FUSE_ALLOW_IDMAP = 1 << 8;
        const FUSE_OVER_IO_URING = 1 << 9;
        const FUSE_REQUEST_TIMEOUT = 1 << 10;
    }

    impl LockFlags: u32 {
        const FUSE_LK_FLOCK = 1 << 0;
    }

    impl IoctlFlags: u32 {
        const FUSE_IOCTL_COMPAT = 1 << 0;
        const FUSE_IOCTL_UNRESTRICTED = 1 << 1;
        const FUSE_IOCTL_RETRY = 1 << 2;
        const FUSE_IOCTL_32BIT = 1 << 3;
        const FUSE_IOCTL_DIR = 1 << 4;
        const FUSE_IOCTL_COMPAT_X32 = 1 << 5;
    }

    impl PollFlags: u32 {
        const FUSE_POLL_SCHEDULE_NOTIFY = 1 << 0;
    }

    impl SetupMappingFlags: u64 {
        const FUSE_SETUPMAPPING_FLAG_WRITE = 1 << 0;
        const FUSE_SETUPMAPPING_FLAG_READ = 1 << 0;
    }
}
