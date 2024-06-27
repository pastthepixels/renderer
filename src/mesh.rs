use std::ops::Deref;

use crate::graphics::Color;
use crate::graphics::Renderer;
use crate::math::Matrix44;
use crate::math::Vector3;
use crate::math::Vector4;
use crate::shaders;
use crate::shaders::Shader;
use crate::shaders::StandardShader;
use crate::world::World;

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

    pub fn compute_normal(&mut self, vertices_list: &[Vector3]) {
        self.3 = (vertices_list[self.0 as usize] - vertices_list[self.1 as usize])
            .cross_product(&(vertices_list[self.1 as usize] - vertices_list[self.2 as usize]))
            .normalised();
    }
}

//
// Transformations
//
pub struct Transformation {
    pub position: Vector3,
    pub scale: f32,
    pub quaternion: Vector4,
    affine_matrix: Matrix44,
}

impl Transformation {
    pub fn transformed(&self, point: &Vector3) -> Vector3 {
        self.affine_matrix.multiply_vec3(point)
    }

    /// Generates an affine transformation matrix. Needs to be called after the position/quaternion
    /// is mutated.
    pub fn generate_affine_matrix(&mut self) {
        self.affine_matrix.data = vec![
            1. - (2. * self.quaternion.y.powi(2) + 2. * self.quaternion.z.powi(2)),
            2. * self.quaternion.x * self.quaternion.y + 2. * self.quaternion.z * self.quaternion.w,
            2. * self.quaternion.x * self.quaternion.z - 2. * self.quaternion.y * self.quaternion.w,
            0.,
            2. * self.quaternion.x * self.quaternion.y - 2. * self.quaternion.w * self.quaternion.z,
            1. - (2. * self.quaternion.x.powi(2) + 2. * self.quaternion.z.powi(2)),
            2. * self.quaternion.y * self.quaternion.z + 2. * self.quaternion.w * self.quaternion.x,
            0.,
            2. * self.quaternion.x * self.quaternion.z + 2. * self.quaternion.w * self.quaternion.y,
            2. * self.quaternion.y * self.quaternion.z - 2. * self.quaternion.w * self.quaternion.x,
            1. - (2. * self.quaternion.x.powi(2) + 2. * self.quaternion.y.powi(2)),
            0.,
            self.position.x,
            self.position.y,
            self.position.z,
            1.,
        ]
    }
}

//
// Meshes
//
pub struct Mesh {
    pub vertices: Vec<Vector3>,
    pub faces: Vec<Face>,
    pub transformation: Transformation,
    pub shader: Box<dyn shaders::Shader>,
}

impl Mesh {
    pub fn new(vertices: Vec<Vector3>, faces: Vec<Face>) -> Mesh {
        Mesh {
            vertices,
            faces,
            transformation: Transformation {
                position: Vector3::new(0., 0., 0.),
                scale: 1.0,
                quaternion: Vector4::new(0., 0., 0., 1.),
                affine_matrix: Matrix44 { data: Vec::new() },
            },
            shader: Box::new(StandardShader::new(Color(255, 255, 255))),
        }
    }

    /// Draws the mesh onto the screen. Must be called after an affine matrix is generated for its
    /// transformation (see Transformation.generate_affine_matrix)
    pub fn draw(&mut self, renderer: &mut Renderer, world: &World) {
        let vertices_projected: Vec<Vector3> = self
            .vertices
            .clone()
            .iter()
            .map(|vertex| {
                world
                    .camera
                    .to_ndc(world.camera.project_point(vertex, &self.transformation))
            })
            .collect();
        for face in &self.faces {
            let a = vertices_projected[face.0 as usize];
            let b = vertices_projected[face.1 as usize];
            let c = vertices_projected[face.2 as usize];
            if (b.x - a.x) * (c.y - a.y) - (c.x - a.x) * (b.y - a.y) > 0. {
                let brightness = self
                    .shader
                    .calculate_lighting(&self.transformation.transformed(&face.3), world);
                renderer.draw_triangle(&a, &b, &c, self.shader.as_ref(), brightness);
            }
        }
    }
}
