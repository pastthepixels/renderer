use std::fs;

use crate::{
    math::Vector3,
    mesh::{Face, Mesh},
    shaders,
};

/// Loads an .obj file to a Mesh
pub fn load(file_path: &str) -> Mesh {
    let contents = fs::read_to_string(file_path).expect("Cannot open .obj");
    let mut vertices: Vec<Vector3> = Vec::new();
    let mut faces: Vec<Face> = Vec::new();

    for line in contents.split("\n") {
        if line.len() > 2 {
            let symbol = &line[..2];
            match symbol {
                "v " => {
                    let split = &line[2..].split(" ").collect::<Vec<_>>();
                    let vector = Vector3::new(
                        split[0].parse().unwrap(),
                        split[1].parse().unwrap(),
                        split[2].parse().unwrap(),
                    );
                    vertices.push(vector);

                    println!("v {} {} {}", split[0], split[1], split[2]);
                }
                "vn" => {
                    // TODO: vector normals
                }
                "f " => {
                    let split = &line[2..].split(" ").collect::<Vec<_>>();
                    let mut face_vertices: Vec<u32> = Vec::new();
                    for vertex in split {
                        let vertex_split = &vertex[0..].split("/").collect::<Vec<_>>();
                        face_vertices.push(vertex_split[0].parse::<u32>().unwrap() - 1);
                    }

                    let mut face = Face::new(face_vertices[0], face_vertices[1], face_vertices[2]);
                    face.compute_normal(&vertices);
                    faces.push(face);

                    println!("f {} {} {}", split[0], split[1], split[2]);
                }
                _ => {}
            }
        }
    }

    Mesh::new(vertices, faces)
}

/*
def __init__(self, file_path: str):
        self.file_path = file_path
        self.vertices = []          # vertices
        self.lines = []             # polylines
        self.faces = []             # faces
        self.normals = []           # vector normals
        self.surface_normals = []   # surfae normals (normals for each face)
        with open(self.file_path) as file:
            for file_line in file.readlines():
                if file_line != "":
                    args = file_line.split(" ")
                    match args[0]:
                        case "v":
                            self.vertices.append(Vector3(
                                float(args[1]),
                                float(args[2]),
                                float(args[3])
                            ))

                        case "vn":
                            self.normals.append(Vector3(
                                float(args[1]),
                                float(args[2]),
                                float(args[3])
                            ))

                        case "l":
                            line = []
                            for point in args[1:]:
                                line.append(int(point) - 1)
                            self.lines.append(line)

                        case "f":
                            face = []
                            normal = Vector3()
                            for point in args[1:]:
                                points = point.split("/")
                                face.append(int(points[0]) - 1)
                                if len(points) >= 3:
                                    normal = self.normals[int(points[2]) - 1]
                            self.surface_normals.append(normal)
                            self.faces.append(face)
*/
