use zerocopy::{KnownLayout, Immutable, FromBytes, TryFromBytes, IntoBytes};

use super::*;

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(KnownLayout, Immutable, TryFromBytes, IntoBytes)]
/// FUSE request opcode.
///
/// The opcode represents the kind of request that the kernel is asking the FUSE
/// server application to do.
///
/// This value is provided in the opcode field of the [`fuse_in_header`].
pub enum fuse_opcode {
    /// Lookup a directory entry given the filename, returning the inode number
    /// and its attributes. The parent directory of the entry is in 
    /// `fuse_in_header.nodeid`.
    ///
    /// Request format
    /// - `entry_name: CString`
    ///
    /// Reply format
    /// - [`fuse_entry_out`]
    FUSE_LOOKUP = 1,
    /// Forget an inode.
    ///
    /// Request format:
    /// - [`fuse_forget_in`]
    ///
    /// Reply format:
    /// - This operation doesn't require a reply
    FUSE_FORGET = 2,
    /// Get attributes for a filesystem object, the object is identifed by
    /// the nodeid field in [`fuse_in_header`] or by the file handle `fh` if 
    /// [`GetattrFlags::FUSE_GETATTR_FH`] is set.
    ///
    /// Request format:
    /// - [`fuse_getattr_in`]
    ///
    /// Reply format:
    /// - [`fuse_attr_out`]
    FUSE_GETATTR = 3,
    FUSE_SETATTR = 4,
    FUSE_READLINK = 5,
    FUSE_SYMLINK = 6,
    FUSE_MKNOD = 8,
    FUSE_MKDIR = 9,
    FUSE_UNLINK = 10,
    FUSE_RMDIR = 11,
    FUSE_RENAME = 12,
    FUSE_LINK = 13,
    FUSE_OPEN = 14,
    FUSE_READ = 15,
    FUSE_WRITE = 16,
    FUSE_STATFS = 17,
    FUSE_RELEASE = 18,
    FUSE_FSYNC = 20,
    FUSE_SETXATTR = 21,
    FUSE_GETXATTR = 22,
    FUSE_LISTXATTR = 23,
    FUSE_REMOVEXATTR = 24,
    FUSE_FLUSH = 25,
    FUSE_INIT = 26,
    FUSE_OPENDIR = 27,
    FUSE_READDIR = 28,
    FUSE_RELEASEDIR = 29,
    FUSE_FSYNCDIR = 30,
    FUSE_GETLK = 31,
    FUSE_SETLK = 32,
    FUSE_SETLKW = 33,
    FUSE_ACCESS = 34,
    FUSE_CREATE = 35,
    FUSE_INTERRUPT = 36,
    FUSE_BMAP = 37,
    FUSE_DESTROY = 38,
    FUSE_IOCTL = 39,
    FUSE_POLL = 40,
    FUSE_NOTIFY_REPLY = 41,
    FUSE_BATCH_FORGET = 42,
    FUSE_FALLOCATE = 43,
    FUSE_READDIRPLUS = 44,
    FUSE_RENAME2 = 45,
    FUSE_LSEEK = 46,
    FUSE_COPY_FILE_RANGE = 47,
    FUSE_SETUPMAPPING = 48,
    FUSE_REMOVEMAPPING = 49,
    FUSE_SYNCFS	= 50,
    FUSE_TMPFILE = 51,
    FUSE_STATX = 52,
    FUSE_COPY_FILE_RANGE_64	= 53,
}

#[repr(C)]
#[derive(Debug)]
#[derive(KnownLayout, Immutable, TryFromBytes, IntoBytes)]
/// Header for all FUSE requests.
///
/// This header can be followed by some extra parameters in the form of
/// structures (named `fuse_*_in`) and then possibly followed by a section for
/// extra fields with variable length (ex. strings).
///
/// The format of the request depends on the specific opcode, the format of each
/// request is documented in the variants of [`fuse_opcode`].
///
/// Here is an example structure of a request (FUSE_MKDIR)
/// ```text
/// ┌────────────────┬───────────────┬──╌╌╌╌╌╌──┐
/// │ fuse_in_header │ fuse_mkdir_in │ dir_name │
/// └────────────────┴───────────────┴──╌╌╌╌╌╌──┘
///  └─┬────────────┘ └─┬───────────┘ └─┬──────┘
///    │                │               │ 
///    │                │               └─ Variable length fields
///    │                └─ Extra fields related to the request
///    └─ The request header
/// ```
pub struct fuse_in_header {
    pub len: u32,
    pub opcode: fuse_opcode,
    pub unique: u64,
    pub nodeid: u64,
    pub uid: u32,
    pub gid: u32,
    pub pid: u32,
    pub total_extlen: u16,
    pub padding: u16,
}

