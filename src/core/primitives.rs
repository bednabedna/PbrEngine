pub use super::defs::Real;

extern crate overload;
use overload::overload;
use rand::Rng;
use serde::Deserialize;
use std::ops;

fn min(a: Real, b: Real) -> Real {
    if a < b {
        a
    } else {
        b
    }
}

fn max(a: Real, b: Real) -> Real {
    if a > b {
        a
    } else {
        b
    }
}

pub type Point2R = Vec2R;
pub type Point3R = Vec3R;

pub type Normal2 = Vec2R;
pub type Normal3 = Vec3R;

#[derive(Copy, Clone, Debug, PartialEq, Deserialize)]
pub struct Vec2R {
    pub x: Real,
    pub y: Real,
}

impl Vec2R {
    pub const X: usize = 0;
    pub const Y: usize = 1;

    pub fn new(x: Real, y: Real) -> Vec2R {
        debug_assert!(!x.is_nan());
        debug_assert!(!y.is_nan());
        Vec2R { x, y }
    }

    pub fn with_z(&self, z: Real) -> Vec3R {
        debug_assert!(!z.is_nan());
        Vec3R {
            x: self.x,
            y: self.y,
            z,
        }
    }

    pub fn length(&self) -> Real {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> Real {
        self.x * self.x + self.y * self.y
    }

    pub fn dot(&self, other: &Vec2R) -> Real {
        self.x * other.x + self.y * other.y
    }

    pub fn normalize(&self) -> Vec2R {
        self / self.length()
    }

    pub fn min_component(&self) -> Real {
        min(self.x, self.y)
    }

    pub fn max_component(&self) -> Real {
        max(self.x, self.y)
    }

    pub fn max(&self, other: &Vec2R) -> Vec2R {
        Vec2R::new(max(self.x, other.x), max(self.y, other.y))
    }

    pub fn min(&self, other: &Vec2R) -> Vec2R {
        Vec2R::new(min(self.x, other.x), min(self.y, other.y))
    }

    pub fn swap(&self) -> Vec2R {
        Vec2R::new(self.y, self.x)
    }

    pub fn distance(&self, other: &Vec2R) -> Real {
        (other - self).length()
    }

    pub fn distance_squared(&self, other: &Vec2R) -> Real {
        (other - self).length_squared()
    }

    pub fn lerp(&self, other: &Vec2R, t: Real) -> Vec2R {
        (1.0 - t) * self + t * other
    }

    pub fn abs(&self) -> Vec2R {
        Vec2R::new(self.x.abs(), self.y.abs())
    }

    pub fn floor(&self) -> Vec2R {
        Vec2R::new(self.x.floor(), self.y.floor())
    }

    pub fn ceil(&self) -> Vec2R {
        Vec2R::new(self.x.ceil(), self.y.ceil())
    }
}

impl Default for Vec2R {
    fn default() -> Self {
        Vec2R::new(0.0, 0.0)
    }
}

impl From<Vec3R> for Vec2R {
    fn from(src: Vec3R) -> Self {
        Vec2R::new(src.x, src.y)
    }
}

overload!((a: ?Vec2R) + (b: ?Vec2R) -> Vec2R { Vec2R::new(a.x + b.x, a.y + b.y) });
overload!((a: ?Vec2R) - (b: ?Vec2R) -> Vec2R { Vec2R::new(a.x - b.x, a.y - b.y) });
overload!((a: ?Vec2R) * (b: ?Vec2R) -> Vec2R { Vec2R::new(a.x * b.x, a.y * b.y) });
overload!((a: ?Vec2R) / (b: ?Vec2R) -> Vec2R { Vec2R::new(a.x / b.x, a.y / b.y) });

overload!((a: &mut Vec2R) += (b: ?Vec2R) { a.x += b.x; a.y += b.y; });
overload!((a: &mut Vec2R) -= (b: ?Vec2R) { a.x -= b.x; a.y -= b.y; });
overload!((a: &mut Vec2R) *= (b: ?Vec2R) { a.x *= b.x; a.y *= b.y; });
overload!((a: &mut Vec2R) /= (b: ?Vec2R) { a.x /= b.x; a.y /= b.y; });

overload!((a: ?Vec2R) * (b: ?Real) -> Vec2R { Vec2R::new(a.x * b, a.y * b) });
overload!((a: ?Vec2R) / (b: ?Real) -> Vec2R { Vec2R::new(a.x / b, a.y / b) });
overload!((a: &mut Vec2R) *= (b: ?Real) { a.x *= b; a.y *= b; });
overload!((a: &mut Vec2R) /= (b: ?Real) { a.x /= b; a.y /= b; });

overload!((a: ?Real) * (b: ?Vec2R) -> Vec2R { Vec2R::new(a * b.x, a * b.y) });

overload!(- (a: & Vec2R) -> Vec2R { Vec2R::new(-a.x, -a.y) });

impl ops::Index<usize> for Vec2R {
    type Output = Real;
    fn index<'a>(&'a self, i: usize) -> &'a Real {
        if i == 0 {
            &self.x
        } else {
            debug_assert!(i == 1);
            &self.y
        }
    }
}

