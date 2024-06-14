use crate::{camera::PerspectiveCamera, math::Vector3};

pub struct World {
    pub camera: PerspectiveCamera,
    pub light: DirectionalLight,
    pub ambient: f32,
}

pub struct DirectionalLight {
    pub direction: Vector3,
    pub intensity: f32,
}

impl DirectionalLight {
    pub fn new(direction: &Vector3, intensity: f32) -> DirectionalLight {
        DirectionalLight {
            direction: *direction,
            intensity,
        }
    }
}
