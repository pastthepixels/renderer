use math::Vector3;

mod camera;
mod graphics;
mod math;
mod mesh;

pub fn main() {
    let mut renderer = graphics::Renderer::new("u tell me a spike rasterised this spike", 800, 600);

    let mut camera = camera::PerspectiveCamera::new(&math::Vector3::new(0., 0.5, 1.), 800., 600.);
    camera.generate_projection_matrix();

    let mesh = mesh::Mesh::new(
        vec![
            math::Vector3::new(-1., 0., 0.),
            math::Vector3::new(-1., -1., 0.),
            math::Vector3::new(0., -1., 0.),
            math::Vector3::new(0., 0., 10.),
        ],
        vec![mesh::Face::new(0, 1, 2), mesh::Face::new(2, 3, 0)],
    );

    while renderer.is_running() {
        renderer.clear();

        mesh.draw(&mut renderer, &camera);

        renderer.update();
    }
}