#[test]
fn vec2r_test() {
    let v1 = Vec2R::new(2.0, 3.0);
    assert_eq!(v1.x, 2.0, "x should be 2.0");
    assert_eq!(v1.y, 3.0, "y should be 3.0");
    assert_eq!(v1 * 2.0, Vec2R::new(4.0, 6.0), "v1 (left) should be scaled");
    assert_eq!(
        2.0 * v1,
        Vec2R::new(4.0, 6.0),
        "v1 (right) should be scaled up"
    );
    assert_eq!(v1 / 2.0, Vec2R::new(1.0, 1.5), "v1 should be scaled down");
    let v2 = Vec2R::new(5.0, 1.0);
    assert_eq!(v1 + v2, Vec2R::new(7.0, 4.0), "v1 + v2");
    assert_eq!(v1 - v2, Vec2R::new(-3.0, 2.0), "v1 - v2");
    assert_eq!(v1 * v2, Vec2R::new(10.0, 3.0), "v1 * v2");
    assert_eq!(v1 / v2, Vec2R::new(0.4, 3.0), "v1 / v2");

    assert_eq!(v1.dot(&v2), 13.0, "v1.dot(v2)");

    assert_eq!(v1.length_squared(), 13.0, "length squared");
    assert_eq!(Vec2R::new(3.0, 4.0).length(), 5.0, "length of (3, 4)");

    let delta = Vec2R::new(5.0, 7.0).normalize() - Vec2R::new(0.581, 0.814);
    assert!(delta.min_component() < 0.001, "normalize");

    assert_eq!(
        Vec2R::new(5.0, 7.0).min(&Vec2R::new(6.0, 2.0)),
        Vec2R::new(5.0, 2.0),
        "min between vecs"
    );
    assert_eq!(
        Vec2R::new(5.0, 7.0).max(&Vec2R::new(6.0, 2.0)),
        Vec2R::new(6.0, 7.0),
        "max between vecs"
    );

    assert_eq!(Vec2R::new(5.0, 7.0).max_component(), 7.0, "max component");
    assert_eq!(Vec2R::new(5.0, 7.0).min_component(), 5.0, "min component");

    assert_eq!(Vec2R::new(5.0, 7.0).swap(), Vec2R::new(7.0, 5.0), "swap");

    let v1 = Vec2R::new(3.0, 2.0);
    let v2 = Vec2R::new(7.0, 8.0);

    assert!((v1.distance(&v2) - 7.21).abs() < 0.01, "distance epsilon");
    assert_eq!(v1.distance_squared(&v2), 52.0, "distance squared");

    assert_eq!(v2.abs(), v2, "abs (self)");
    assert_eq!(Vec2R::new(-7.0, -8.0).abs(), v2, "abs (other)");

    assert_eq!(
        Vec2R::new(8.7, -5.0).floor(),
        Vec2R::new(8.0, -5.0),
        "floor 1"
    );
    assert_eq!(
        Vec2R::new(8.7, -5.1).floor(),
        Vec2R::new(8.0, -6.0),
        "floor 2"
    );
    assert_eq!(
        Vec2R::new(8.7, -5.0).ceil(),
        Vec2R::new(9.0, -5.0),
        "ceil 1"
    );
    assert_eq!(
        Vec2R::new(8.7, -5.1).ceil(),
        Vec2R::new(9.0, -5.0),
        "ceil 2"
    );
}

