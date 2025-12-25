# Changelog

Todas as mudanças notáveis deste projeto serão documentadas aqui.

O formato é baseado em [Keep a Changelog](https://keepachangelog.com/pt-BR/1.0.0/).

## [0.1.0] - 2024-12-25

### Adicionado
- Módulo `syscall` com wrappers para syscalls do Redstone OS
  - `sys_exit`, `sys_yield`, `sys_getpid`, `sys_write`
- Módulo `io` com macros `print!` e `println!`
- Módulo `memory` com `alloc`, `free`, `alloc_rw`
- Módulo `ipc` com `create_port`, `send`, `recv`, `peek`
- Módulo `time` com `sleep`, `monotonic`, `uptime_ms`
- Módulo `prelude` para importação rápida
- Documentação completa em português

### Notas
- Primeira versão pública
- Numeração de syscalls própria do Redstone OS (não compatível com Linux/POSIX)
