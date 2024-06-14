use crate::camera::PerspectiveCamera;
use crate::graphics::Color;
use crate::graphics::Renderer;
use crate::math::Matrix44;
use crate::math::Vector3;
use crate::math::Vector4;
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
    pub quaternion: Vector4,
}

impl Transformation {
    pub fn transformed(&self, point: &Vector3) -> Vector3 {
        // Rotate
        let matrix = Matrix44 {
            data: vec![
                self.quaternion.x,
                -self.quaternion.y,
                -self.quaternion.z,
                -self.quaternion.w,
                self.quaternion.y,
                self.quaternion.x,
                -self.quaternion.w,
                self.quaternion.z,
                self.quaternion.z,
                self.quaternion.w,
                self.quaternion.x,
                -self.quaternion.y,
                self.quaternion.w,
                -self.quaternion.z,
                self.quaternion.y,
                self.quaternion.x,
            ],
        };
        let vec_rotated = matrix.multiply_vec4(&Vector4::new(point.x, point.y, point.z, 1.));
        let point = Vector3::new(vec_rotated.x, vec_rotated.y, vec_rotated.z);
        // Translate
        point + self.position
    }
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
                quaternion: Vector4::new(1., 0., 0., 0.),
            },
            color: Color(255, 255, 255),
        }
    }

    pub fn draw(&self, renderer: &mut Renderer, world: &World) {
        for face in &self.faces {
            /*
            TODO: incorporate transformations
            */
            let shading = self
                .transformation
                .transformed(&face.3)
                .cos_similarity(&world.light.direction)
                * world.light.intensity
                + world.ambient;
            let color = self.color * (1. - shading);
            let a = world.camera.to_ndc(
                &world
                    .camera
                    .project_point(&self.vertices[face.0 as usize], &self.transformation),
            );
            let b = world.camera.to_ndc(
                &world
                    .camera
                    .project_point(&self.vertices[face.1 as usize], &self.transformation),
            );
            let c = world.camera.to_ndc(
                &world
                    .camera
                    .project_point(&self.vertices[face.2 as usize], &self.transformation),
            );
            if (b - a).x * (c - a).y - (c - a).x * (b - a).y > 0. {
                renderer.draw_triangle(&a, &b, &c, &color);
            }
        }
    }
}