#[repr(C)]
#[derive(Debug, Clone)]
#[derive(KnownLayout, Immutable, IntoBytes)]
pub struct fuse_out_header {
    pub len: u32,
    pub error: i32,
    pub unique: u64,
}

// FUSE_LOOKUP

#[repr(C)]
#[derive(Debug, Clone)]
#[derive(KnownLayout, Immutable, IntoBytes)]
pub struct fuse_entry_out {
    pub nodeid: u64,
    pub generation: u64,
    pub entry_valid: seconds,
    pub attr_valid: seconds,
    pub entry_valid_nsec: nanos,
    pub attr_valid_nsec: nanos,
    pub attr: fuse_attr,
}

// FUSE_FORGET

#[repr(C)]
#[derive(Debug, Clone)]
#[derive(KnownLayout, Immutable, FromBytes, IntoBytes)]
pub struct fuse_forget_in {
    pub nlookup: u64,
}

// FUSE_GETATTR

#[repr(C)]
#[derive(Debug, Clone)]
#[derive(KnownLayout, Immutable, FromBytes, IntoBytes)]
pub struct fuse_getattr_in {
    pub getattr_flags: GetattrFlags,
    pub dummy: u32,
    pub fh: u64,
}

#[repr(C)]
#[derive(Debug, Clone)]
#[derive(KnownLayout, Immutable, IntoBytes)]
pub struct fuse_attr_out {
    pub attr_valid: seconds,
    pub attr_valid_nsec: nanos,
    pub dummy: Padding<u32>,
    pub attr: fuse_attr,
}

// FUSE_SETATTR

#[repr(C)]
#[derive(Debug, Clone)]
#[derive(KnownLayout, Immutable, FromBytes, IntoBytes)]
pub struct fuse_setattr_in {
    pub valid: SetattrValid,
    pub padding: u32,
    pub fh: u64,
    pub size: u64,
    pub lock_owner: u64,
    pub atime: seconds,
    pub mtime: seconds,
    pub ctime: seconds,
    pub atimensec: nanos,
    pub mtimensec: nanos,
    pub ctimensec: nanos,
    pub mode: u32,
    pub unused4: u32,
    pub uid: u32,
    pub gid: u32,
    pub unused5: u32,
}

// FUSE_READLINK

// FUSE_SYMLINK

// FUSE_MKNOD

#[repr(C)]
#[derive(Debug, Clone)]
#[derive(KnownLayout, Immutable, FromBytes, IntoBytes)]
pub struct fuse_mknod_in {
    pub mode: u32,
    pub rdev: u32,
    pub umask: u32,
    pub padding: u32,
}

// FUSE_MKDIR

#[repr(C)]
#[derive(Debug, Clone)]
#[derive(KnownLayout, Immutable, FromBytes, IntoBytes)]
pub struct fuse_mkdir_in {
    pub mode: u32,
    pub umask: u32,
}

// FUSE_UNLINK

// FUSE_RMDIR

// FUSE_RENAME

#[repr(C)]
#[derive(Debug, Clone)]
#[derive(KnownLayout, Immutable, FromBytes, IntoBytes)]
pub struct fuse_rename_in {
    pub newdir: u64
}

// FUSE_LINK

#[repr(C)]
#[derive(Debug, Clone)]
#[derive(KnownLayout, Immutable, FromBytes, IntoBytes)]
pub struct fuse_link_in {
    pub oldnodeid: u64
}

// FUSE_OPEN

#[repr(C)]
#[derive(Debug, Clone)]
#[derive(KnownLayout, Immutable, FromBytes, IntoBytes)]
pub struct fuse_open_in {
    // TODO: these are the flags coming from the open(2) syscall
    pub flags: u32,
    pub open_flags: OpenInFlags,
}

