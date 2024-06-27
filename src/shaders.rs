use crate::{graphics::Color, math::Vector3, world::World};

// Generic trait for all shaders.
pub trait Shader {
    fn fragment(&self, barycentric: &Vector3) -> Color;
    // by default, calculates lighting but does not apply it in the fragment shader (unshaded)
    fn calculate_lighting(&mut self, normal: &Vector3, world: &World) -> f32 {
        1. - (normal.cos_similarity(&world.light.direction) * world.light.intensity + world.ambient)
    }
}

// "Standard material" -- shading, colors
pub struct StandardShader {
    pub color: Color,
}

impl Shader for StandardShader {
    fn fragment(&self, _barycentric: &Vector3) -> Color {
        self.color
    }
}

impl StandardShader {
    pub fn new(color: Color) -> StandardShader {
        StandardShader { color }
    }
}

// Wireframe shader -- no shading, colors but wireframe
pub struct WireframeShader {
    pub color: Color,
    pub thickness: f32,
}

impl Shader for WireframeShader {
    fn fragment(&self, barycentric: &Vector3) -> Color {
        if barycentric.x <= self.thickness
            || barycentric.y <= self.thickness
            || barycentric.z <= self.thickness
        {
            self.color
        } else {
            Color(0, 0, 0)
        }
    }
    fn calculate_lighting(&mut self, _normal: &Vector3, _world: &World) -> f32 {
        1.
    }
}

impl WireframeShader {
    pub fn new(color: Color, thickness: f32) -> WireframeShader {
        WireframeShader { color, thickness }
    }
}
