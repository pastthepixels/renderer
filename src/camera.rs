use crate::{
    math::{Matrix44, Vector2, Vector3, Vector4},
    mesh::Transformation,
};

pub struct PerspectiveCamera {
    pub position: Vector3,
    pub size: Vector2,
    pub z_near: f32,
    pub z_far: f32,
    pub aspect: f32,
    pub fov: f32,
    projection_matrix: Matrix44,
}

impl PerspectiveCamera {
    pub fn new(position: &Vector3, sizex: f32, sizey: f32) -> PerspectiveCamera {
        PerspectiveCamera {
            position: *position,
            size: Vector2::new(sizex, sizey),
            z_near: 0.1,
            z_far: 100.,
            aspect: sizex / sizey,
            fov: std::f32::consts::PI / 2.,
            projection_matrix: Matrix44 { data: vec![] },
        }
    }

    pub fn generate_projection_matrix(&mut self) {
        self.projection_matrix = Matrix44 {
            data: vec![
                1. / ((self.fov / 2.).tan() * self.aspect),
                0.,
                0.,
                0.,
                0.,
                1. / (self.fov / 2.).tan(),
                0.,
                0.,
                0.,
                0.,
                -self.z_far / (self.z_far - self.z_near),
                1.,
                0.,
                0.,
                -self.z_far * self.z_near / (self.z_far - self.z_near),
                0.,
            ],
        }
    }

    pub fn project_point(&self, point: &Vector3, transformation: &Transformation) -> Vector3 {
        let point = transformation.transformed(point) - self.position;
        let mut projected = self
            .projection_matrix
            .multiply_vec4(&Vector4 {
                x: point.x,
                y: point.y,
                z: point.z,
                w: 1.,
            })
            .to_vector3();
        if point.z < 0. {
            projected.z *= -1.;
        }
        projected
    }

    pub fn to_ndc(&self, mut projected: Vector3) -> Vector3 {
        projected.x = ((-projected.x + 1.) * self.size.x) / 2.;
        projected.y = ((projected.y + 1.) * self.size.y) / 2.;
        projected
    }
}