#[repr(C)]
#[derive(Debug, Clone)]
#[derive(KnownLayout, Immutable, IntoBytes)]
pub struct fuse_open_out {
    pub fh: u64,
    pub open_flags: OpenOutFlags,
    pub backing_id: i32,
}

// FUSE_READ

#[repr(C)]
#[derive(Debug, Clone)]
#[derive(KnownLayout, Immutable, FromBytes, IntoBytes)]
pub struct fuse_read_in {
    pub fh: u64,
    pub offset: u64,
    pub size: u32,
    pub read_flags: ReadFlags,
    pub lock_owner: u64,
    // File open flags
    pub flags: u32,
    pub padding: u32,
}

// FUSE_WRITE

#[repr(C)]
#[derive(Debug, Clone)]
#[derive(KnownLayout, Immutable, FromBytes, IntoBytes)]
pub struct fuse_write_in {
    pub fh: u64,
    pub offset: u64,
    pub size: u32,
    pub write_flags: WriteFlags,
    pub lock_owner: u64,
    pub flags: u32,
    pub padding: u32,
}

#[repr(C)]
#[derive(Debug, Clone)]
#[derive(KnownLayout, Immutable, IntoBytes)]
pub struct fuse_write_out {
    pub size: u32,
    pub padding: Padding<u32>,
}

// FUSE_STATFS

#[repr(C)]
#[derive(Debug, Clone)]
#[derive(KnownLayout, Immutable, IntoBytes)]
pub struct fuse_statfs_out {
    pub st: fuse_kstatfs,
}

// FUSE_RELEASE

#[repr(C)]
#[derive(Debug, Clone)]
#[derive(KnownLayout, Immutable, FromBytes, IntoBytes)]
pub struct fuse_release_in {
    pub fh: u64,
    // File open flags
    pub flags: u32,
    pub release_flags: u32,
    pub lock_owner: u64,
}

// FUSE_FSYNC

#[repr(C)]
#[derive(Debug, Clone)]
#[derive(KnownLayout, Immutable, FromBytes, IntoBytes)]
pub struct fuse_fsync_in {
    pub fh: u64,
    pub fsync_flags: FsyncFlags,
    pub padding: u32,
}

// FUSE_SETXATTR

#[repr(C)]
#[derive(Debug, Clone)]
#[derive(KnownLayout, Immutable, FromBytes, IntoBytes)]
pub struct fuse_setxattr_in {
    pub size: u32,
    // File open flags?
    pub flags: u32,
    pub setxattr_flags: u32,
    pub padding: u32,
}

// FUSE_GETXATTR

#[repr(C)]
#[derive(Debug, Clone)]
#[derive(KnownLayout, Immutable, FromBytes, IntoBytes)]
pub struct fuse_getxattr_in {
    pub size: u32,
    pub padding: u32,
}

#[repr(C)]
#[derive(Debug, Clone)]
#[derive(KnownLayout, Immutable, IntoBytes)]
pub struct fuse_getxattr_out {
    pub size: u32,
    pub padding: Padding<u32>,
}

// FUSE_LISTXATTR
// uses fuse_getxattr_in

// FUSE_REMOVEXATTR

// FUSE_FLUSH

#[repr(C)]
#[derive(Debug, Clone)]
#[derive(KnownLayout, Immutable, FromBytes, IntoBytes)]
pub struct fuse_flush_in {
    pub fh: u64,
    pub unused: u32,
    pub padding: u32,
    pub lock_owner: u64,
}

// FUSE_INIT

#[repr(C)]
#[derive(Debug, Clone)]
#[derive(KnownLayout, Immutable, FromBytes, IntoBytes)]
pub struct fuse_init_in {
    pub major: u32,
    pub minor: u32,
    pub max_readahead: u32,
    pub flags: u32,
    pub flags2: u32,
    pub unused: [u32; 11],
}

