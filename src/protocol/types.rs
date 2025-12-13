use zerocopy::{KnownLayout, Immutable, FromBytes, IntoBytes};

use super::*;

#[repr(C)]
#[derive(Debug, Clone)]
#[derive(KnownLayout, Immutable, FromBytes, IntoBytes)]
pub struct fuse_attr {
    pub ino: u64,
    pub size: u64,
    pub blocks: u64,
    pub atime: seconds,
    pub mtime: seconds,
    pub ctime: seconds,
    pub atimensec: nanos,
    pub mtimensec: nanos,
    pub ctimensec: nanos,
    pub mode: u32,
    pub nlink: u32,
    pub uid: u32,
    pub gid: u32,
    pub rdev: u32,
    pub blksize: u32,
    pub flags: AttrFlags,
}

#[repr(C)]
#[derive(Debug, Clone)]
#[derive(KnownLayout, Immutable, IntoBytes)]
pub struct fuse_sx_time {
    pub tv_sec: seconds,
    pub tv_nsec: nanos,
    pub __reserved: Padding<i32>,
}

#[repr(C)]
#[derive(Debug, Clone)]
#[derive(KnownLayout, Immutable, IntoBytes)]
pub struct fuse_statx {
    pub mask: u32,
    pub blksize: u32,
    pub attributes: u64,
    pub nlink: u32,
    pub uid: u32,
    pub gid: u32,
    pub mode: u16,
    pub __spare0: Padding<u16>,
    pub ino: u64,
    pub size: u64,
    pub blocks: u64,
    pub attributes_mask: u64,
    pub atime: fuse_sx_time,
    pub btime: fuse_sx_time,
    pub ctime: fuse_sx_time,
    pub mtime: fuse_sx_time,
    pub rdev_major: u32,
    pub rdev_minor: u32,
    pub dev_major: u32,
    pub dev_minor: u32,
    pub __spare2: Padding<[u64; 14]>,
}

#[repr(C)]
#[derive(Debug, Clone)]
#[derive(KnownLayout, Immutable, IntoBytes)]
pub struct fuse_kstatfs {
    pub blocks: u64,
    pub bfree: u64,
    pub bavail: u64,
    pub files: u64,
    pub ffree: u64,
    pub bsize: u32,
    pub namelen: u32,
    pub frsize: u32,
    pub padding: Padding<u32>,
    pub spare: Padding<[u32; 6]>,
}

#[repr(C)]
#[derive(Debug, Clone)]
#[derive(KnownLayout, Immutable, FromBytes, IntoBytes)]
pub struct fuse_file_lock {
    pub start: u64,
    pub end: u64,
    pub r#type: u32,
    pub pid: u32,
}
