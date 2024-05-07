from algebra import Vector2, Vector3, Vector4, Matrix44
from math import pi, tan


class PerspectiveCamera:
    def __init__(
        self,
        position: Vector3,
        direction: Vector3,
        winsize: Vector2,
        nearZ: float = 0.1,
        farZ: float = 100,
        aspect: int = 1,
        fov=pi / 2,
    ):
        self.position = position
        self.normal = direction
        self.zNear = nearZ
        self.zFar = farZ
        self.fov = fov
        self.size = winsize
        self.aspect = aspect
        self.generate_projection_matrix()

    def generate_projection_matrix(self):
        self.PROJ_MATRIX = Matrix44(
            [
                [1 / (tan(self.fov / 2) * self.aspect), 0, 0, 0],
                [0, 1 / (tan(self.fov / 2)), 0, 0],
                [0, 0, -self.zFar / (self.zFar - self.zNear), -1],
                [0, 0, -self.zFar * self.zNear / (self.zFar - self.zNear), 0],
            ]
        )

    def set_size(self, width: float, height: float):
        self.size.x = width
        self.size.y = height
        self.aspect = width / height

    def project_point(self, point: Vector3) -> Vector3:
        projected = self.PROJ_MATRIX.multiplyVec4(Vector4(point.x, point.y, point.z, 1))
        projected = projected.to_vector3()
        # TODO - fix Z so that it is between near and far, to fix hack
        projected.z = (projected.z - self.zNear) / float(self.zFar - self.zNear)
        return projected

    def to_NDC(self, projected: Vector3) -> Vector2:
        # FIXME this is inverted because maybe SDL??? idk why actually
        return Vector2(
            ((-projected.x + 1) * self.size.x) / 2,
            ((projected.y + 1) * self.size.y) / 2,
        )
