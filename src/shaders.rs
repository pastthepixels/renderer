use crate::{
    graphics::Color,
    math::{Vector2, Vector3},
    world::World,
};

use image::{flat::Error, io::Reader as ImageReader};

// Generic trait for all shaders.
pub trait Shader {
    fn fragment(&self, barycentric: &Vector3, uva: &Vector2, uvb: &Vector2, uvc: &Vector2)
        -> Color;
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
    fn fragment(
        &self,
        _barycentric: &Vector3,
        _uva: &Vector2,
        _uvb: &Vector2,
        _uvc: &Vector2,
    ) -> Color {
        self.color
    }
}

impl StandardShader {
    pub fn new(color: Color) -> StandardShader {
        StandardShader { color }
    }
}

// "Texture material" -- shading, textures
pub struct TextureShader {
    image: Vec<Color>,
    width: f32,
    height: f32,
}

impl Shader for TextureShader {
    fn fragment(
        &self,
        barycentric: &Vector3,
        uva: &Vector2,
        uvb: &Vector2,
        uvc: &Vector2,
    ) -> Color {
        // "coords" are the barycentric coordinates of the current pixel
        // "at", "bt", "ct" are the texture coordinates of the corners of the current triangle
        let uvx = barycentric.x * uva.x + barycentric.y * uvb.x + barycentric.z * uvc.x;
        let uvy = barycentric.x * uva.y + barycentric.y * uvb.y + barycentric.z * uvc.y;

        // convert to texture space
        let tx = (uvx * self.width) as usize;
        let ty = (uvy * self.height) as usize;

        // grab the corresponding pixel color on the texture
        self.image
            [(tx + (ty * self.width as usize)).min(self.width as usize * self.height as usize - 1)]
    }
}

impl TextureShader {
    pub fn new(image: Vec<Color>, width: f32, height: f32) -> TextureShader {
        TextureShader {
            image,
            width,
            height,
        }
    }
    pub fn image2vec(path: &str) -> Result<Vec<Color>, std::io::Error> {
        let image = ImageReader::open(path)?.decode();
        let mut vector = Vec::new();
        if image.is_ok() {
            for pixel in image.unwrap().to_rgb8().pixels() {
                vector.push(Color(pixel.0[0], pixel.0[1], pixel.0[2]));
            }
        }
        Ok(vector)
    }
}

// Wireframe shader -- no shading, colors but wireframe
pub struct WireframeShader {
    pub color: Color,
    pub thickness: f32,
}

impl Shader for WireframeShader {
    fn fragment(
        &self,
        barycentric: &Vector3,
        _uva: &Vector2,
        _uvb: &Vector2,
        _uvc: &Vector2,
    ) -> Color {
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
