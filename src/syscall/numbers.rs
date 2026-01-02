//! # Syscall Numbers
//!
//! Números de syscall do Redstone OS.
//!
//! ⚠️ DEVEM corresponder exatamente ao kernel (forge/src/syscall/numbers.rs).
//!
//! ## Organização
//!
//! | Range     | Categoria        |
//! |-----------|------------------|
//! | 0x01-0x0F | Processo         |
//! | 0x10-0x1F | Memória          |
//! | 0x20-0x2F | Handles          |
//! | 0x30-0x3F | IPC              |
//! | 0x40-0x4F | Gráficos/Input   |
//! | 0x50-0x5F | Tempo            |
//! | 0x60-0x7F | Filesystem       |
//! | 0x80-0x8F | Events           |
//! | 0xF0-0xFF | Sistema/Debug    |

// =============================================================================
// PROCESSO (0x01 - 0x0F)
// =============================================================================

pub const SYS_EXIT: usize = 0x01;
pub const SYS_SPAWN: usize = 0x02;
pub const SYS_WAIT: usize = 0x03;
pub const SYS_YIELD: usize = 0x04;
pub const SYS_GETPID: usize = 0x05;
pub const SYS_GETTASKINFO: usize = 0x06;
pub const SYS_GETTID: usize = 0x07;
pub const SYS_THREAD_CREATE: usize = 0x08;
pub const SYS_THREAD_EXIT: usize = 0x09;

// =============================================================================
// MEMÓRIA (0x10 - 0x1F)
// =============================================================================

pub const SYS_ALLOC: usize = 0x10;
pub const SYS_FREE: usize = 0x11;
pub const SYS_MAP: usize = 0x12;
pub const SYS_UNMAP: usize = 0x13;
pub const SYS_MPROTECT: usize = 0x14;

// =============================================================================
// HANDLES (0x20 - 0x2F)
// =============================================================================

pub const SYS_HANDLE_DUP: usize = 0x20;
pub const SYS_HANDLE_CLOSE: usize = 0x21;
pub const SYS_CHECK_RIGHTS: usize = 0x22;

// =============================================================================
// IPC (0x30 - 0x3F)
// =============================================================================

pub const SYS_CREATE_PORT: usize = 0x30;
pub const SYS_SEND_MSG: usize = 0x31;
pub const SYS_RECV_MSG: usize = 0x32;
pub const SYS_FUTEX_WAIT: usize = 0x33;
pub const SYS_FUTEX_WAKE: usize = 0x34;
pub const SYS_SHM_CREATE: usize = 0x35;
pub const SYS_SHM_MAP: usize = 0x36;
pub const SYS_PORT_CONNECT: usize = 0x37;
pub const SYS_SHM_GET_SIZE: usize = 0x38;

// =============================================================================
// GRÁFICOS / INPUT (0x40 - 0x4F)
// =============================================================================

pub const SYS_FB_INFO: usize = 0x40;
pub const SYS_FB_WRITE: usize = 0x41;
pub const SYS_FB_CLEAR: usize = 0x42;
pub const SYS_MOUSE_READ: usize = 0x48;
pub const SYS_KEYBOARD_READ: usize = 0x49;

// =============================================================================
// TEMPO (0x50 - 0x5F)
// =============================================================================

pub const SYS_CLOCK_GET: usize = 0x50;
pub const SYS_SLEEP: usize = 0x51;
pub const SYS_TIMER_CREATE: usize = 0x52;
pub const SYS_TIMER_SET: usize = 0x53;

// =============================================================================
// FILESYSTEM - BÁSICO (0x60 - 0x67)
// =============================================================================

/// Abre um arquivo ou diretório.
pub const SYS_OPEN: usize = 0x60;

/// Lê dados de um arquivo.
pub const SYS_READ: usize = 0x61;

/// Escreve dados em um arquivo.
pub const SYS_WRITE: usize = 0x62;

/// Move posição de leitura/escrita (cursor).
pub const SYS_SEEK: usize = 0x63;

