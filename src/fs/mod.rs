//! # Filesystem
//!
//! Abstrações de alto nível para operações de arquivos e diretórios.
//!
//! ## Estrutura
//!
//! | Módulo | Descrição |
//! |--------|-----------|
//! | `types` | Tipos compartilhados (OpenFlags, Stat, DirEntry) |
//! | `file` | Abstração de arquivos (`File`, `BufReader`) |
//! | `dir` | Abstração de diretórios (`Dir`, `ReadDir`) |
//! | `path` | Utilitários de caminhos |
//! | `ops` | Operações de filesystem (stat, mkdir, etc) |
//!
//! ## Exemplo
//!
//! ```rust
//! use redpowder::fs::{File, Dir, OpenFlags};
//!
//! // Ler arquivo
//! let file = File::open("/apps/config.txt")?;
//! let content = file.read_all()?;
//!
//! // Listar diretório
//! for entry in Dir::open("/apps")?.entries() {
//!     println!("{}", entry.name());
//! }
//! ```

pub mod dir;
pub mod file;
pub mod ops;
pub mod path;
pub mod types;

// Re-exports principais
pub use dir::{list_dir, Dir, ReadDir};
pub use file::File;
pub use ops::{chdir, exists, getcwd, is_dir, is_file, stat};
pub use types::{
    DirEntry, FileStat, FileType, OpenFlags, SeekFrom, O_APPEND, O_CREATE, O_DIRECTORY, O_EXCL,
    O_RDONLY, O_RDWR, O_TRUNC, O_WRONLY,
};
