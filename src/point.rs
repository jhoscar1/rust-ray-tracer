use crate::float_eq;
use crate::vector::Vector;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug)]
pub struct Point {
  pub x: f64,
  pub y: f64,
  pub z: f64,
  pub id: f64,
}

impl Point {
  pub fn new(x: f64, y: f64, z: f64) -> Self {
    Point { x, y, z, id: 1.0 }
  }
}

impl PartialEq for Point {
  fn eq(&self, other: &Self) -> bool {
    float_eq(self.x, other.x)
      && float_eq(self.y, other.y)
      && float_eq(self.z, other.z)
      && float_eq(self.id, other.id)
  }
}

impl Add<Vector> for Point {
  type Output = Point;

  fn add(self, other: Vector) -> Point {
    Point::new(self.x + other.x, self.y + other.y, self.z + other.z)
  }
}

impl Sub for Point {
  type Output = Vector;

  fn sub(self, rhs: Self) -> Vector {
    Vector::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
  }
}

impl Sub<Vector> for Point {
  type Output = Point;

  fn sub(self, rhs: Vector) -> Point {
    Point::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
  }
}

impl Neg for Point {
  type Output = Self;

  fn neg(self) -> Self {
    Self {
      x: -self.x,
      y: -self.y,
      z: -self.z,
      id: 1.0,
    }
  }
}

impl Mul<f64> for Point {
  type Output = Self;

  fn mul(self, rhs: f64) -> Self {
    Self {
      x: self.x * rhs,
      y: self.y * rhs,
      z: self.z * rhs,
      id: 1.0,
    }
  }
}

impl Mul<Point> for f64 {
  type Output = Point;

  fn mul(self, rhs: Point) -> Point {
    Point::new(self * rhs.x, self * rhs.y, self * rhs.z)
  }
}

impl Div<f64> for Point {
  type Output = Self;

  fn div(self, rhs: f64) -> Self {
    Self {
      x: self.x / rhs,
      y: self.y / rhs,
      z: self.z / rhs,
      id: 1.0,
    }
  }
}

impl Div<Point> for f64 {
  type Output = Point;

  fn div(self, rhs: Point) -> Point {
    Point::new(self / rhs.x, self / rhs.y, self / rhs.z)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn point_has_id_of_one() {
    let point = Point::new(1.0, 4.1, 3.2);
    assert_eq!(point.id, 1.0)
  }

  #[test]
  fn point_equality() {
    let p1 = Point::new(1.1, 4.0, 3.0);
    let p2 = Point::new(1.1, 4.0, 3.0);
    let equality = p1 == p2;
    assert_eq!(equality, true);
    assert_eq!(p1, p2);
  }

  #[test]
  fn add_point_and_vector() {
    let p1 = Point::new(1.0, 2.1, 6.0);
    let v1 = Vector::new(1.0, 2.1, 6.0);
    let p2 = p1 + v1;
    assert_eq!(p2, Point::new(2.0, 4.2, 12.0))
  }

  #[test]
  fn point_subtraction() {
    let p1 = Point::new(1.1, 4.0, 3.0);
    let p2 = Point::new(2.1, 3.0, 6.0);
    let v1 = p2 - p1;

    assert_eq!(v1, Vector::new(1.0, -1.0, 3.0));
  }

  #[test]
  fn point_vector_subtraction() {
    let p1 = Point::new(2.6, 3.4, 7.1);
    let v1 = Vector::new(0.6, 5.1, 5.7);
    let p2 = p1 - v1;

    assert_eq!(p2, Point::new(2.0, -1.7, 1.4));
  }

  #[test]
  fn point_multiplication() {
    let p1 = Point::new(2.0, -3.0, 1.0);
    assert_eq!(p1 * 2.5, Point::new(5.0, -7.5, 2.5));
  }

  #[test]
  fn point_division() {
    let p1 = Point::new(2.0, -3.0, 1.0);
    assert_eq!(p1 / 2.0, Point::new(1.0, -1.5, 0.5))
  }
}
