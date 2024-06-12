use crate::camera::PerspectiveCamera;
use crate::graphics::Color;
use crate::graphics::Renderer;
use crate::math::Vector3;

//
// Faces
//
#[derive(Copy, Clone)]
pub struct Face(u32, u32, u32, Vector3);

impl Face {
    /// Creates a new Face.
    /// Faces store vertices as indices in a list (or Rust vector) containing Vector3.
    pub fn new(a: u32, b: u32, c: u32) -> Face {
        // TODO: pointer instead of copy
        Face(a, b, c, Vector3::new(0., 0., 0.)) // TODO: compute_normal
    }

    pub fn compute_normal(&mut self, vertices_list: &Vec<Vector3>) {
        self.3 = (vertices_list[self.0 as usize] - vertices_list[self.1 as usize])
            .cross_product(&(vertices_list[self.1 as usize] - vertices_list[self.2 as usize]))
            .normalised();
    }
}

//
// Transformations
//
#[derive(Copy, Clone)]
pub struct Transformation {
    pub position: Vector3,
    pub scale: f32,
}

//
// Meshes
//
pub struct Mesh {
    pub vertices: Vec<Vector3>,
    pub faces: Vec<Face>,
    pub transformation: Transformation,
    pub color: Color,
}

impl Mesh {
    pub fn new(vertices: Vec<Vector3>, faces: Vec<Face>) -> Mesh {
        Mesh {
            vertices,
            faces,
            transformation: Transformation {
                position: Vector3::new(0., 0., 0.),
                scale: 1.0,
            },
            color: Color(255, 255, 255),
        }
    }

    pub fn draw(&self, renderer: &mut Renderer, camera: &PerspectiveCamera) {
        for face in &self.faces {
            // TODO: incorporate transformations
            // TODO: hard-coded light
            let color = self.color * (1. - face.3.cos_similarity(&Vector3::new(0., -1., 1.)));
            let a = camera.to_ndc(&camera.project_point(&self.vertices[face.0 as usize]));
            let b = camera.to_ndc(&camera.project_point(&self.vertices[face.1 as usize]));
            let c = camera.to_ndc(&camera.project_point(&self.vertices[face.2 as usize]));
            if (b - a).x * (c - a).y - (c - a).x * (b - a).y <= 0. {
                renderer.draw_triangle(&a, &b, &c, &color);
            }
        }
    }
}
