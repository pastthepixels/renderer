mod camera;
mod graphics;
mod loader;
mod math;
mod mesh;

pub fn main() {
    let file_path = "models/teapot.obj";

    let mut renderer = graphics::Renderer::new(
        &format!("u tell me a spike rasterised this {}", file_path),
        800,
        600,
    );

    let mut camera = camera::PerspectiveCamera::new(&math::Vector3::new(0., 0., 30.), 800., 600.);
    camera.generate_projection_matrix();

    let mesh_loaded = loader::load(file_path);

    let mut keyframe = 0.;

    while renderer.is_running() {
        renderer.clear();

        mesh_loaded.draw(&mut renderer, &camera);

        if keyframe >= 2. * std::f32::consts::PI {
            keyframe = 0.;
        } else {
            keyframe += 0.005 * std::f32::consts::PI;
        }
        camera.position.y = &keyframe.sin() * 0.8;

        renderer.update();
    }
}
