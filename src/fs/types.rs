//! # Filesystem Types
//!
//! Tipos compartilhados para operações de filesystem.

// =============================================================================
// OPEN FLAGS
// =============================================================================

/// Flags para abertura de arquivos
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct OpenFlags(pub u32);

impl OpenFlags {
    /// Cria flags com valor inicial
    pub const fn new(value: u32) -> Self {
        Self(value)
    }

    /// Combina com outra flag
    pub const fn with(self, other: u32) -> Self {
        Self(self.0 | other)
    }

    /// Verifica se pode ler
    pub fn can_read(&self) -> bool {
        (self.0 & 0x3) != O_WRONLY
    }

    /// Verifica se pode escrever
    pub fn can_write(&self) -> bool {
        (self.0 & 0x3) != O_RDONLY
    }

    /// Valor raw
    pub fn bits(&self) -> u32 {
        self.0
    }
}

impl core::ops::BitOr for OpenFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<u32> for OpenFlags {
    type Output = Self;
    fn bitor(self, rhs: u32) -> Self {
        Self(self.0 | rhs)
    }
}

// Constantes de flags
pub const O_RDONLY: u32 = 0;
pub const O_WRONLY: u32 = 1;
pub const O_RDWR: u32 = 2;
pub const O_CREATE: u32 = 0x0100;
pub const O_TRUNC: u32 = 0x0200;
pub const O_APPEND: u32 = 0x0400;
pub const O_EXCL: u32 = 0x0800;
pub const O_DIRECTORY: u32 = 0x1000;

// =============================================================================
// SEEK
// =============================================================================

/// Origem para operação de seek
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum SeekFrom {
    /// Do início do arquivo
    Start = 0,
    /// Da posição atual
    Current = 1,
    /// Do fim do arquivo
    End = 2,
}

// =============================================================================
// FILE TYPE
// =============================================================================

/// Tipo de arquivo
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum FileType {
    Unknown = 0,
    Regular = 1,
    Directory = 2,
    Symlink = 3,
    CharDevice = 4,
    BlockDevice = 5,
    Fifo = 6,
    Socket = 7,
}

impl FileType {
    /// Cria a partir de valor u8
    pub fn from_u8(value: u8) -> Self {
        match value {
            1 => Self::Regular,
            2 => Self::Directory,
            3 => Self::Symlink,
            4 => Self::CharDevice,
            5 => Self::BlockDevice,
            6 => Self::Fifo,
            7 => Self::Socket,
            _ => Self::Unknown,
        }
    }

    /// É arquivo regular?
    pub fn is_file(&self) -> bool {
        *self == Self::Regular
    }

    /// É diretório?
    pub fn is_dir(&self) -> bool {
        *self == Self::Directory
    }

    /// É symlink?
    pub fn is_symlink(&self) -> bool {
        *self == Self::Symlink
    }
}

// =============================================================================
// FILE STAT
// =============================================================================

/// Informações de arquivo (correspondente ao kernel)
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct FileStat {
    /// Tipo de arquivo
    pub file_type: u8,
    /// Permissões (mode)
    pub mode: u16,
    /// Padding
    pub _pad: u8,
    /// Tamanho em bytes
    pub size: u64,
    /// Número de hard links
    pub nlink: u32,
    /// UID do dono
    pub uid: u32,
    /// GID do grupo
    pub gid: u32,
    /// Padding
    pub _pad2: u32,
    /// Tempo de último acesso (ms desde epoch)
    pub atime: u64,
    /// Tempo de última modificação (ms desde epoch)
    pub mtime: u64,
    /// Tempo de criação (ms desde epoch)
    pub ctime: u64,
}

impl FileStat {
    /// Tamanho da estrutura
    pub const SIZE: usize = core::mem::size_of::<Self>();

    /// Cria estrutura zerada
    pub const fn zeroed() -> Self {
        Self {
            file_type: 0,
            mode: 0,
            _pad: 0,
            size: 0,
            nlink: 0,
            uid: 0,
            gid: 0,
            _pad2: 0,
            atime: 0,
            mtime: 0,
            ctime: 0,
        }
    }

    /// Tipo de arquivo
    pub fn file_type(&self) -> FileType {
        FileType::from_u8(self.file_type)
    }

    /// É arquivo regular?
    pub fn is_file(&self) -> bool {
        self.file_type().is_file()
    }

    /// É diretório?
    pub fn is_dir(&self) -> bool {
        self.file_type().is_dir()
    }
}

impl Default for FileStat {
    fn default() -> Self {
        Self::zeroed()
    }
}

// =============================================================================
// DIRECTORY ENTRY
// =============================================================================

/// Entrada de diretório
///
/// Retornada por `Dir::read()` ou `ReadDir` iterator.
#[derive(Debug, Clone)]
pub struct DirEntry {
    /// Nome do arquivo/diretório
    name: [u8; 256],
    name_len: usize,
    /// Tipo de arquivo
    file_type: FileType,
    /// Número do inode (pode ser 0)
    ino: u64,
}

impl DirEntry {
    /// Cria DirEntry vazia
    pub const fn empty() -> Self {
        Self {
            name: [0u8; 256],
            name_len: 0,
            file_type: FileType::Unknown,
            ino: 0,
        }
    }

    /// Nome do arquivo
    pub fn name(&self) -> &str {
        core::str::from_utf8(&self.name[..self.name_len]).unwrap_or("")
    }

    /// Tipo de arquivo
    pub fn file_type(&self) -> FileType {
        self.file_type
    }

    /// É arquivo regular?
    pub fn is_file(&self) -> bool {
        self.file_type.is_file()
    }

    /// É diretório?
    pub fn is_dir(&self) -> bool {
        self.file_type.is_dir()
    }

    /// Número do inode
    pub fn ino(&self) -> u64 {
        self.ino
    }

    /// Parseia de buffer raw retornado por getdents
    ///
    /// Layout:
    /// - 0..8: ino (u64)
    /// - 8..10: rec_len (u16)
    /// - 10: file_type (u8)
    /// - 11: name_len (u8)
    /// - 12..: name bytes
    pub fn parse_from_buffer(buf: &[u8]) -> Option<(Self, usize)> {
        if buf.len() < 12 {
            return None;
        }

        let ino = u64::from_le_bytes([
            buf[0], buf[1], buf[2], buf[3], buf[4], buf[5], buf[6], buf[7],
        ]);
        let rec_len = u16::from_le_bytes([buf[8], buf[9]]) as usize;
        let file_type = FileType::from_u8(buf[10]);
        let name_len = buf[11] as usize;

        if rec_len < 12 || buf.len() < rec_len || name_len > 255 {
            return None;
        }

        let mut entry = Self::empty();
        entry.ino = ino;
        entry.file_type = file_type;
        entry.name_len = name_len;
        entry.name[..name_len].copy_from_slice(&buf[12..12 + name_len]);

        Some((entry, rec_len))
    }
}

// =============================================================================
// FILESYSTEM STAT
// =============================================================================

/// Informações do filesystem (para statfs)
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct FsStat {
    /// Tipo de filesystem
    pub fs_type: u32,
    /// Tamanho do bloco
    pub block_size: u32,
    /// Total de blocos
    pub total_blocks: u64,
    /// Blocos livres
    pub free_blocks: u64,
    /// Total de inodes
    pub total_inodes: u64,
    /// Inodes livres
    pub free_inodes: u64,
    /// Tamanho máximo de nome
    pub max_name_len: u32,
    /// Padding
    pub _pad: u32,
}
