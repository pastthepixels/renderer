use crate::graphics::Color;
use crate::graphics::Renderer;
use crate::math::Vector3;

//
// Faces
//
pub struct Face {
    pub vertices: Vec<i32>,
    pub normal: Vector3,
}

impl Face {
    fn compute_normal(&self, vertices_list: &Vec<Vector3>) {
        todo!(); // TODO: implement
    }
}

//
// Transformations
//
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
    fn draw(&self, renderer: &Renderer) {
        todo!();
    }
}
