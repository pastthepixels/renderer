// TODO replace with https://docs.rs/nalgebra/latest/nalgebra/

//
// 2D vectors
//
#[derive(Copy, Clone)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl std::fmt::Display for Vector2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vector2({}, {})", self.x, self.y)
    }
}

impl std::ops::Add<Vector2> for Vector2 {
    type Output = Vector2;

    fn add(self, rhs: Vector2) -> Self::Output {
        Vector2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Sub<Vector2> for Vector2 {
    type Output = Vector2;

    fn sub(self, rhs: Vector2) -> Self::Output {
        Vector2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl std::ops::Mul<f32> for Vector2 {
    type Output = Vector2;

    fn mul(self, rhs: f32) -> Self::Output {
        Vector2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Vector2 {
        Vector2 { x, y }
    }

    /// Returns the dot product of the current vector with another vector.
    pub fn dot_product(&self, other: &Vector2) -> f32 {
        self.x * other.x + self.y * other.y
    }
}

//
// 3D vectors
//

#[derive(Copy, Clone)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl std::fmt::Display for Vector3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vector3({}, {}, {})", self.x, self.y, self.z)
    }
}

impl std::ops::Add<Vector3> for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Vector3) -> Self::Output {
        Vector3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl std::ops::Sub<Vector3> for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Vector3) -> Self::Output {
        Vector3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl std::ops::Mul<f32> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: f32) -> Self::Output {
        Vector3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3 { x, y, z }
    }

    /// Returns the dot product of the current vector with another vector.
    pub fn dot_product(&self, other: &Vector3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Returns the cross product of the current vector with another vector.
    pub fn cross_product(&self, other: &Vector3) -> Vector3 {
        Vector3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    /// Returns the distance to another vector.
    pub fn distance_to(&self, other: &Vector3) -> f32 {
        f32::sqrt(
            f32::powi(self.x - other.x, 2)
                + f32::powi(self.y - other.y, 2)
                + f32::powi(self.z - other.z, 2),
        )
    }

    /// Returns the length of a vector.
    pub fn length(&self) -> f32 {
        f32::sqrt(self.x.powi(2) + self.y.powi(2) + self.z.powi(2))
    }

    /// Normalises a vector.
    pub fn normalised(&self) -> Vector3 {
        let inverse_length = 1. / self.length();
        Vector3 {
            x: self.x * inverse_length,
            y: self.y * inverse_length,
            z: self.z * inverse_length,
        }
    }

    /// Returns how much two vectors are similar (pointing in the same direction) using cosine
    /// similarity.
    pub fn cos_similarity(&self, other: &Vector3) -> f32 {
        self.dot_product(other) / (self.length() * other.length())
    }
}

//
// 4D vectors
//

#[derive(Copy, Clone)]
pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl std::fmt::Display for Vector4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vector4({}, {}, {}, {})", self.x, self.y, self.z, self.w)
    }
}

impl Vector4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Vector4 {
        Vector4 { x, y, z, w }
    }
    pub fn to_vector3(&self) -> Vector3 {
        if self.w == 0. {
            Vector3 {
                x: 0.,
                y: 0.,
                z: 0.,
            }
        } else {
            let inverse_w = 1. / self.w;
            Vector3 {
                x: self.x * inverse_w,
                y: self.y * inverse_w,
                z: self.z * inverse_w,
            }
        }
    }

    /// Returns the length of a vector.
    pub fn length(&self) -> f32 {
        f32::sqrt(self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.w.powi(2))
    }

    /// Normalises a vector.
    pub fn normalised(&self) -> Vector4 {
        let inverse_length = 1. / self.length();
        Vector4 {
            x: self.x * inverse_length,
            y: self.y * inverse_length,
            z: self.z * inverse_length,
            w: self.w * inverse_length,
        }
    }
}

//
// Matrices (4x4)

pub struct Matrix44 {
    pub data: Vec<f32>,
}

impl Matrix44 {
    /// Multiplies a 4x4 matrix by a Vector4.
    pub fn multiply_vec4(&self, other: &Vector4) -> Vector4 {
        Vector4 {
            x: self.get(0, 0) * other.x
                + self.get(0, 1) * other.y
                + self.get(0, 2) * other.z
                + self.get(0, 3) * other.w,
            y: self.get(1, 0) * other.x
                + self.get(1, 1) * other.y
                + self.get(1, 2) * other.z
                + self.get(1, 3) * other.w,
            z: self.get(2, 0) * other.x
                + self.get(2, 1) * other.y
                + self.get(2, 2) * other.z
                + self.get(2, 3) * other.w,
            w: self.get(3, 0) * other.x
                + self.get(3, 1) * other.y
                + self.get(3, 2) * other.z
                + self.get(3, 3) * other.w,
        }
    }

    /// Gets the i, j entry of a 4x4 matrix.
    pub fn get(&self, row: u32, col: u32) -> f32 {
        self.data[(row * 4 + col) as usize]
    }
}
