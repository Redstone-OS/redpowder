//! # Syscall Numbers
//!
//! Números de syscall do Redstone OS.
//! DEVEM corresponder exatamente ao kernel (forge/src/syscall/numbers.rs).

// === PROCESSO (0x01-0x0F) ===
pub const SYS_EXIT: usize = 0x01;
pub const SYS_SPAWN: usize = 0x02;
pub const SYS_WAIT: usize = 0x03;
pub const SYS_YIELD: usize = 0x04;
pub const SYS_GETPID: usize = 0x05;
pub const SYS_GETTASKINFO: usize = 0x06;

// === MEMÓRIA (0x10-0x1F) ===
pub const SYS_ALLOC: usize = 0x10;
pub const SYS_FREE: usize = 0x11;
pub const SYS_MAP: usize = 0x12;
pub const SYS_UNMAP: usize = 0x13;

// === HANDLES (0x20-0x2F) ===
pub const SYS_HANDLE_DUP: usize = 0x20;
pub const SYS_HANDLE_CLOSE: usize = 0x21;
pub const SYS_CHECK_RIGHTS: usize = 0x22;

// === IPC (0x30-0x3F) ===
pub const SYS_CREATE_PORT: usize = 0x30;
pub const SYS_SEND_MSG: usize = 0x31;
pub const SYS_RECV_MSG: usize = 0x32;

// === GRÁFICOS / INPUT (0x40-0x4F) ===
pub const SYS_FB_INFO: usize = 0x40;
pub const SYS_FB_WRITE: usize = 0x41;
pub const SYS_FB_CLEAR: usize = 0x42;
pub const SYS_MOUSE_READ: usize = 0x48;
pub const SYS_KEYBOARD_READ: usize = 0x49;

// === TEMPO (0x50-0x5F) ===
pub const SYS_CLOCK_GET: usize = 0x50;
pub const SYS_SLEEP: usize = 0x51;

// === FILESYSTEM (0x60-0x6F) ===
pub const SYS_OPEN: usize = 0x60;
pub const SYS_CLOSE: usize = 0x61;
pub const SYS_READ: usize = 0x62;
pub const SYS_WRITE: usize = 0x63;
pub const SYS_STAT: usize = 0x64;
pub const SYS_FSTAT: usize = 0x65;
pub const SYS_LSEEK: usize = 0x66;

// === EVENTS (0x80-0x8F) ===
pub const SYS_POLL: usize = 0x80;

// === SISTEMA (0xF0-0xFF) ===
pub const SYS_SYSINFO: usize = 0xF0;
pub const SYS_REBOOT: usize = 0xF1;
pub const SYS_POWEROFF: usize = 0xF2;
pub const SYS_CONSOLE_WRITE: usize = 0xF3;
pub const SYS_CONSOLE_READ: usize = 0xF4;
pub const SYS_DEBUG: usize = 0xFF;
