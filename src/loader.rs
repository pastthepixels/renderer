use std::fs;

use crate::{
    math::{Vector2, Vector3},
    mesh::{Face, Mesh},
    shaders,
};

/// Loads an .obj file to a Mesh
pub fn load(file_path: &str) -> Mesh {
    let contents = fs::read_to_string(file_path).expect("Cannot open .obj");
    let mut vertices: Vec<Vector3> = Vec::new();
    let mut uvs: Vec<Vector2> = Vec::new();
    let mut faces: Vec<Face> = Vec::new();

    for line in contents.split('\n') {
        if line.len() > 2 {
            let symbol = &line[..2];
            match symbol {
                "v " => {
                    let split = &line[2..].split(' ').collect::<Vec<_>>();
                    let vector = Vector3::new(
                        split[0].parse().unwrap(),
                        split[1].parse().unwrap(),
                        split[2].parse().unwrap(),
                    );
                    vertices.push(vector);

                    println!("v {} {} {}", split[0], split[1], split[2]);
                }
                "vt" => {
                    let split = &line[3..].split(' ').collect::<Vec<_>>();
                    // TODO: inverts uvy, bad idea?
                    let vector = Vector2::new(
                        split[0].parse().unwrap(),
                        1. - split[1].parse::<f32>().unwrap(),
                    );
                    uvs.push(vector);

                    println!("vt {} {}", split[0], split[1]);
                }
                "vn" => {
                    // TODO: vector normals
                }
                "f " => {
                    let split = &line[2..].split(' ').collect::<Vec<_>>();
                    let mut face_vertices: Vec<usize> = Vec::new();
                    let mut face_uvs: Vec<usize> = Vec::new();
                    for vertex in split {
                        let vertex_split = &vertex[0..].split('/').collect::<Vec<_>>();
                        face_vertices.push(vertex_split[0].parse::<usize>().unwrap() - 1);
                        face_uvs.push(vertex_split[1].parse::<usize>().unwrap() - 1);
                    }

                    let mut face = Face::new(
                        face_vertices[0],
                        face_vertices[1],
                        face_vertices[2],
                        face_uvs[0],
                        face_uvs[1],
                        face_uvs[2],
                    );
                    face.compute_normal(&vertices);
                    faces.push(face);

                    println!("f {} {} {}", split[0], split[1], split[2]);
                }
                _ => {}
            }
        }
    }

    Mesh::new(vertices, faces, uvs)
}