/// Lê dados em offset específico (atômico).
pub const SYS_PREAD: usize = 0x64;

/// Escreve dados em offset específico (atômico).
pub const SYS_PWRITE: usize = 0x65;

/// Força flush de buffers.
pub const SYS_FLUSH: usize = 0x66;

/// Redimensiona um arquivo.
pub const SYS_TRUNCATE: usize = 0x67;

// =============================================================================
// FILESYSTEM - METADADOS (0x68 - 0x6B)
// =============================================================================

/// Obtém informações de arquivo por caminho.
pub const SYS_STAT: usize = 0x68;

/// Obtém informações de arquivo por handle.
pub const SYS_FSTAT: usize = 0x69;

/// Altera permissões de arquivo.
pub const SYS_CHMOD: usize = 0x6A;

/// Altera dono/grupo de arquivo.
pub const SYS_CHOWN: usize = 0x6B;

// =============================================================================
// FILESYSTEM - DIRETÓRIOS (0x6C - 0x6F)
// =============================================================================

/// Lista entradas de diretório (batch).
pub const SYS_GETDENTS: usize = 0x6C;

/// Cria um diretório.
pub const SYS_MKDIR: usize = 0x6D;

/// Remove um diretório vazio.
pub const SYS_RMDIR: usize = 0x6E;

/// Obtém diretório de trabalho atual.
pub const SYS_GETCWD: usize = 0x6F;

// =============================================================================
// FILESYSTEM - MANIPULAÇÃO (0x70 - 0x73)
// =============================================================================

/// Cria um arquivo vazio.
pub const SYS_CREATE: usize = 0x70;

/// Remove um arquivo.
pub const SYS_UNLINK: usize = 0x71;

/// Renomeia/move arquivo.
pub const SYS_RENAME: usize = 0x72;

/// Cria um hard link.
pub const SYS_LINK: usize = 0x73;

// =============================================================================
// FILESYSTEM - SYMLINKS (0x74 - 0x76)
// =============================================================================

/// Cria um link simbólico.
pub const SYS_SYMLINK: usize = 0x74;

/// Lê destino de link simbólico.
pub const SYS_READLINK: usize = 0x75;

/// Resolve caminho canônico.
pub const SYS_REALPATH: usize = 0x76;

// =============================================================================
// FILESYSTEM - MONTAGEM (0x77 - 0x7A)
// =============================================================================

/// Monta um filesystem.
pub const SYS_MOUNT: usize = 0x77;

/// Desmonta um filesystem.
pub const SYS_UMOUNT: usize = 0x78;

/// Obtém informações do filesystem.
pub const SYS_STATFS: usize = 0x79;

/// Sincroniza todos os buffers.
pub const SYS_SYNC: usize = 0x7A;

// =============================================================================
// FILESYSTEM - AVANÇADO (0x7B - 0x7F)
// =============================================================================

/// Controle de dispositivo.
pub const SYS_IOCTL: usize = 0x7B;

/// Controle de handle.
pub const SYS_FCNTL: usize = 0x7C;

/// Lock de arquivo.
pub const SYS_FLOCK: usize = 0x7D;

/// Verifica permissões de acesso.
pub const SYS_ACCESS: usize = 0x7E;

/// Altera diretório de trabalho.
pub const SYS_CHDIR: usize = 0x7F;

// =============================================================================
// EVENTS (0x80 - 0x8F)
// =============================================================================

pub const SYS_POLL: usize = 0x80;

// =============================================================================
// SISTEMA / DEBUG (0xF0 - 0xFF)
// =============================================================================

pub const SYS_SYSINFO: usize = 0xF0;
pub const SYS_REBOOT: usize = 0xF1;
pub const SYS_POWEROFF: usize = 0xF2;
pub const SYS_CONSOLE_WRITE: usize = 0xF3;
pub const SYS_CONSOLE_READ: usize = 0xF4;
pub const SYS_DEBUG: usize = 0xFF;
