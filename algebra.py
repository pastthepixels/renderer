from math import sqrt, floor

# Y is the up vector!!

"""
2d vector
"""


class Vector2:
    def __init__(self, x: float = 0, y: float = 0):
        """
        Constructor with float data types.
        """
        self.x = x
        self.y = y

    def __floor__(self):
        return Vector2(x=floor(self.x), y=floor(self.y))

    def dot_product(self, other):
        return self.x * other.x + self.y * other.y

    def distance_to(self, other) -> float:
        return sqrt((self.x - other.x) ** 2 + (self.y - other.y) ** 2)

    def length(self):
        return sqrt(self.x**2 + self.y**2)

    def __eq__(self, o):
        return self.x == o.x and self.y == o.y

    def __lt__(self, o):
        return self.x < o.x and self.y < o.y

    def __repr__(self):
        return f"Vector2({self.x}, {self.y}])"

    def __add__(self, o):
        return Vector2(x=self.x + o.x, y=self.y + o.y)

    def __sub__(self, o):
        return Vector2(x=self.x - o.x, y=self.y - o.y)

    def __mul__(self, o):
        """
        Scalar multiplication
        """
        return Vector2(x=self.x * o, y=self.y * o)


"""
3d vector
"""


class Vector3:
    def __init__(self, x: float = 0, y: float = 0, z: float = 0):
        self.x = x
        self.y = y
        self.z = z

    def dot_product(self, other) -> float:
        return self.x * other.x + self.y * other.y + self.z * other.z

    def distance_to(self, other) -> float:
        return sqrt(
            (self.x - other.x) ** 2 + (self.y - other.y) ** 2 + (self.z - other.z) ** 2
        )

    def length(self) -> float:
        return sqrt(self.x**2 + self.y**2 + self.z**2)

    def project_to_vector(self, vector):
        return vector * (self.dot_product(vector) / (vector.length() ** 2))

    def cross_product(self, vector):
        return Vector3(
            self.y * vector.z - self.z * vector.y,
            self.z * vector.x - self.x * vector.z,
            self.x * vector.y - self.y * vector.x,
        )

    def normalise(self):
        return self * (1 / self.length() if self.length() > 0 else 1)

    def cos_similarity(self, vector) -> float:
        return self.dot_product(vector) / (self.length() * vector.length())

    def __eq__(self, o):
        return self.x == o.x and self.y == o.y and self.z == o.z

    def __repr__(self):
        return f"Vector3({self.x}, {self.y}, {self.z})"

    def __add__(self, o):
        return Vector3(self.x + o.x, self.y + o.y, self.z + o.z)

    def __sub__(self, o):
        return Vector3(self.x - o.x, self.y - o.y, self.z - o.z)

    def __mul__(self, o):
        """
        Scalar multiplication
        """
        return Vector3(self.x * o, self.y * o, self.z * o)


"""
4d Vector
"""


class Vector4:
    def __init__(self, x: float, y: float, z: float, w: float):
        self.x = x
        self.y = y
        self.z = z
        self.w = w

    def to_vector3(self):
        return Vector3(
            self.x * (1 / self.w), self.y * (1 / self.w), self.z * (1 / self.w)
        )


"""
Matrix
"""


class Matrix44:
    def __init__(self, data: list):
        self.data = data

    def multiplyVec4(self, other: Vector4):
        return Vector4(
            self.data[0][0] * other.x
            + self.data[0][1] * other.y
            + self.data[0][2] * other.z
            + self.data[0][3] * other.w,
            self.data[1][0] * other.x
            + self.data[1][1] * other.y
            + self.data[1][2] * other.z
            + self.data[1][3] * other.w,
            self.data[2][0] * other.x
            + self.data[2][1] * other.y
            + self.data[2][2] * other.z
            + self.data[2][3] * other.w,
            self.data[3][0] * other.x
            + self.data[3][1] * other.y
            + self.data[3][2] * other.z
            + self.data[2][3] * other.w,
        )
