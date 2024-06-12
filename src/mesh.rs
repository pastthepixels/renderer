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

    fn compute_normal(&self, vertices_list: &Vec<Vector3>) {
        todo!(); // TODO: implement
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
            renderer.draw_triangle(
                &camera.to_ndc(&camera.project_point(&self.vertices[face.0 as usize])),
                &camera.to_ndc(&camera.project_point(&self.vertices[face.1 as usize])),
                &camera.to_ndc(&camera.project_point(&self.vertices[face.2 as usize])),
            );
        }
    }
}
