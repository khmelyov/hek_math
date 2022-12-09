use std::fmt;
use nalgebra::{Matrix1x4, RowVector, Vector4};
use crate::constants::*;

pub struct MumfordView {
    pub a_0: i128,
    pub a_1: i128,
    pub b_0: i128,
    pub b_1: i128,
}

impl MumfordView {
     pub const fn b0_square(&self) -> i128 { self.b_0 * self.b_0 }

     pub const fn new(a_0: i128, a_1: i128, b_0: i128, b_1: i128) -> Self {
        Self { a_0, a_1, b_0, b_1 }
    }

    pub fn to_projection (&self) -> ProjectionCoords {
        let x = A_ * (p.a_0 * (MU - p.a_0) * (LAMBDA + p.a_1 + NU) - p.b0_square());
        let y = B_ * (p.a_0 * (NU * LAMBDA - p.a_0) * (1 + p.a_1 + MU) - p.b0_square());
        let z = C_ * (p.a_0 * (MU - p.a_0) * (LAMBDA + p.a_1 + MU) - p.b0_square());
        let t = D_ * (p.a_0 * (MU - p.a_0) * (1 + p.a_1 + NU) - p.b0_square());
        ProjectionCoords { x, y, z, t, is_fast: true }
    }
}

impl fmt::Display for MumfordView {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "a_0: {}, a_1: {}, b_0: {}, b_1: {}", self.a_0, self.a_1, self.b_0, self.b_1)
    }
}

pub struct ProjectionCoords {
    pub x: i128,
    pub y: i128,
    pub z: i128,
    pub t: i128,
    is_fast: bool
}

impl ProjectionCoords {
    pub fn matrix(&self) -> Matrix1x4<i128> {
        Matrix1x4::new(self.x, self.y, self.z, self.t)
    }

    pub fn to_gen(&self) -> ProjectionCoords {
        let m_t: Matrix4<i128> = M.transpose();
        let result: Matrix1x4<i128> = p.matrix().mul(&m_t);
        let x: i128 = result[0];
        let y: i128 = result[1];
        let z: i128 = result[2];
        let t: i128 = result[3];
        ProjectionCoords { x, y, z, t, is_fast: false }
    }

    pub fn x_add(&self, other: ProjectionCoords) -> ProjectionCoords {
        if self.is_fast && other.is_fast {
            unimplemented!();
        } else unimplemented!()
    }
}

impl fmt::Display for ProjectionCoords {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}, z: {}, t: {}", self.x, self.y, self.z, self.t)
    }
}