use crate::float_eq;
use crate::point::Point;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug)]
pub struct Vector {
  pub x: f64,
  pub y: f64,
  pub z: f64,
  pub id: f64,
}

impl Vector {
  pub fn new(x: f64, y: f64, z: f64) -> Self {
    Vector { x, y, z, id: 0.0 }
  }

  pub fn mag(&self) -> f64 {
    (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
  }

  /// Normalizing a vector is calculating a unit vector from a given vector
  ///
  /// normalize computes a unit vector from Vector `self`
  ///
  /// # Example
  /// ```
  /// use speed_tracer::Vector;
  ///
  /// let v1 = Vector::new(4.0, 0.0, 0.0);
  /// assert_eq!(v1.normalize(), Vector::new(1.0, 0.0, 0.0));
  /// ```
  pub fn normalize(self) -> Vector {
    todo!();
  }
}

impl PartialEq for Vector {
  fn eq(&self, other: &Self) -> bool {
    float_eq(self.x, other.x) && float_eq(self.y, other.y) && float_eq(self.z, other.z)
  }
}

impl Add for Vector {
  type Output = Self;

  fn add(self, other: Self) -> Self {
    Self {
      x: self.x + other.x,
      y: self.y + other.y,
      z: self.z + other.z,
      id: 0.0,
    }
  }
}

impl Add<Point> for Vector {
  type Output = Point;

  fn add(self, other: Point) -> Point {
    Point {
      x: self.x + other.x,
      y: self.y + other.y,
      z: self.z + other.z,
      id: 1.0,
    }
  }
}

impl Sub for Vector {
  type Output = Self;

  fn sub(self, rhs: Self) -> Self {
    Self {
      x: self.x - rhs.x,
      y: self.y - rhs.y,
      z: self.z - rhs.z,
      id: 0.0,
    }
  }
}

impl Neg for Vector {
  type Output = Vector;

  fn neg(self) -> Self {
    Self {
      x: -self.x,
      y: -self.y,
      z: -self.z,
      id: 0.0,
    }
  }
}

impl Mul<f64> for Vector {
  type Output = Self;

  fn mul(self, rhs: f64) -> Self {
    Self {
      x: self.x * rhs,
      y: self.y * rhs,
      z: self.z * rhs,
      id: 0.0,
    }
  }
}

impl Mul<Vector> for f64 {
  type Output = Vector;

  fn mul(self, rhs: Vector) -> Vector {
    Vector::new(self * rhs.x, self * rhs.y, self * rhs.z)
  }
}

impl Div<f64> for Vector {
  type Output = Self;

  fn div(self, rhs: f64) -> Self {
    Self {
      x: self.x / rhs,
      y: self.y / rhs,
      z: self.z / rhs,
      id: 0.0,
    }
  }
}

impl Div<Vector> for f64 {
  type Output = Vector;

  fn div(self, rhs: Vector) -> Vector {
    Vector::new(self / rhs.x, self / rhs.y, self / rhs.z)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn vector_has_id_of_zero() {
    let vector = Vector::new(1.0, 4.1, 3.2);
    assert_eq!(vector.id, 0.0)
  }

  #[test]
  fn vector_substraction() {
    let v1 = Vector::new(2.7, 6.8, 1.4);
    let v2 = Vector::new(6.2, 2.9, 3.4);

    assert_eq!(v2 - v1, Vector::new(3.5, -3.9, 2.0))
  }

  #[test]
  fn zero_vector_subtraction() {
    let zero = Vector::new(0.0, 0.0, 0.0);
    let v1 = Vector::new(1.0, -2.0, 1.0);

    assert_eq!(zero - v1, Vector::new(-1.0, 2.0, -1.0))
  }

  #[test]
  fn vector_scalar_multiplication() {
    let v1 = Vector::new(2.0, -1.0, 3.0);
    assert_eq!(v1 * 3.5, Vector::new(7.0, -3.5, 10.5))
  }

  #[test]
  fn vector_scalar_division() {
    let v1 = Vector::new(2.0, -1.0, 3.0);
    assert_eq!(v1 / 2.0, Vector::new(1.0, -0.5, 1.5))
  }

  #[test]
  fn vector_magnitude() {
    // This is called unit vector (vector with mag of 1)
    let unit1 = Vector::new(1.0, 0.0, 0.0);
    let v1 = Vector::new(1.0, 2.0, 3.0);

    assert_eq!(unit1.mag(), 1.0);
    assert_eq!(v1.mag(), 14.0_f64.sqrt());
  }
}
