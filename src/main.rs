mod camera;
mod graphics;
mod loader;
mod math;
mod mesh;
mod shaders;
mod world;

pub fn main() {
    let file_path = "models/quad_damage/quad_damage.obj";

    let mut renderer = graphics::Renderer::new(
        &format!("u tell me a spike rasterised this {}", file_path),
        800,
        600,
    );

    let mut world = world::World {
        camera: camera::PerspectiveCamera::new(&math::Vector3::new(0., 0., -20.), 800., 600.),
        light: world::DirectionalLight::new(&math::Vector3::new(0., -1., 0.), 0.5),
        ambient: 0.2,
    };

    world.camera.generate_projection_matrix();

    let mut mesh_loaded = loader::load(file_path);

    let shader = shaders::StandardShader::new(graphics::Color(72, 82, 118));
    mesh_loaded.shader = Box::new(shader);

    let mut keyframe = 0.;

    mesh_loaded.transformation.quaternion.y = std::f32::consts::FRAC_1_SQRT_2;
    mesh_loaded.transformation.quaternion.w = std::f32::consts::FRAC_1_SQRT_2;

    while renderer.is_running() {
        renderer.clear();

        if keyframe >= 2. * std::f32::consts::PI {
            keyframe = 0.;
        } else {
            keyframe += 0.01 * std::f32::consts::PI;
        }

        world.camera.position.y = &keyframe.sin() * 0.2;

        mesh_loaded.transformation.quaternion.y = (keyframe / 2.).sin();
        mesh_loaded.transformation.quaternion.w = (keyframe / 2.).cos();
        mesh_loaded.transformation.quaternion = mesh_loaded.transformation.quaternion.normalised();
        mesh_loaded.transformation.generate_affine_matrix();

        mesh_loaded.draw(&mut renderer, &world);
        renderer.update();
        renderer.auto_resize(&mut world.camera);
    }
}
