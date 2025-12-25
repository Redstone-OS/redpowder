//! Números das Syscalls - ABI Redstone OS

// Processo (0x01-0x0F)
pub const SYS_EXIT: usize = 0x01;
pub const SYS_SPAWN: usize = 0x02;
pub const SYS_WAIT: usize = 0x03;
pub const SYS_YIELD: usize = 0x04;
pub const SYS_GETPID: usize = 0x05;
pub const SYS_GETTASKINFO: usize = 0x06;

// Memória (0x10-0x1F)
pub const SYS_ALLOC: usize = 0x10;
pub const SYS_FREE: usize = 0x11;
pub const SYS_MAP: usize = 0x12;
pub const SYS_UNMAP: usize = 0x13;

// Handles (0x20-0x2F)
pub const SYS_HANDLE_CREATE: usize = 0x20;
pub const SYS_HANDLE_DUP: usize = 0x21;
pub const SYS_HANDLE_CLOSE: usize = 0x22;
pub const SYS_CHECK_RIGHTS: usize = 0x23;

// IPC (0x30-0x3F)
pub const SYS_CREATE_PORT: usize = 0x30;
pub const SYS_SEND_MSG: usize = 0x31;
pub const SYS_RECV_MSG: usize = 0x32;
pub const SYS_PEEK_MSG: usize = 0x33;

// IO (0x40-0x4F)
pub const SYS_READV: usize = 0x40;
pub const SYS_WRITEV: usize = 0x41;

// Tempo (0x50-0x5F)
pub const SYS_CLOCK_GET: usize = 0x50;
pub const SYS_SLEEP: usize = 0x51;
pub const SYS_MONOTONIC: usize = 0x52;

// Async IO (0xE0-0xEF)
pub const SYS_CREATE_RING: usize = 0xE0;
pub const SYS_SUBMIT_IO: usize = 0xE1;
pub const SYS_WAIT_IO: usize = 0xE2;
pub const SYS_CLOSE_RING: usize = 0xE3;

// Sistema (0xF0-0xFF)
pub const SYS_SYSINFO: usize = 0xF0;
pub const SYS_DEBUG: usize = 0xFF;
