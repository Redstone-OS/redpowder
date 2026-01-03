//! # Draw Primitives
//!
//! Funções de desenho de primitivas geométricas.

use gfx_types::geometry::{Circle, Line, Point, Rect};

// =============================================================================
// LINHA (Bresenham)
// =============================================================================

/// Desenha uma linha usando algoritmo de Bresenham.
///
/// Retorna iterador sobre os pontos da linha.
pub fn draw_line(line: Line) -> impl Iterator<Item = Point> {
    LineIterator::new(line.start, line.end)
}

/// Desenha uma linha entre dois pontos (conveniente).
pub fn line_points(x0: i32, y0: i32, x1: i32, y1: i32) -> impl Iterator<Item = Point> {
    LineIterator::new(Point::new(x0, y0), Point::new(x1, y1))
}

struct LineIterator {
    x: i32,
    y: i32,
    x1: i32,
    y1: i32,
    dx: i32,
    dy: i32,
    sx: i32,
    sy: i32,
    err: i32,
    done: bool,
}

impl LineIterator {
    fn new(start: Point, end: Point) -> Self {
        let dx = (end.x - start.x).abs();
        let dy = -(end.y - start.y).abs();
        let sx = if start.x < end.x { 1 } else { -1 };
        let sy = if start.y < end.y { 1 } else { -1 };
        let err = dx + dy;

        Self {
            x: start.x,
            y: start.y,
            x1: end.x,
            y1: end.y,
            dx,
            dy,
            sx,
            sy,
            err,
            done: false,
        }
    }
}

impl Iterator for LineIterator {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        let current = Point::new(self.x, self.y);

        if self.x == self.x1 && self.y == self.y1 {
            self.done = true;
            return Some(current);
        }

        let e2 = 2 * self.err;
        if e2 >= self.dy {
            self.err += self.dy;
            self.x += self.sx;
        }
        if e2 <= self.dx {
            self.err += self.dx;
            self.y += self.sy;
        }

        Some(current)
    }
}

// =============================================================================
// RETÂNGULO
// =============================================================================

/// Desenha borda de retângulo.
///
/// Retorna iterador sobre os pontos da borda.
pub fn draw_rect(rect: Rect) -> impl Iterator<Item = Point> {
    RectBorderIterator::new(rect)
}

struct RectBorderIterator {
    rect: Rect,
    side: u8, // 0=top, 1=right, 2=bottom, 3=left
    pos: i32,
}

impl RectBorderIterator {
    fn new(rect: Rect) -> Self {
        Self {
            rect,
            side: 0,
            pos: 0,
        }
    }
}

impl Iterator for RectBorderIterator {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.side > 3 {
                return None;
            }

            let point = match self.side {
                0 => {
                    // Top
                    if self.pos >= self.rect.width as i32 {
                        self.side = 1;
                        self.pos = 1;
                        continue;
                    }
                    Point::new(self.rect.x + self.pos, self.rect.y)
                }
                1 => {
                    // Right
                    if self.pos >= self.rect.height as i32 {
                        self.side = 2;
                        self.pos = self.rect.width as i32 - 2;
                        continue;
                    }
                    Point::new(self.rect.right() - 1, self.rect.y + self.pos)
                }
                2 => {
                    // Bottom
                    if self.pos < 0 {
                        self.side = 3;
                        self.pos = self.rect.height as i32 - 2;
                        continue;
                    }
                    Point::new(self.rect.x + self.pos, self.rect.bottom() - 1)
                }
                3 => {
                    // Left
                    if self.pos <= 0 {
                        self.side = 4;
                        continue;
                    }
                    Point::new(self.rect.x, self.rect.y + self.pos)
                }
                _ => return None,
            };

            self.pos += if self.side == 2 || self.side == 3 {
                -1
            } else {
                1
            };
            return Some(point);
        }
    }
}

// =============================================================================
// CÍRCULO (Bresenham/Midpoint)
// =============================================================================

/// Desenha borda de círculo.
///
/// Retorna iterador sobre os pontos da borda.
pub fn draw_circle(circle: Circle) -> impl Iterator<Item = Point> {
    CircleIterator::new(
        circle.center.x as i32,
        circle.center.y as i32,
        circle.radius as i32,
    )
}

