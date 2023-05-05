use sdl2::{render::{Canvas, RenderTarget}, rect::Rect, pixels::Color};

const WIDTH: usize = 64;
const HEIGTH: usize = 32;

pub struct CH8Display<T: RenderTarget> {
    ch8_display: [[u8; HEIGTH]; WIDTH],
    canvas: Canvas<T>
}

impl<T: RenderTarget> CH8Display<T> {
    pub fn new(canvas: Canvas<T>) -> CH8Display<T> {
        let ch8_display = CH8Display {
            ch8_display: [[0; HEIGTH]; WIDTH],
            canvas
        };
        ch8_display
    }

    pub fn clear(&mut self) {
        self.canvas.clear();
        self.ch8_display = [[0; HEIGTH]; WIDTH];
    }

    pub fn draw_sprite(&mut self, mut x_coord: usize, mut y_coord: usize, vec: Vec<u8>) -> bool {
        let mut changed = false;
        let base_x_coord = x_coord;
        let base_y_coord = y_coord;
        for i in &vec {
            let mut offset = 7;
            while offset >= 0 && x_coord < WIDTH {
                let select_bit = (i >> offset) & 0x1;
                let pixel = &mut self.ch8_display[x_coord][y_coord];
                if *pixel != select_bit {
                    changed = true;
                }
                *pixel ^= select_bit;
                offset -= 1;
                x_coord += 1;
            }
            y_coord += 1;
            x_coord = base_x_coord;
            if y_coord >= HEIGTH {
                break;
            }
        }
        self.render(base_x_coord, base_y_coord, vec.len());
        changed
    }

    fn render(&mut self, x_coord: usize, y_coord: usize, heigth: usize) {
        for i in y_coord..y_coord + heigth {
            for j in x_coord..x_coord + 8 {
                match self.ch8_display[j][i] {
                    0 => self.canvas.set_draw_color(Color::RGB(0, 0, 0)),
                    1 => self.canvas.set_draw_color(Color::RGB(255, 255,255)),
                    _ => panic!()
                }
                self.canvas.fill_rect(Rect::new(j as i32 * 5, i as i32 * 5, 5, 5));
            }
        }
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.present();
    }
}