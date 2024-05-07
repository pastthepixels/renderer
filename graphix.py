from functools import cmp_to_key
from math import ceil, floor, pi, sin
from time import sleep, time

import numpy
import pygame
from pygame import gfxdraw, surfarray

from algebra import Vector2, Vector3
from camera import PerspectiveCamera
from loader import OBJLoader

# Constants
# =========
DRAW_BASIS = False


# Maths
# =====


class Color(Vector3):
    def getRed(self):
        return int(self.x * 255)

    def getGreen(self):
        return int(self.y * 255)

    def getBlue(self):
        return int(self.z * 255)

    def __mul__(self, o):
        return Color(self.x * o, self.y * o, self.z * o)

    def toTuple(self) -> tuple:
        return (self.getRed(), self.getGreen(), self.getBlue())


class Face:
    def __init__(
        self, indexA: int, indexB: int, indexC: int, normal: Vector3 = Vector3(0, 0, 0)
    ):
        self.a = indexA
        self.b = indexB
        self.c = indexC
        self.set_normal(normal)

    def __getitem__(self, index: int):
        match index:
            case 0:
                return self.a
            case 1:
                return self.b
            case 2:
                return self.c

    def set_normal(self, vec: Vector3):
        self.normal = vec

    def get_normal(self):
        return self.normal


class Mesh:
    def __init__(
        self,
        vertices: list,
        faces: list,
        face_normals: list = [],
        origin: Vector3 = Vector3(0, 0, 0),
        scale: float = 1,
        color: Color = Color(0, 0, 0),
    ):
        """
        Creates a Mesh.

        vertices: List of vertices in tuples; ex. [(0, 0, 0), (1, 1, 1)]
        faces:    List of faces where each item is an iterable containing indexes of vectors.
        """
        self.vertices = vertices
        self.scale = scale
        self.origin = origin
        self.faces = self.tuple_to_Faces(
            faces,
            face_normals if face_normals != [] else self.compute_face_normals(faces),
        )
        self.color = color
        self.verts_projected = []

    def tuple_to_Faces(self, face_list: list, normals_list: list):
        faces = []
        for face in face_list:
            faces.append(
                Face(face[0], face[1], face[2], normals_list[face_list.index(face)])
            )
        return faces

    def compute_face_normals(self, faces) -> list:
        normals = []
        for face in faces:
            normal = (
                (self.get_point(face[0]) - self.get_point(face[1]))
                .cross_product(self.get_point(face[1]) - self.get_point(face[2]))
                .normalise()
            )
            normals.append(normal)
        return normals

    def get_point(self, index) -> Vector3:
        return (self.vertices[index] * self.scale) + self.origin

    def draw(self, renderer):
        self.verts_projected = [
            camera.project_point(self.get_point(i))
            for i in range(0, len(self.vertices))
        ]
        self.faces.sort(key=cmp_to_key(self.face_dist_cmp))
        for face in range(len(self.faces)):
            # backface culling
            a = self.verts_projected[self.faces[face][0]]
            b = self.verts_projected[self.faces[face][1]]
            c = self.verts_projected[self.faces[face][2]]
            if (b - a).x * (c - a).y - (c - a).x * (b - a).y > 0:
                continue
            # drawing
            # get color based on light
            shading = min(
                max(
                    self.faces[face].get_normal().cos_similarity(directional_light * -1)
                    * directional_light_intensity
                    + ambient_light_intensity,
                    0,
                ),
                1,
            )

            # draw function
            renderer.draw_triangle_barycentric(
                camera.to_NDC(self.verts_projected[self.faces[face][0]]),
                camera.to_NDC(self.verts_projected[self.faces[face][1]]),
                camera.to_NDC(self.verts_projected[self.faces[face][2]]),
                a_depth=self.verts_projected[self.faces[face][0]].z,
                b_depth=self.verts_projected[self.faces[face][1]].z,
                c_depth=self.verts_projected[self.faces[face][2]].z,
                fill=self.color * shading,
            )

    def face_dist_cmp(self, a, b):
        """
        Sorts faces s.t. FRONT TO BACK (we are rendering just like Quake).
        To render BACK TO FRONT (like a normal person), swap a_depth and b_depth in the return.
        """
        a_depth = (
            abs(self.verts_projected[a[0]].z)
            + abs(self.verts_projected[a[1]].z)
            + abs(self.verts_projected[a[2]].z)
        )
        b_depth = (
            abs(self.verts_projected[b[0]].z)
            + abs(self.verts_projected[b[1]].z)
            + abs(self.verts_projected[b[2]].z)
        )
        return a_depth - b_depth  #


meshes = []

loader = OBJLoader("/var/home/pastthepixels/Documents/graphix/models/teapot.obj")

meshes.append(
    Mesh(
        loader.get_vertices(),
        loader.get_faces(),
        face_normals=loader.get_surface_normals(),
        origin=Vector3(0, 0, 0),
        scale=0.3,
        color=Color(0.5, 0.35, 0.5),
    )
)

# FIXME watch farZ
camera = PerspectiveCamera(
    Vector3(0, 0, 0), Vector3(0, -1, 0), winsize=Vector2(256, 256), farZ=2.5
)

directional_light = Vector3(0, -1, 1).normalise()
directional_light_intensity = 0.8
ambient_light_intensity = 0.5

# Drawing
# =======