#[repr(C)]
#[derive(Debug, Clone)]
#[derive(KnownLayout, Immutable, IntoBytes)]
pub struct fuse_init_out {
    pub major: u32,
    pub minor: u32,
    pub max_readahead: u32,
    pub flags: u32,
    pub max_background: u16,
    pub congestion_threshold: u16,
    pub max_write: u32,
    pub time_gran: u32,
    pub max_pages: u16,
    pub max_alignment: u16,
    pub flags2: u32,
    pub max_stack_depth: u32,
    pub request_timeout: u16,
    pub unused: Padding<[u16; 11]>,
}

// FUSE_OPENDIR
// uses fuse_open_in

// FUSE_READDIR
// uses fuse_read_in

// FUSE_RELEASEDIR
// uses fuse_release_in

// FUSE_FSYNCDIR
// uses fuse_fsync_in

// FUSE_GETLK

#[repr(C)]
#[derive(Debug, Clone)]
#[derive(KnownLayout, Immutable, FromBytes, IntoBytes)]
pub struct fuse_lk_in {
    pub fh: u64,
    pub owner: u64,
    pub lk: fuse_file_lock,
    pub lk_flags: LockFlags,
    pub padding: u32,
}

#[repr(C)]
#[derive(Debug, Clone)]
#[derive(KnownLayout, Immutable, IntoBytes)]
pub struct fuse_lk_out {
    pub lk: fuse_file_lock
}

// FUSE_SETLK
// uses fuse_lk_in

// FUSE_SETLKW
// uses fuse_lk_in

// FUSE_ACCESS

#[repr(C)]
#[derive(Debug, Clone)]
#[derive(KnownLayout, Immutable, FromBytes, IntoBytes)]
pub struct fuse_access_in {
    pub mask: u32,
    pub padding: u32,
}

// FUSE_CREATE

#[repr(C)]
#[derive(Debug, Clone)]
#[derive(KnownLayout, Immutable, FromBytes, IntoBytes)]
pub struct fuse_create_in {
    // File open flags
    pub flags: u32,
    pub mode: u32,
    pub umask: u32,
    pub open_flags: OpenInFlags,
}

// FUSE_INTERRUPT

#[repr(C)]
#[derive(Debug, Clone)]
#[derive(KnownLayout, Immutable, FromBytes, IntoBytes)]
pub struct fuse_interrupt_in {
    pub unique: u64,
}

// FUSE_BMAP

#[repr(C)]
#[derive(Debug, Clone)]
#[derive(KnownLayout, Immutable, FromBytes, IntoBytes)]
pub struct fuse_bmap_in {
    pub block: u64,
    pub blocksize: u32,
    pub padding: u32,
}

#[repr(C)]
#[derive(Debug, Clone)]
#[derive(KnownLayout, Immutable, IntoBytes)]
pub struct fuse_bmap_out {
    pub block: u64,
}

// FUSE_DESTROY

// FUSE_IOCTL

#[repr(C)]
#[derive(Debug, Clone)]
#[derive(KnownLayout, Immutable, FromBytes, IntoBytes)]
pub struct fuse_ioctl_in {
    pub fh: u64,
    pub flags: IoctlFlags,
    pub cmd: u32,
    pub arg: u64,
    pub in_size: u32,
    pub out_size: u32,
}

#[repr(C)]
#[derive(Debug, Clone)]
#[derive(KnownLayout, Immutable, IntoBytes)]
pub struct fuse_ioctl_out {
    pub result: i32,
    pub flags: IoctlFlags,
    pub in_iovs: u32,
    pub out_iovs: u32,
}

#[repr(C)]
#[derive(Debug, Clone)]
#[derive(KnownLayout, Immutable, FromBytes, IntoBytes)]
pub struct fuse_ioctl_iovec {
    pub base: u64,
    pub len: u64,
}

// FUSE_POLL

#[repr(C)]
#[derive(Debug, Clone)]
#[derive(KnownLayout, Immutable, FromBytes, IntoBytes)]
pub struct fuse_poll_in {
    pub fh: u64,
    pub kh: u64,
    pub flags: u32,
    pub events: u32,
}

#[repr(C)]
#[derive(Debug, Clone)]
#[derive(KnownLayout, Immutable, IntoBytes)]
pub struct fuse_poll_out {
    pub revents: u32,
    pub padding: Padding<u32>,
}

