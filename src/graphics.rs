extern crate sdl2;

use sdl2::video::Window;

use crate::{camera::PerspectiveCamera, math::Vector3};

#[derive(Copy, Clone)]
pub struct Color(pub u8, pub u8, pub u8);

pub struct Renderer {
    pub clear_color: Color,
    canvas: sdl2::render::Canvas<Window>,
    event_pump: sdl2::EventPump,
    running: bool,
    depth_buffer: Vec<f32>,
    empty_buffer: Vec<f32>,
    width: u32,
    height: u32,
}

impl Color {
    pub fn to_sdl_color(self) -> sdl2::pixels::Color {
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
            .resizable()
            .build()
            .unwrap();

        Renderer {
            clear_color: Color(0, 0, 0),
            canvas: window.into_canvas().build().unwrap(),
            event_pump: sdl_context.event_pump().unwrap(),
            running: true,
            depth_buffer: Vec::new(),
            empty_buffer: vec![-1.; (width * height) as usize],
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
                self.empty_buffer = vec![-1.; (width * height) as usize];
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
        self.canvas.set_draw_color(self.clear_color.to_sdl_color());
        self.canvas.clear();
    }

    /// Draws a barycentric triangle
    pub fn draw_triangle(&mut self, a: &Vector3, b: &Vector3, c: &Vector3, color: &Color) {
        // TODO: image buffer
        self.canvas.set_draw_color(color.to_sdl_color());
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
        for y in min_y..(max_y + 1) {
            // Barycentric coordinates for the left of the row.
            let coords_row = top_left + (delta_y * (y - min_y) as f32);
            for x in min_x..(max_x + 1) {
                // Index of the point in the depth buffer
                let depth_index = self.width as usize * y as usize + x as usize;
                if depth_index < self.depth_buffer.len() && x < self.width.try_into().unwrap() {
                    // Barycentric coordinates
                    let coords = coords_row + delta_x * ((x - min_x) as f32);
                    if coords.x >= 0.
                        && coords.y >= 0.
                        && coords.z >= 0.
                        && coords.x + coords.y + coords.z >= 0.99
                    {
                        // Depth
                        let depth_entry = self.depth_buffer[depth_index];
                        let depth = coords.x * a.z + coords.y * b.z + coords.z * c.z;
                        if (depth_entry > depth || depth_entry == -1.) && depth > 0. {
                            // Write to screen / depth buffer
                            self.depth_buffer[depth_index] = depth;
                            self.canvas
                                .draw_point(sdl2::rect::Point::new(x, y))
                                .expect(":(");
                        }
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
        let d00 = (b_x - a_x).powi(2) + (b_y - a_y).powi(2);
        let d01 = (b_x - a_x) * (c_x - a_x) + (b_y - a_y) * (c_y - a_y);
        let d11 = (c_x - a_x).powi(2) + (c_y - a_y).powi(2);
        let d20 = (b_x - a_x) * (p_x - a_x) + (b_y - a_y) * (p_y - a_y);
        let d21 = (p_x - a_x) * (c_x - a_x) + (p_y - a_y) * (c_y - a_y);
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
                x: 1. - v - w,
                y: v,
                z: w,
            }
        }
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