/// Desenha círculo dado centro e raio (conveniente).
pub fn circle_points(cx: i32, cy: i32, radius: i32) -> impl Iterator<Item = Point> {
    CircleIterator::new(cx, cy, radius)
}

struct CircleIterator {
    cx: i32,
    cy: i32,
    x: i32,
    y: i32,
    d: i32,
    octant: u8,
    done: bool,
}

impl CircleIterator {
    fn new(cx: i32, cy: i32, radius: i32) -> Self {
        Self {
            cx,
            cy,
            x: 0,
            y: radius,
            d: 1 - radius,
            octant: 0,
            done: radius <= 0,
        }
    }

    fn get_point(&self) -> Point {
        match self.octant {
            0 => Point::new(self.cx + self.x, self.cy - self.y),
            1 => Point::new(self.cx + self.y, self.cy - self.x),
            2 => Point::new(self.cx + self.y, self.cy + self.x),
            3 => Point::new(self.cx + self.x, self.cy + self.y),
            4 => Point::new(self.cx - self.x, self.cy + self.y),
            5 => Point::new(self.cx - self.y, self.cy + self.x),
            6 => Point::new(self.cx - self.y, self.cy - self.x),
            7 => Point::new(self.cx - self.x, self.cy - self.y),
            _ => Point::ZERO,
        }
    }
}

impl Iterator for CircleIterator {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        let point = self.get_point();

        self.octant += 1;
        if self.octant >= 8 {
            self.octant = 0;

            // Avançar algoritmo
            if self.d < 0 {
                self.d += 2 * self.x + 3;
            } else {
                self.d += 2 * (self.x - self.y) + 5;
                self.y -= 1;
            }
            self.x += 1;

            if self.x > self.y {
                self.done = true;
            }
        }

        Some(point)
    }
}

// =============================================================================
// FILLED SHAPES
// =============================================================================

/// Preenche um círculo.
pub fn fill_circle(circle: Circle) -> impl Iterator<Item = (i32, i32, i32)> {
    FilledCircleIterator::new(
        circle.center.x as i32,
        circle.center.y as i32,
        circle.radius as i32,
    )
}

/// Retorna linhas horizontais para preencher um círculo.
/// Cada item é (x, y, width).
struct FilledCircleIterator {
    cx: i32,
    cy: i32,
    x: i32,
    y: i32,
    d: i32,
    phase: u8, // 0 = top/bottom lines, 1 = left/right expansion
    done: bool,
}

impl FilledCircleIterator {
    fn new(cx: i32, cy: i32, radius: i32) -> Self {
        Self {
            cx,
            cy,
            x: 0,
            y: radius,
            d: 1 - radius,
            phase: 0,
            done: radius <= 0,
        }
    }
}

impl Iterator for FilledCircleIterator {
    type Item = (i32, i32, i32); // (x, y, width)

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        // Retorna linhas horizontais para preencher o círculo
        let result = match self.phase {
            0 => {
                // Linha no topo e embaixo (usando y)
                self.phase = 1;
                Some((self.cx - self.y, self.cy - self.x, 2 * self.y + 1))
            }
            1 => {
                self.phase = 2;
                if self.x != 0 {
                    Some((self.cx - self.y, self.cy + self.x, 2 * self.y + 1))
                } else {
                    self.next() // Skip duplicate center line
                }
            }
            _ => {
                self.phase = 0;

                // Avançar algoritmo
                if self.d < 0 {
                    self.d += 2 * self.x + 3;
                } else {
                    // Desenha linhas usando x antes de decrementar y
                    let result = if self.x != self.y {
                        Some((self.cx - self.x, self.cy - self.y, 2 * self.x + 1))
                    } else {
                        None
                    };

                    self.d += 2 * (self.x - self.y) + 5;
                    self.y -= 1;

                    if result.is_some() {
                        return result;
                    }
                }
                self.x += 1;

                if self.x > self.y {
                    self.done = true;
                    return None;
                }

                self.next()
            }
        };

        result
    }
}
