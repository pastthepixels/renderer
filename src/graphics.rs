extern crate sdl2;

use sdl2::video::Window;

use crate::math::{Vector2, Vector3};

pub struct Color(pub u8, pub u8, pub u8);

pub struct Renderer {
    pub clear_color: Color,
    canvas: sdl2::render::Canvas<Window>,
    event_pump: sdl2::EventPump,
    running: bool,
}

impl Color {
    pub fn to_sdl_color(&self) -> sdl2::pixels::Color {
        sdl2::pixels::Color::RGB(self.0, self.1, self.2)
    }
}

impl Renderer {
    /// Creates a new Renderer
    pub fn new(title: &str, width: u32, height: u32) -> Renderer {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window(title, width, height)
            .position_centered()
            .build()
            .unwrap();

        Renderer {
            clear_color: Color(0, 0, 0),
            canvas: window.into_canvas().build().unwrap(),
            event_pump: sdl_context.event_pump().unwrap(),
            running: true,
        }
    }

    /// Initialises and runs a renderer with SDL2
    /// TODO: removeme
    pub fn run(&mut self) {
        let mut fps_manager = sdl2::gfx::framerate::FPSManager::new();
    }

    /// Returns true if the renderer is running
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Draws everything, must be called at the end of each loop
    pub fn update(&mut self) {
        for event in self.event_pump.poll_iter() {
            if let sdl2::event::Event::Quit { .. } = event {
                self.running = false;
            }
        }
        self.canvas.present();
        ::std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 60));
    }

    /// Clears the canvas, must be called at the start of each loop
    pub fn clear(&mut self) {
        self.canvas.set_draw_color(self.clear_color.to_sdl_color());
        self.canvas.clear();
    }

    /// Draws a barycentric triangle
    /// TODO: incorporate fill and depth (depth: make them vec3's)
    pub fn draw_triangle(&mut self, a: &Vector2, b: &Vector2, c: &Vector2) {
        // Get bounding box
        let max_x = a.x.max(b.x).max(c.x);
        let max_y = a.y.max(b.y).max(c.y);
        let min_x = a.x.min(b.x).min(c.x);
        let min_y = a.y.min(b.y).min(c.y);
        // TODO: this comment
        let top_left = self.get_barycentric_coords(a, b, c, &Vector2::new(min_x, min_y));
        let delta_y =
            self.get_barycentric_coords(a, b, c, &Vector2::new(min_x, min_y + 1.)) - top_left;
        let delta_x =
            self.get_barycentric_coords(a, b, c, &Vector2::new(min_x + 1., min_y)) - top_left;
        // TODO: this comment
        for y in (min_y as i32)..(max_y as i32 + 1) {
            let coords_row = top_left + (delta_y * ((y as f32) - min_y));
            for x in (min_x as i32)..(max_x as i32 + 1) {
                // TODO: don't overdraw
                let coords = coords_row + (delta_x * ((x as f32) - min_x));
                if coords.x >= 0.
                    && coords.y >= 0.
                    && coords.z >= 0.
                    && coords.x + coords.y + coords.z >= 0.99
                {
                    // TODO: depth
                    self.canvas.set_draw_color(sdl2::pixels::Color::RGB(
                        (coords.x * 255.) as u8,
                        (coords.y * 255.) as u8,
                        (coords.z * 255.) as u8,
                    ));
                    self.canvas.draw_point(sdl2::rect::Point::new(x, y));
                }
            }
        }
    }

    fn get_barycentric_coords(
        &self,
        a: &Vector2,
        b: &Vector2,
        c: &Vector2,
        p: &Vector2,
    ) -> Vector3 {
        let vec0 = *b - *a;
        let vec1 = *c - *a;
        let vec2 = *p - *a;

        let d00 = vec0.dot_product(&vec0);
        let d01 = vec0.dot_product(&vec1);
        let d11 = vec1.dot_product(&vec1);
        let d20 = vec2.dot_product(&vec0);
        let d21 = vec2.dot_product(&vec1);

        let det = d00 * d11 - d01 * d01;

        if det == 0. {
            Vector3 {
                x: 0.,
                y: 0.,
                z: 0.,
            }
        } else {
            let v = (d11 * d20 - d01 * d21) / det;
            let w = (d00 * d21 - d01 * d20) / det;
            Vector3 {
                x: 1.0 - v - w,
                y: v,
                z: w,
            }
        }
    }
}
