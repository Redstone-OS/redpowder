# Redpowder SDK v0.3.0

**SDK para desenvolvimento userland no RedstoneOS**

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![no_std](https://img.shields.io/badge/no__std-compatible-green.svg)](https://docs.rust-embedded.org/book/intro/no-std.html)

---

## 📋 Índice

- [Filosofia](#filosofia)
- [Instalação](#instalação)
- [Módulos](#módulos)
- [Uso Rápido](#uso-rápido)
- [Gráficos](#gráficos)
- [Janelas](#janelas)
- [Input](#input)
- [Changelog](#changelog)

---

## ✨ Filosofia

- **No-std**: Zero dependências de runtime
- **Type-safe**: Handles tipados, erros explícitos
- **Capability-based**: Segue modelo do kernel
- **GFX-Powered**: Tipos gráficos completos via `gfx_types`
- **Math-Included**: Funções matemáticas via `rdsmath`

---

## 📦 Instalação

```toml
[dependencies]
redpowder = { path = "../sdk/redpowder" }
```

---

## 📁 Módulos

| Módulo | Função |
|--------|--------|
| `syscall` | Invocação de syscalls (inline asm) |
| `console` | print!, println!, reboot, poweroff |
| `fs` | Arquivos e diretórios (File, Dir, stat) |
| `process` | Processos (exit, spawn, yield) |
| `mem` | Memória (alloc, free, map) |
| `ipc` | IPC (Port, send, recv) |
| `time` | Tempo (sleep, clock) |
| `io` | Handle, Rights |
| `event` | Eventos e polling |
| `sys` | sysinfo, debug |
| `graphics` | Framebuffer, canvas, desenho |
| `input` | Mouse, teclado, touch |
| `window` | Janelas (protocolo Firefly) |
| `gfx` | Re-export completo de `gfx_types` |
| `math` | Re-export de `rdsmath` |

---

## 🚀 Uso Rápido

```rust
#![no_std]
#![no_main]

use redpowder::prelude::*;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello from RedstoneOS!");
    
    // Geometria
    let rect = Rect::new(10, 10, 100, 50);
    let point = Point::new(50, 30);
    println!("Rect contains point: {}", rect.contains_point(point));
    
    // Cores
    let bg = Color::from_hex(0x1e1e2e);
    let fg = Color::WHITE;
    
    // Math
    let angle = PI / 4.0;
    let s = sinf(angle);
    let c = cosf(angle);
    
    exit(0);
}
```

---

## 🎨 Gráficos

### Framebuffer Direto

```rust
use redpowder::graphics::{Framebuffer, Color, Rect};

let mut fb = Framebuffer::new()?;
fb.clear(Color::BLACK);
fb.fill(Rect::new(10, 10, 100, 50), Color::RED);
```

### Canvas (Buffer Local)

```rust
use redpowder::graphics::{Canvas, Color, Rect};

let mut buffer = vec![0u32; 800 * 600];
let mut canvas = Canvas::new(&mut buffer, 800, 600);

canvas.clear(Color::from_hex(0x1e1e2e));
canvas.fill_rect(Rect::new(10, 10, 100, 50), Color::RED);
canvas.stroke_rect(Rect::new(10, 10, 100, 50), Color::WHITE, 1);
canvas.line(0, 0, 100, 100, Color::BLUE);
canvas.stroke_circle(50, 50, 30, Color::GREEN);
```

### Primitivas de Desenho

```rust
use redpowder::graphics::{draw_line, draw_circle, draw_rect};
use redpowder::prelude::*;

// Iteradores sobre os pontos
for point in draw_line(Line::new(Point::new(0, 0), Point::new(100, 100))) {
    canvas.put_pixel(point.x, point.y, Color::WHITE);
}

for point in draw_circle(Circle::from_coords(50.0, 50.0, 30.0)) {
    canvas.put_pixel(point.x, point.y, Color::GREEN);
}
```

---

## 🪟 Janelas

```rust
use redpowder::window::{Window, WindowFlags};
use redpowder::prelude::*;

// Criar janela
let mut window = Window::create(100, 100, 400, 300, "Minha App")?;

// Ou com flags
let mut window = Window::create_with_flags(
    100, 100, 400, 300,
    WindowFlags::HAS_SHADOW | WindowFlags::BORDERLESS,
    "App Premium"
)?;

// Desenhar no buffer
window.clear(Color::from_hex(0x1e1e2e));
window.fill_rect(Rect::new(10, 10, 100, 50), Color::RED);

// Apresentar
window.present()?;

// Loop de eventos
loop {
    for event in window.poll_events() {
        match event {
            Event::Input(input) => { /* mouse, teclado */ }
            Event::Resize(resize) => { /* redimensionado */ }
            _ => {}
        }
    }
    
    // Atualizar...
    window.present()?;
}
```

---

## ⌨️ Input

### Mouse

```rust
use redpowder::input::{poll_mouse, MouseButton};

let mouse = poll_mouse()?;
println!("Mouse: ({}, {})", mouse.x, mouse.y);

if mouse.left_button() {
    println!("Clique!");
}

if mouse.is_pressed(MouseButton::Right) {
    println!("Botão direito!");
}
```

### Teclado

```rust
use redpowder::input::{poll_keyboard, read_key, KeyEvent, KeyCode};

// Ler um único evento
if let Some(event) = read_key()? {
    if event.pressed {
        match event.keycode() {
            KeyCode::Enter => println!("Enter!"),
            KeyCode::Esc => break,
            key => {
                if let Some(c) = key.to_char(false) {
                    print!("{}", c);
                }
            }
        }
    }
}

// Ler múltiplos eventos
let mut events = [KeyEvent::default(); 16];
let count = poll_keyboard(&mut events)?;
for event in &events[..count] {
    // processar...
}
```

---

## 🔢 Math (via rdsmath)

```rust
use redpowder::prelude::*;
// ou
use redpowder::math::*;

// Constantes
let pi = PI;
let tau = TAU;

// Trigonometria
let s = sinf(angle);
let c = cosf(angle);
let t = tanf(angle);
let a = atan2f(y, x);

// Raiz quadrada
let root = sqrtf(2.0);

// Interpolação
let value = lerpf(0.0, 100.0, 0.5); // = 50.0
let smooth = smoothstepf(0.0, 1.0, 0.5);

// Clamp
let clamped = clampf(150.0, 0.0, 100.0); // = 100.0
let sat = saturatef(1.5); // = 1.0

// Arredondamento
let floor = floorf(3.7); // = 3.0
let ceil = ceilf(3.2); // = 4.0
let round = roundf(3.5); // = 4.0
let abs = absf(-5.0); // = 5.0
```

---

## 📝 Changelog

### v0.3.0 (Current)
- Refatoração completa dos módulos `graphics`, `window` e `input`
- Integração total com `gfx_types` v0.2.0
- Integração com `rdsmath` v0.1.0
- Estrutura modular (submódulos especializados)
- Novos tipos: `Canvas`, `Framebuffer`, primitivas de desenho
- Re-exports de tipos gráficos no prelude

### v0.2.0
- Adição de módulo de janelas
- Suporte a IPC

### v0.1.0
- Versão inicial

---

## 📄 Licença

MIT License - RedstoneOS Team
