mod camera;
mod graphics;
mod loader;
mod math;
mod mesh;
mod world;

pub fn main() {
    let file_path = "models/teapot.obj";

    let mut renderer = graphics::Renderer::new(
        &format!("u tell me a spike rasterised this {}", file_path),
        800,
        600,
    );

    let mut world = world::World {
        camera: camera::PerspectiveCamera::new(&math::Vector3::new(0., 0., -50.), 800., 600.),
        light: world::DirectionalLight::new(&math::Vector3::new(0., -1., 0.), 0.5),
        ambient: 0.2,
    };

    world.camera.generate_projection_matrix();

    let mut mesh_loaded = loader::load(file_path);

    let mut keyframe = 0.;

    while renderer.is_running() {
        renderer.clear();

        mesh_loaded.draw(&mut renderer, &world);

        if keyframe >= 2. * std::f32::consts::PI {
            keyframe = 0.;
        } else {
            keyframe += 0.005 * std::f32::consts::PI;
        }
        //world.camera.position.y = &keyframe.sin() * 0.8;

        mesh_loaded.transformation.quaternion.w = &keyframe.sin() * 0.8;

        renderer.update();
    }
}
