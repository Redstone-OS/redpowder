//! # Filesystem
//!
//! Operações de arquivos.

use crate::io::Handle;
use crate::syscall::{check_error, syscall1, syscall2, syscall3, SysResult};
use crate::syscall::{SYS_CLOSE, SYS_FSTAT, SYS_LSEEK, SYS_OPEN, SYS_READ, SYS_STAT, SYS_WRITE};

/// Flags de abertura
pub mod flags {
    pub const O_RDONLY: u32 = 0;
    pub const O_WRONLY: u32 = 1;
    pub const O_RDWR: u32 = 2;
    pub const O_CREATE: u32 = 1 << 6;
    pub const O_TRUNC: u32 = 1 << 9;
    pub const O_APPEND: u32 = 1 << 10;
}

/// Whence para seek
#[repr(u32)]
pub enum SeekFrom {
    Start = 0,
    Current = 1,
    End = 2,
}

/// Estrutura de arquivo
pub struct File {
    handle: Handle,
}

impl File {
    /// Abre um arquivo
    pub fn open(path: &str, flags: u32) -> SysResult<Self> {
        let ret = syscall3(SYS_OPEN, path.as_ptr() as usize, path.len(), flags as usize);
        let handle = Handle::from_raw(check_error(ret)? as u32);
        Ok(Self { handle })
    }

    /// Cria um novo arquivo
    pub fn create(path: &str) -> SysResult<Self> {
        Self::open(path, flags::O_WRONLY | flags::O_CREATE | flags::O_TRUNC)
    }

    /// Lê dados do arquivo
    pub fn read(&self, buf: &mut [u8]) -> SysResult<usize> {
        let ret = syscall3(
            SYS_READ,
            self.handle.raw() as usize,
            buf.as_mut_ptr() as usize,
            buf.len(),
        );
        check_error(ret)
    }

    /// Escreve dados no arquivo
    pub fn write(&self, buf: &[u8]) -> SysResult<usize> {
        let ret = syscall3(
            SYS_WRITE,
            self.handle.raw() as usize,
            buf.as_ptr() as usize,
            buf.len(),
        );
        check_error(ret)
    }

    /// Move posição de leitura/escrita
    pub fn seek(&self, offset: i64, whence: SeekFrom) -> SysResult<u64> {
        let ret = syscall3(
            SYS_LSEEK,
            self.handle.raw() as usize,
            offset as usize,
            whence as usize,
        );
        check_error(ret).map(|v| v as u64)
    }

    /// Obtém handle interno
    pub fn handle(&self) -> &Handle {
        &self.handle
    }
}

impl Drop for File {
    fn drop(&mut self) {
        let _ = syscall1(SYS_CLOSE, self.handle.raw() as usize);
    }
}

/// Informações de arquivo
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct Stat {
    pub mode: u32,
    pub size: u64,
    pub ctime_sec: u64,
    pub ctime_nsec: u32,
    pub mtime_sec: u64,
    pub mtime_nsec: u32,
    pub atime_sec: u64,
    pub atime_nsec: u32,
    pub nlink: u32,
    pub dev: u32,
    pub ino: u64,
}

/// Obtém stat de arquivo por caminho
pub fn stat(path: &str) -> SysResult<Stat> {
    let mut st = Stat::default();
    let ret = syscall3(
        SYS_STAT,
        path.as_ptr() as usize,
        path.len(),
        &mut st as *mut Stat as usize,
    );
    check_error(ret)?;
    Ok(st)
}

/// Obtém stat de arquivo por handle
pub fn fstat(file: &File) -> SysResult<Stat> {
    let mut st = Stat::default();
    let ret = syscall2(
        SYS_FSTAT,
        file.handle.raw() as usize,
        &mut st as *mut Stat as usize,
    );
    check_error(ret)?;
    Ok(st)
}
