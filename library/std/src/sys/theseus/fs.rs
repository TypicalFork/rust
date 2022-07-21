use crate::ffi::OsString;
use crate::fmt;
use crate::hash::{Hash, Hasher};
use crate::io::{self, IoSlice, IoSliceMut, ReadBuf, SeekFrom};
use crate::path::{Path, PathBuf};
use crate::sys::time::SystemTime;
use crate::sys::unsupported;

/// An open file or directory
#[derive(Debug)]
pub struct File;

#[derive(Clone, Copy, Debug)]
pub struct FileAttr {
    size: u64,
    /// `true` if file, `false` if directory
    is_file: bool,
    symlink: bool,
}

#[derive(Debug)]
pub struct ReadDir;

pub struct DirEntry;

// Includes only the system-generic flags for now.
#[derive(Clone, Copy, Debug)]
pub struct OpenOptions {
    read: bool,
    write: bool,
    append: bool,
    truncate: bool,
    create: bool,
    create_new: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FilePermissions;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FileType {
    ty: FileTypeInner,
    symlink: bool,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum FileTypeInner {
    File,
    Dir,
}

#[derive(Debug)]
pub struct DirBuilder;

impl FileAttr {
    pub fn size(&self) -> u64 {
        self.size
    }

    pub fn perm(&self) -> FilePermissions {
        FilePermissions
    }

    pub fn file_type(&self) -> FileType {
        FileType {
            ty: match self.is_file {
                true => FileTypeInner::File,
                false => FileType::Inner::Dir
            }
            symlink: self.symlink,
        }
    }

    pub fn modified(&self) -> io::Result<SystemTime> {
        self.0
    }

    pub fn accessed(&self) -> io::Result<SystemTime> {
        self.0
    }

    pub fn created(&self) -> io::Result<SystemTime> {
        self.0
    }
}

impl FilePermissions {
    pub fn readonly(&self) -> bool {
        // TODO: Theseus currently does not have the concept of permissions.
        false
    }

    pub fn set_readonly(&mut self, _readonly: bool) {
        // TODO: Theseus currently does not have the concept of permissions.
    }
}

impl FileType {
    pub fn is_dir(&self) -> bool {
        self.0
    }

    pub fn is_file(&self) -> bool {
        self.0
    }

    pub fn is_symlink(&self) -> bool {
        self.0
    }
}

impl Iterator for ReadDir {
    type Item = io::Result<DirEntry>;

    fn next(&mut self) -> Option<io::Result<DirEntry>> {
        None
    }
}

impl DirEntry {
    pub fn path(&self) -> PathBuf {
        unimplemented!();
    }

    pub fn file_name(&self) -> OsString {
        unimplemented!();
    }

    pub fn metadata(&self) -> io::Result<FileAttr> {
        unimplemented!();
    }

    pub fn file_type(&self) -> io::Result<FileType> {
        unimplemented!();
    }
}

impl OpenOptions {
    pub fn new() -> Self {
        OpenOptions {
            read: false,
            write: false,
            append: false,
            truncate: false,
            create: false,
            create_new: false,        }
    }

    pub fn read(&mut self, read: bool) {
        self.read = read;        
    }

    pub fn write(&mut self, write: bool) {
        self.write = write;
    }

    pub fn append(&mut self, append: bool) {
        self.append = append;
    }

    pub fn truncate(&mut self, truncate: bool) {
        self.truncate = truncate;
    }

    pub fn create(&mut self, create: bool) {
        self.create = create;
    }

    pub fn create_new(&mut self, create_new: bool) {
        self.create_new = create_new;
    }
}

impl File {
    pub fn open(_path: &Path, _opts: &OpenOptions) -> io::Result<Self> {
        unsupported()
    }

    pub fn file_attr(&self) -> io::Result<FileAttr> {
        self.0
    }

    pub fn fsync(&self) -> io::Result<()> {
        self.0
    }

    pub fn datasync(&self) -> io::Result<()> {
        self.0
    }

    pub fn truncate(&self, _size: u64) -> io::Result<()> {
        self.0
    }

    pub fn read(&self, _buf: &mut [u8]) -> io::Result<usize> {
        self.0
    }

    pub fn read_vectored(&self, _bufs: &mut [IoSliceMut<'_>]) -> io::Result<usize> {
        self.0
    }

    pub fn is_read_vectored(&self) -> bool {
        self.0
    }

    pub fn read_buf(&self, _buf: &mut ReadBuf<'_>) -> io::Result<()> {
        self.0
    }

    pub fn write(&self, _buf: &[u8]) -> io::Result<usize> {
        self.0
    }

    pub fn write_vectored(&self, _bufs: &[IoSlice<'_>]) -> io::Result<usize> {
        self.0
    }

    pub fn is_write_vectored(&self) -> bool {
        self.0
    }

    pub fn flush(&self) -> io::Result<()> {
        self.0
    }

    pub fn seek(&self, _pos: SeekFrom) -> io::Result<u64> {
        self.0
    }

    pub fn duplicate(&self) -> io::Result<File> {
        self.0
    }

    pub fn set_permissions(&self, _perm: FilePermissions) -> io::Result<()> {
        self.0
    }
}

impl DirBuilder {
    pub fn new() -> DirBuilder {
        DirBuilder {}
    }

    pub fn mkdir(&self, _p: &Path) -> io::Result<()> {
        unsupported()
    }
}

impl fmt::Debug for File {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0
    }
}

pub fn readdir(_p: &Path) -> io::Result<ReadDir> {
    unsupported()
}

pub fn unlink(_p: &Path) -> io::Result<()> {
    unsupported()
}

pub fn rename(_old: &Path, _new: &Path) -> io::Result<()> {
    unsupported()
}

pub fn set_perm(_p: &Path, perm: FilePermissions) -> io::Result<()> {
    match perm.0 {}
}

pub fn rmdir(_p: &Path) -> io::Result<()> {
    unsupported()
}

pub fn remove_dir_all(_path: &Path) -> io::Result<()> {
    unsupported()
}

pub fn try_exists(_path: &Path) -> io::Result<bool> {
    unsupported()
}

pub fn readlink(_p: &Path) -> io::Result<PathBuf> {
    unsupported()
}

pub fn symlink(_original: &Path, _link: &Path) -> io::Result<()> {
    unsupported()
}

pub fn link(_src: &Path, _dst: &Path) -> io::Result<()> {
    unsupported()
}

pub fn stat(_p: &Path) -> io::Result<FileAttr> {
    unsupported()
}

pub fn lstat(_p: &Path) -> io::Result<FileAttr> {
    unsupported()
}

pub fn canonicalize(_p: &Path) -> io::Result<PathBuf> {
    unsupported()
}

pub fn copy(_from: &Path, _to: &Path) -> io::Result<u64> {
    unsupported()
}
