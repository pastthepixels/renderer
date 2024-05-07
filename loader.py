from algebra import Vector3

"""
Loads an .obj file.

Vertices in each face is winded CCW

"""


class OBJLoader:
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

    def get_vertices(self) -> list:
        return self.vertices

    def get_faces(self) -> list:
        return self.faces

    def get_surface_normals(self) -> list:
        return self.surface_normals

    def get_lines(self) -> list:
        return self.lines