#[derive(Copy, Clone, Debug, PartialEq, Deserialize)]
pub struct Vec3R {
    pub x: Real,
    pub y: Real,
    pub z: Real,
}

impl Vec3R {
    pub const X: usize = 0;
    pub const Y: usize = 1;
    pub const Z: usize = 2;

    pub fn new(x: Real, y: Real, z: Real) -> Vec3R {
        debug_assert!(!x.is_nan());
        debug_assert!(!y.is_nan());
        debug_assert!(!z.is_nan());
        Vec3R { x, y, z }
    }

    pub fn length(&self) -> Real {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> Real {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn dot(&self, other: &Vec3R) -> Real {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Vec3R) -> Vec3R {
        Vec3R::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn normalize(&self) -> Vec3R {
        self / self.length()
    }

    pub fn unit(&self) -> Unit3R {
        Unit3R(self.normalize())
    }

    pub fn min_component(&self) -> Real {
        min(min(self.x, self.y), self.z)
    }

    pub fn max_component(&self) -> Real {
        max(max(self.x, self.y), self.z)
    }

    pub fn distance(&self, other: &Vec3R) -> Real {
        (self - other).length()
    }

    pub fn distance_squared(&self, other: &Vec3R) -> Real {
        (self - other).length_squared()
    }

    pub fn max(&self, other: &Vec3R) -> Vec3R {
        Vec3R::new(
            max(self.x, other.x),
            max(self.y, other.y),
            max(self.z, other.z),
        )
    }

    pub fn min(&self, other: &Vec3R) -> Vec3R {
        Vec3R::new(
            min(self.x, other.x),
            min(self.y, other.y),
            min(self.z, other.z),
        )
    }

    pub fn permute(&self, x: usize, y: usize, z: usize) -> Vec3R {
        Vec3R::new(self[x], self[y], self[z])
    }

    pub fn lerp(&self, other: &Vec3R, t: Real) -> Vec3R {
        (1.0 - t) * self + t * other
    }

    pub fn abs(&self) -> Vec3R {
        Vec3R::new(self.x.abs(), self.y.abs(), self.z.abs())
    }

    pub fn floor(&self) -> Vec3R {
        Vec3R::new(self.x.floor(), self.y.floor(), self.z.floor())
    }

    pub fn ceil(&self) -> Vec3R {
        Vec3R::new(self.x.ceil(), self.y.ceil(), self.z.ceil())
    }
}

impl Default for Vec3R {
    fn default() -> Self {
        Vec3R::new(0.0, 0.0, 0.0)
    }
}

impl From<Vec2R> for Vec3R {
    fn from(src: Vec2R) -> Self {
        Vec3R::new(src.x, src.y, 0.0)
    }
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Unit3R(Vec3R);

impl Unit3R {
    pub const UP: Unit3R = Unit3R(Vec3R {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    });
    /// Unit vector from  a Vector that is already normalized.
    /// The function renormalizes the vector if is not a unit.
    /// If the vector is often normalized this is slightly faster
    /// then normalizing it each time.
    pub fn normalized(src: Vec3R) -> Unit3R {
        if (src.length_squared() - 1.0).abs() > 100.0 * Real::EPSILON {
            src.unit()
        } else {
            Unit3R(src)
        }
    }
    pub fn random() -> Unit3R {
        let mut rng = rand::thread_rng();
        let a: Real = rng.gen_range(0.0, 2.0 * std::f64::consts::PI as Real);
        let z: Real = rng.gen_range(-1.0, 1.0);
        let r = (1.0 - z * z).sqrt();
        Unit3R(Vec3R::new(r * a.cos(), r * a.sin(), z))
    }
    pub fn vec(&self) -> &Vec3R {
        &self.0
    }
    pub fn x(&self) -> Real {
        self.0.x
    }
    pub fn y(&self) -> Real {
        self.0.y
    }
    pub fn z(&self) -> Real {
        self.0.z
    }
}

overload!(- (a: ? Unit3R) -> Unit3R { Unit3R(-a.vec()) });

#[test]
fn test_unit_vec() {
    let unit = Unit3R::normalized(Vec3R::new(10.0, -5.0, 0.3));
    assert!((unit.vec().length() - 1.0).abs() < 100.0 * Real::EPSILON);
    for _ in 0..1000 {
        assert!((Unit3R::random().vec().length_squared() - 1.0).abs() < 100.0 * Real::EPSILON);
    }
}

impl From<&Vec3R> for Unit3R {
    fn from(src: &Vec3R) -> Self {
        Unit3R(src.normalize())
    }
}

impl ops::Index<usize> for Vec3R {
    type Output = Real;
    fn index<'a>(&'a self, i: usize) -> &'a Real {
        if i == 0 {
            &self.x
        } else if i == 1 {
            &self.y
        } else {
            debug_assert!(i == 2);
            &self.z
        }
    }
}

overload!((a: ?Vec3R) + (b: ?Vec3R) -> Vec3R { Vec3R::new(a.x + b.x, a.y + b.y, a.z + b.z) });
overload!((a: ?Vec3R) - (b: ?Vec3R) -> Vec3R { Vec3R::new(a.x - b.x, a.y - b.y, a.z - b.z) });
overload!((a: ?Vec3R) * (b: ?Vec3R) -> Vec3R { Vec3R::new(a.x * b.x, a.y * b.y, a.z * b.z) });
overload!((a: ?Vec3R) / (b: ?Vec3R) -> Vec3R { Vec3R::new(a.x / b.x, a.y / b.y, a.z / b.z) });

overload!((a: &mut Vec3R) += (b: ?Vec3R) { a.x += b.x; a.y += b.y; a.z += b.z; });
overload!((a: &mut Vec3R) -= (b: ?Vec3R) { a.x -= b.x; a.y -= b.y; a.z -= b.z; });
overload!((a: &mut Vec3R) *= (b: ?Vec3R) { a.x *= b.x; a.y *= b.y; a.z *= b.z; });
overload!((a: &mut Vec3R) /= (b: ?Vec3R) { a.x /= b.x; a.y /= b.y; a.z /= b.z; });

overload!((a: ?Vec3R) * (b: ?Real) -> Vec3R { Vec3R::new(a.x * b, a.y * b, a.z*b) });
overload!((a: ?Vec3R) / (b: ?Real) -> Vec3R { Vec3R::new(a.x / b, a.y / b, a.z/b) });

overload!((a: &mut Vec3R) *= (b: ?Real) { a.x *= b; a.y *= b; a.z *= b; });
overload!((a: &mut Vec3R) /= (b: ?Real) { a.x /= b; a.y /= b; a.z /= b; });

overload!((a: ?Real) * (b: ?Vec3R) -> Vec3R { Vec3R::new(a * b.x, a * b.y, a * b.z) });

overload!(- (a: ?Vec3R) -> Vec3R { Vec3R::new(-a.x, -a.y, -a.z) });

#[test]
fn vec3r_test() {
    let v1 = Vec3R::new(2.0, 3.0, 4.0);
    assert_eq!(v1.x, 2.0, "x should be 2.0");
    assert_eq!(v1.y, 3.0, "y should be 3.0");
    assert_eq!(v1.z, 4.0, "z should be 4.0");
    assert_eq!(v1, Vec3R::new(2.0, 3.0, 4.0), "v1 sould be (2.0, 3.0, 4.0)");
    assert_eq!(
        v1 * 2.0,
        Vec3R::new(4.0, 6.0, 8.0),
        "v1 (left) should be scaled up"
    );
    assert_eq!(
        2.0 * v1,
        Vec3R::new(4.0, 6.0, 8.0),
        "v1 (right) should be scaled up"
    );
    assert_eq!(
        v1 / 2.0,
        Vec3R::new(1.0, 1.5, 2.0),
        "v1 should be scaled down"
    );
    let v2 = Vec3R::new(5.0, 1.0, 8.0);
    assert_ne!(v1, v2, "v1 souldn't be equal to v2");
    assert_eq!(v1 + v2, Vec3R::new(7.0, 4.0, 12.0), "v1 + v2");
    assert_eq!(v1 - v2, Vec3R::new(-3.0, 2.0, -4.0), "v1 - v2");
    assert_eq!(v1 * v2, Vec3R::new(10.0, 3.0, 32.0), "v1 * v2");
    assert_eq!(v1 / v2, Vec3R::new(0.4, 3.0, 0.5), "v1 / v2");

    assert_eq!(v1.dot(&v2), 45.0, "v1.dot(v2)");

    assert_eq!(v1.length_squared(), 29.0, "length squared");
    assert_eq!(
        Vec3R::new(3.0, 2.0, 6.0).length(),
        7.0,
        "length of (3, 2, 6)"
    );

    assert_eq!(
        Vec3R::new(1.0, 0.0, 0.0).cross(&Vec3R::new(0.0, 1.0, 0.0)),
        Vec3R::new(0.0, 0.0, 1.0),
        "v1.cross(v2)"
    );

    let delta = Vec3R::new(3.0, 1.0, 2.0).normalize() - Vec3R::new(0.802, 0.267, 0.534);
    assert!(delta.min_component() < 0.001, "normalize");

    assert_eq!(
        Vec3R::new(5.0, 7.0, 9.0).min(&Vec3R::new(6.0, 2.0, 1.0)),
        Vec3R::new(5.0, 2.0, 1.0),
        "min between vecs"
    );
    assert_eq!(
        Vec3R::new(5.0, 7.0, 9.0).max(&Vec3R::new(6.0, 2.0, 1.0)),
        Vec3R::new(6.0, 7.0, 9.0),
        "max between vecs"
    );

    assert_eq!(
        Vec3R::new(5.0, 7.0, 9.0).max_component(),
        9.0,
        "max component"
    );
    assert_eq!(
        Vec3R::new(5.0, 7.0, 9.0).min_component(),
        5.0,
        "min component"
    );

    assert_eq!(
        Vec3R::new(3.0, 5.0, 7.0).permute(Vec3R::Y, Vec3R::Z, Vec3R::X),
        Vec3R::new(5.0, 7.0, 3.0),
        "permute"
    );

    let v1 = Vec3R::new(8.0, -5.0, 0.0);
    let v2 = Vec3R::new(2.0, 3.0, 1.0);

    assert!((v1.distance(&v2) - 10.05).abs() < 0.01, "distance epsilon");
    assert_eq!(v1.distance_squared(&v2), 101.0, "distance squared");

    assert_eq!(v2.abs(), v2, "abs (self)");
    assert_eq!(Vec3R::new(-2.0, -3.0, -1.0).abs(), v2, "abs (other)");

    assert_eq!(
        Vec3R::new(8.7, -5.1, 2.0).floor(),
        Vec3R::new(8.0, -6.0, 2.0),
        "floor"
    );
    assert_eq!(
        Vec3R::new(8.7, -5.1, 2.0).ceil(),
        Vec3R::new(9.0, -5.0, 2.0),
        "ceil"
    );
}

#[derive(Debug)]
pub struct Ray {
    pub origin: Point3R,
    pub direction: Unit3R,
    pub color: Vec3R,
}

impl Ray {
    pub fn new(origin: Point3R, direction: Unit3R) -> Ray {
        // let's move the hit point so that, due to approx error, the new ray doesn't instersect the shape
        Ray {
            origin: origin,
            direction,
            color: Vec3R::new(1.0, 1.0, 1.0),
        }
    }
    pub fn with_color(origin: Point3R, direction: Unit3R, color: Vec3R) -> Ray {
        Ray {
            origin,
            direction,
            color,
        }
    }
    pub fn at(&self, time: Real) -> Point3R {
        self.origin + self.direction.vec() * time
    }
}

#[test]
fn ray_test() {
    let ray = Ray::new(
        Point3R::new(1.0, 7.0, 9.0),
        Vec3R::new(2.0, 3.0, 4.0).unit(),
    );
    assert_eq!(
        ray.at(2.0),
        ray.origin + ray.direction.vec() * 2.0,
        "position at 2"
    );
}

pub struct Hit {
    pub point: Point3R,
    pub normal: Unit3R,
    pub is_front_face: bool,
}
