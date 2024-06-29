use std::ops::Deref;

use crate::graphics::Color;
use crate::graphics::Renderer;
use crate::math::Matrix44;
use crate::math::Vector2;
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
pub struct Face {
    pub a: usize,
    pub b: usize,
    pub c: usize,
    pub normal: Vector3,
    pub uva: usize,
    pub uvb: usize,
    pub uvc: usize,
}

impl Face {
    /// Creates a new Face.
    /// Faces store vertices as indices in a list (or Rust vector) containing Vector3.
    pub fn new(a: usize, b: usize, c: usize, uva: usize, uvb: usize, uvc: usize) -> Face {
        // TODO: pointer instead of copy
        Face {
            a,
            b,
            c,
            uva,
            uvb,
            uvc,
            normal: Vector3::new(0., 0., 0.),
        }
    }

    pub fn compute_normal(&mut self, vertices_list: &[Vector3]) {
        self.normal = (vertices_list[self.a] - vertices_list[self.b])
            .cross_product(&(vertices_list[self.b] - vertices_list[self.c]))
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
    pub uvs: Vec<Vector2>,
    pub transformation: Transformation,
    pub shader: Box<dyn shaders::Shader>,
}

impl Mesh {
    pub fn new(vertices: Vec<Vector3>, faces: Vec<Face>, uvs: Vec<Vector2>) -> Mesh {
        Mesh {
            vertices,
            faces,
            uvs,
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
            let a = vertices_projected[face.a];
            let b = vertices_projected[face.b];
            let c = vertices_projected[face.c];
            if (b.x - a.x) * (c.y - a.y) - (c.x - a.x) * (b.y - a.y) > 0. {
                let brightness = self
                    .shader
                    .calculate_lighting(&self.transformation.transformed(&face.normal), world);
                // TODO: Don't need to calculate perspective correct textures -- yet?
                /*let aw = world
                    .camera
                    .get_w(&self.vertices[face.a], &self.transformation);
                let bw = world
                    .camera
                    .get_w(&self.vertices[face.b], &self.transformation);
                let cw = world
                    .camera
                    .get_w(&self.vertices[face.c], &self.transformation);*/
                renderer.draw_triangle(
                    &a,
                    &b,
                    &c,
                    (&self.uvs[face.uva], 1.),
                    (&self.uvs[face.uvb], 1.),
                    (&self.uvs[face.uvc], 1.),
                    self.shader.as_ref(),
                    brightness,
                );
            }
        }
    }
}
