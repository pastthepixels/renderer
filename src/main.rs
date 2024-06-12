mod camera;
mod graphics;
mod math;
mod mesh;

pub fn main() {
    let mut renderer = graphics::Renderer::new("u tell me a spike rasterised this spike", 800, 600);
    renderer.run();
}
