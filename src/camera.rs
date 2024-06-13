use crate::math::{Matrix44, Vector2, Vector3, Vector4};

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
                -1.,
                0.,
                0.,
                -self.z_far * self.z_near / (self.z_far - self.z_near),
                0.,
            ],
        }
    }

    pub fn project_point(&self, point: &Vector3) -> Vector3 {
        let projected = self.projection_matrix.multiply_vec4(&Vector4 {
            x: point.x + self.position.x,
            y: point.y + self.position.y,
            z: point.z + self.position.z,
            w: 1.,
        });
        let mut projected = projected.to_vector3();
        // TODO: fix Z so it is between near and far, to fix hack
        projected.z = (projected.z - self.z_near) / (self.z_far - self.z_near);
        projected
    }

    pub fn to_ndc(&self, projected: &Vector3) -> Vector3 {
        // FIXME: un-invert this, fix matrix44 multiplication.
        Vector3 {
            x: ((-projected.x + 1.) * self.size.x) / 2.,
            y: ((projected.y + 1.) * self.size.y) / 2.,
            z: projected.z, // x and y are 2d coords, z contains depth info
        }
    }
}