// FUSE_NOTIFY_REPLY
// TODO: support notifications

// FUSE_BATCH_FORGET

#[repr(C)]
#[derive(Debug, Clone)]
#[derive(KnownLayout, Immutable, FromBytes, IntoBytes)]
pub struct fuse_batch_forget_in {
    pub count: u32,
    pub dummy: u32,
}

#[repr(C)]
#[derive(Debug, Clone)]
#[derive(KnownLayout, Immutable, FromBytes, IntoBytes)]
pub struct fuse_forget_one {
    pub nodeid: u64,
    pub nlookup: u64,
}

// FUSE_FALLOCATE

#[repr(C)]
#[derive(Debug, Clone)]
#[derive(KnownLayout, Immutable, FromBytes, IntoBytes)]
pub struct fuse_fallocate_in {
    pub fh: u64,
    pub offset: u64,
    pub length: u64,
    pub mode: u32,
    pub padding: u32,
}

// FUSE_READDIRPLUS
// uses fuse_read_in

// FUSE_RENAME2

#[repr(C)]
#[derive(Debug, Clone)]
#[derive(KnownLayout, Immutable, FromBytes, IntoBytes)]
pub struct fuse_rename2_in {
    pub newdir: u64,
    pub flags: u32,
    pub padding: u32,
}

// FUSE_LSEEK

#[repr(C)]
#[derive(Debug, Clone)]
#[derive(KnownLayout, Immutable, FromBytes, IntoBytes)]
pub struct fuse_lseek_in {
    pub fh: u64,
    pub offset: u64,
    pub whence: u32,
    pub padding: u32,
}

#[repr(C)]
#[derive(Debug, Clone)]
#[derive(KnownLayout, Immutable, IntoBytes)]
pub struct fuse_lseek_out {
    pub offset: u64,
}

// FUSE_COPY_FILE_RANGE

#[repr(C)]
#[derive(Debug, Clone)]
#[derive(KnownLayout, Immutable, FromBytes, IntoBytes)]
pub struct fuse_copy_file_range_in {
    pub fh_in: u64,
    pub off_in: u64,
    pub nodeid_out: u64,
    pub fh_out: u64,
    pub off_out: u64,
    pub len: u64,
    pub flags: u64,
}

// FUSE_SETUPMAPPING

#[repr(C)]
#[derive(Debug, Clone)]
#[derive(KnownLayout, Immutable, FromBytes, IntoBytes)]
pub struct fuse_setupmapping_in {
    pub fh: u64,
    pub foffset: u64,
    pub len: u64,
    pub flags: u64,
    pub moffset: u64,
}

// FUSE_REMOVEMAPPING

#[repr(C)]
#[derive(Debug, Clone)]
#[derive(KnownLayout, Immutable, FromBytes, IntoBytes)]
pub struct fuse_removemapping_in {
    pub count: u32,
}

#[repr(C)]
#[derive(Debug, Clone)]
#[derive(KnownLayout, Immutable, FromBytes, IntoBytes)]
pub struct fuse_removemapping_one {
    pub moffset: u64,
    pub len: u64,
}

// FUSE_SYNCFS

#[repr(C)]
#[derive(Debug, Clone)]
#[derive(KnownLayout, Immutable, FromBytes, IntoBytes)]
pub struct fuse_syncfs_in {
    pub padding: u64,
}

// FUSE_TMPFILE
// TODO

// FUSE_STATX

#[repr(C)]
#[derive(Debug, Clone)]
#[derive(KnownLayout, Immutable, FromBytes, IntoBytes)]
pub struct fuse_statx_in {
    pub getattr_flags: GetattrFlags,
    pub reserved: u32,
    pub fh: u64,
    pub sx_flags: u32,
    pub sx_mask: u32,
}

#[repr(C)]
#[derive(Debug, Clone)]
#[derive(KnownLayout, Immutable, IntoBytes)]
pub struct fuse_statx_out {
    pub attr_valid: seconds,
    pub attr_valid_nsec: nanos,
    pub flags: u32,
    pub spare: Padding<[u64; 2]>,
    pub stat: fuse_statx,
}

// FUSE_COPY_FILE_RANGE_64
