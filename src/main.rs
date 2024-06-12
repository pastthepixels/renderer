use math::Vector3;

mod camera;
mod graphics;
mod loader;
mod math;
mod mesh;

pub fn main() {
    let mut renderer = graphics::Renderer::new("u tell me a spike rasterised this spike", 800, 600);

    let mut camera = camera::PerspectiveCamera::new(&math::Vector3::new(0., 0., 20.), 800., 600.);
    camera.generate_projection_matrix();

    let mesh = mesh::Mesh::new(
        vec![
            math::Vector3::new(-1., 1., -1.),
            math::Vector3::new(-1., 1., 1.),
            math::Vector3::new(1., 1., 1.),
            math::Vector3::new(1., 1., -1.),
            math::Vector3::new(-1., -1., -1.),
            math::Vector3::new(-1., -1., 1.),
            math::Vector3::new(1., -1., 1.),
            math::Vector3::new(1., -1., -1.),
        ],
        vec![
            mesh::Face::new(0, 1, 2),
            mesh::Face::new(2, 3, 0),
            mesh::Face::new(4, 5, 6),
            mesh::Face::new(6, 7, 4),
        ],
    );

    let mesh_loaded = loader::load("models/teapot.obj");

    while renderer.is_running() {
        renderer.clear();

        mesh_loaded.draw(&mut renderer, &camera);
        /*
        for vertex in &mesh_loaded.vertices {
            let vertex = &camera.to_ndc(&camera.project_point(vertex));
            renderer.draw_pixel_debug(vertex.x as i32, vertex.y as i32);
        }*/

        renderer.update();
    }
}