class Renderer:
    def __init__(
        self,
        window,
        clear_color: Color = Color(0, 0, 0),
        size: Vector2 = Vector2(800, 600),
    ):
        self.clear_color = clear_color
        self.window = window
        self.size = size
        self.clear()

    def clear(self):
        self.window.fill((0, 0, 0))
        self.pixel_array = pygame.PixelArray(self.window)

    def update(self, rect=None):
        self.window.lock()
        pygame.display.update(rect)

    def set_pixel(self, x: int, y: int, color: Color):
        self.pixel_array[x, y] = color.toTuple()

    def draw_line(self, a: Vector2, b: Vector2, fill: Color):
        gfxdraw.line(
            self.window,
            int(a.x),
            int(a.y),
            int(b.x),
            int(b.y),
            (fill.getRed(), fill.getGreen(), fill.getBlue()),
        )

    def draw_triangle(self, a: Vector2, b: Vector2, c: Vector2, fill: Color):
        gfxdraw.filled_trigon(
            self.window,
            int(a.x),
            int(a.y),
            int(b.x),
            int(b.y),
            int(c.x),
            int(c.y),
            (fill.getRed(), fill.getGreen(), fill.getBlue()),
        )

    def draw_triangle_barycentric(
        self,
        a: Vector2,
        b: Vector2,
        c: Vector2,
        fill: Color,
        a_depth=0,
        b_depth=0,
        c_depth=0,
    ):
        max_x = max(a.x, max(b.x, c.x))
        max_y = max(a.y, max(b.y, c.y))
        min_x = min(a.x, min(b.x, c.x))
        min_y = min(a.y, min(b.y, c.y))

        max_x = floor(min(max_x, self.size.x - 1))
        max_y = ceil(min(max_y, self.size.y - 1))
        min_x = floor(max(min_x, 0))
        min_y = ceil(max(min_y, 0))

        top_left = self.get_barycentric_coords(a, b, c, Vector2(min_x, min_y))
        delta_y = (
            self.get_barycentric_coords(a, b, c, Vector2(min_x, min_y + 1)) - top_left
        )
        delta_x = (
            self.get_barycentric_coords(a, b, c, Vector2(min_x + 1, min_y)) - top_left
        )

        for y in range(min_y, max_y + 1):
            coords_row = top_left + (delta_y * (y - min_y))
            for x in range(min_x, max_x + 1):
                pixel_color = self.pixel_array[x, y]
                if pixel_color == self.window.map_rgb((0, 0, 0)):
                    coords = coords_row + (delta_x * (x - min_x))
                    if (
                        coords.x >= 0
                        and coords.y >= 0
                        and coords.z >= 0
                        and coords.x + coords.y + coords.z >= 0.99
                    ):
                        depth = max(
                            min(
                                a_depth * coords.x
                                + b_depth * coords.y
                                + c_depth * coords.z,
                                1,
                            ),
                            0,
                        )
                        # fill = Color(coords.x, coords.y, coords.z)
                        self.set_pixel(x, y, fill * (1 - depth))
                        self.update(pygame.Rect(x - 1, y - 1, x, y))

    def get_barycentric_coords(self, a: Vector2, b: Vector2, c: Vector2, p: Vector2):
        vec0 = b - a
        vec1 = c - a
        vec2 = p - a

        d00 = vec0.dot_product(vec0)
        d01 = vec0.dot_product(vec1)
        d11 = vec1.dot_product(vec1)
        d20 = vec2.dot_product(vec0)
        d21 = vec2.dot_product(vec1)

        det = d00 * d11 - d01 * d01

        if det == 0:
            return Vector3(0, 0, 0)

        v = (d11 * d20 - d01 * d21) / det
        w = (d00 * d21 - d01 * d20) / det
        u = 1.0 - v - w
        return Vector3(u, v, w)


def main() -> int:
    global directional_light_intensity

    pygame.init()

    window = pygame.display.set_mode(
        (camera.size.x, camera.size.y),
        pygame.RESIZABLE | pygame.HWACCEL | pygame.PREALLOC | pygame.DOUBLEBUF,
    )

    renderer = Renderer(window, clear_color=Color(0, 0, 0), size=camera.size)

    pygame.display.set_caption("u tell me a spike this perspective graphics")

    start_time = time()
    running = True

    while running:
        # handle quit signal
        for event in pygame.event.get():
            if event.type == pygame.QUIT:
                running = False
            elif event.type == pygame.VIDEORESIZE:
                camera.set_size(event.w, event.h)
                renderer.size = camera.size
                camera.generate_projection_matrix()
                renderer.clear()

        # draw all
        origin = Vector3(
            sin((time() - start_time) * 0.5) * 0.7,
            sin((time() - start_time) * 0.7) * 0.7,
            (sin(time() - start_time) + 1) * 5,
        )
        # meshes[0].origin = origin

        # renderer.clear()
        for mesh in meshes:
            mesh.draw(renderer)

        if DRAW_BASIS:
            # Z (into the screen)
            renderer.draw_line(
                camera.to_NDC(camera.project_point(Vector3(0, 0, 0) + origin)),
                camera.to_NDC(camera.project_point(Vector3(0, 0, 1) + origin)),
                Color(0, 0, 0.8),
            )

            # X
            renderer.draw_line(
                camera.to_NDC(camera.project_point(Vector3(0, 0, 0) + origin)),
                camera.to_NDC(camera.project_point(Vector3(1, 0, 0) + origin)),
                Color(0.8, 0, 0),
            )

            # Y
            renderer.draw_line(
                camera.to_NDC(camera.project_point(Vector3(0, 0, 0) + origin)),
                camera.to_NDC(camera.project_point(Vector3(0, 1, 0) + origin)),
                Color(0, 0.8, 0),
            )

            renderer.draw_line(Vector2(5, 5), Vector2(5, 100), Color(0, 0, 0))
            renderer.draw_line(Vector2(5, 5), Vector2(100, 5), Color(0, 0, 0))

        renderer.update()

    return 0


exit(main())
