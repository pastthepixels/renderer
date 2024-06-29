extern crate sdl2;

use sdl2::video::Window;

use crate::{
    camera::PerspectiveCamera,
    math::{self, Vector2, Vector3},
    shaders::Shader,
};

#[derive(Copy, Clone)]
pub struct Color(pub u8, pub u8, pub u8);

pub struct Renderer {
    pub clear_color: Color,
    canvas: sdl2::render::Canvas<Window>,
    event_pump: sdl2::EventPump,
    running: bool,
    depth_buffer: Box<[f32]>,
    empty_buffer: Box<[f32]>,
    width: u32,
    height: u32,
}

impl Renderer {
    /// Creates a new Renderer
    pub fn new(title: &str, width: u32, height: u32) -> Renderer {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window(title, width, height)
            .position_centered()
            .resizable()
            .build()
            .unwrap();

        Renderer {
            clear_color: Color(0, 0, 0),
            canvas: window.into_canvas().build().unwrap(),
            event_pump: sdl_context.event_pump().unwrap(),
            running: true,
            depth_buffer: Box::new([]),
            empty_buffer: vec![0.; (width * height) as usize].into_boxed_slice(),
            width,
            height,
        }
    }

    /// Returns true if the renderer is running
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Automatically resizes the camera if the screen size changes
    pub fn auto_resize(&mut self, camera: &mut PerspectiveCamera) {
        for event in self.event_pump.poll_iter() {
            if let sdl2::event::Event::Window {
                win_event: sdl2::event::WindowEvent::Resized(width, height),
                ..
            } = event
            {
                self.width = width as u32;
                self.height = height as u32;
                camera.size.x = width as f32;
                camera.size.y = height as f32;
                camera.aspect = width as f32 / height as f32;
                camera.generate_projection_matrix();
                self.empty_buffer = vec![0.; (width * height) as usize].into_boxed_slice();
            }
        }
    }

    /// Draws everything, must be called at the end of each loop
    pub fn update(&mut self) {
        for event in self.event_pump.poll_iter() {
            if let sdl2::event::Event::Quit { .. } = event {
                self.running = false;
            }
        }
        self.canvas.present();
        ::std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 75));
    }

    /// Clears the canvas, must be called at the start of each loop
    pub fn clear(&mut self) {
        self.depth_buffer = self.empty_buffer.clone();

        self.canvas.set_draw_color(sdl2::pixels::Color::RGB(
            self.clear_color.0,
            self.clear_color.1,
            self.clear_color.2,
        ));
        self.canvas.clear();
    }

    /// Draws a barycentric triangle
    pub fn draw_triangle(
        &mut self,
        a: &Vector3,
        b: &Vector3,
        c: &Vector3,
        uva: &Vector2,
        uvb: &Vector2,
        uvc: &Vector2,
        shader: &dyn Shader,
        brightness: f32,
    ) {
        // TODO: image buffer
        // Get bounding box (and then clip to screen bounds)
        let max_x =
            (self.width as i32).min(*[a.x, b.x, c.x].map(|y| y as i32).iter().max().unwrap());
        let max_y =
            (self.height as i32).min(*[a.y, b.y, c.y].map(|y| y as i32).iter().max().unwrap());
        let min_x = 0.max(*[a.x, b.x, c.x].map(|x| x as i32).iter().min().unwrap());
        let min_y = 0.max(*[a.y, b.y, c.y].map(|y| y as i32).iter().min().unwrap());
        // Get the barycentric coordinates at the top left and when x or y increments
        let top_left = Renderer::get_barycentric_coords(
            a.x,
            a.y,
            b.x,
            b.y,
            c.x,
            c.y,
            min_x as f32,
            min_y as f32,
        );
        let delta_y = Renderer::get_barycentric_coords(
            a.x,
            a.y,
            b.x,
            b.y,
            c.x,
            c.y,
            min_x as f32,
            (min_y + 1) as f32,
        ) - top_left;
        let delta_x = Renderer::get_barycentric_coords(
            a.x,
            a.y,
            b.x,
            b.y,
            c.x,
            c.y,
            (min_x + 1) as f32,
            min_y as f32,
        ) - top_left;
        // Drawing
        let mut coords_row = top_left;
        for y in min_y..(max_y + 1) {
            // Barycentric coordinates for the left of the row.
            let mut coords = coords_row + delta_y;
            coords_row = coords;
            for x in min_x..(max_x + 1) {
                // Barycentric coordinates
                coords = coords + delta_x;
                if coords.x >= 0. && coords.y >= 0. && coords.z >= 0. {
                    // Depth
                    let depth_index = self.width as usize * y as usize + x as usize;
                    let depth_entry = self.depth_buffer[depth_index];
                    let depth = coords.x * a.z + coords.y * b.z + coords.z * c.z;
                    if depth > 0. && (depth_entry > depth || depth_entry == 0.) {
                        // Write to screen / depth buffer
                        self.depth_buffer[depth_index] = depth;
                        self.draw_pixel(
                            x,
                            y,
                            &(shader.fragment(&coords, uva, uvb, uvc) * brightness),
                        );
                    }
                }
            }
        }
    }

    fn get_barycentric_coords(
        a_x: f32,
        a_y: f32,
        b_x: f32,
        b_y: f32,
        c_x: f32,
        c_y: f32,
        p_x: f32,
        p_y: f32,
    ) -> Vector3 {
        let dca = math::Vector2::new(c_x - a_x, c_y - a_y);
        let dpa = math::Vector2::new(p_x - a_x, p_y - a_y);
        let dba = math::Vector2::new(b_x - a_x, b_y - a_y);
        let deti = 1. / (dba.x * dca.y - dca.x * dba.y);

        let v = (dpa.x * dca.y - dpa.y * dca.x) * deti;
        let w = (dpa.y * dba.x - dpa.x * dba.y) * deti;

        Vector3 {
            x: 1. - v - w,
            y: v,
            z: w,
        }
    }

    fn draw_pixel(&mut self, x: i32, y: i32, color: &Color) {
        self.canvas
            .set_draw_color(sdl2::pixels::Color::RGB(color.0, color.1, color.2));
        self.canvas
            .draw_point(sdl2::rect::Point::new(x, y))
            .expect(":(");
    }
}

impl std::ops::Mul<f32> for Color {
    type Output = Color;

    fn mul(self, rhs: f32) -> Self::Output {
        Color(
            (self.0 as f32 * rhs) as u8,
            (self.1 as f32 * rhs) as u8,
            (self.2 as f32 * rhs) as u8,
        )
    }
}
