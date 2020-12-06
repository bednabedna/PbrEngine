use super::defs::Real;
use super::primitives::*;
use serde::Deserialize;

pub trait Geometry {
    fn intersect(&self, ray: &Ray, t_min: Real, t_max: Real) -> Real;
    fn hit(&self, ray: &Ray, time: Real) -> Hit;
}

impl<'a> From<GeometryType> for Box<dyn Geometry + Send + Sync> {
    fn from(src: GeometryType) -> Box<dyn Geometry + Send + Sync> {
        match src {
            GeometryType::Sphere(geo) => Box::new(geo),
            GeometryType::Line(geo) => Box::new(geo),
            GeometryType::Cube(geo) => Box::new(geo),
        }
    }
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum GeometryType {
    Sphere(Sphere),
    Line(Line),
    Cube(Cube),
}

#[derive(Deserialize, Debug)]
pub struct Sphere {
    pub center: Point3R,
    pub radius: Real,
}

impl Sphere {
    pub fn new(center: Point3R, radius: Real) -> Sphere {
        Sphere { center, radius }
    }
}

/*
    Delta = Origin - Center
    t^2*(Direction.dot.Direction) + 2t*(Delta.dot.Direction) + Delta.dot.Delta - r^2 = 0
    a = Direction.dot.Direction
    b = 2*Delta.dot.Direction
    c = Delta.dot.Delta - r^2
    discriminant = bb - 4ac

    anziche  -b - sqrt(bb - 4ac) / 2a
    uso la forma ridotta -h - sqrt(hh - ac) / a
    con h = b/2

    posso semplificare a perche Direction e' nromalizzato
*/
impl Geometry for Sphere {
    fn intersect(&self, ray: &Ray, t_min: Real, t_max: Real) -> Real {
        let diff = ray.origin - self.center;
        let h = diff.dot(ray.direction.vec());
        let c = diff.dot(&diff) - self.radius * self.radius;
        let discriminant = h * h - c;
        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            let t = -h - root;
            if t < t_max && t > t_min {
                return t;
            }
            let t = -h + root;
            if t < t_max && t > t_min {
                return t;
            }
            return Real::INFINITY;
        } else {
            Real::INFINITY
        }
    }

    fn hit(&self, ray: &Ray, time: Real) -> Hit {
        let hit = ray.at(time);
        let outward_normal = (hit - self.center) / self.radius;
        let is_front_face = ray.direction.vec().dot(&outward_normal) <= 0.0;

        let normal = Unit3R::normalized(if is_front_face {
            outward_normal
        } else {
            -outward_normal
        });

        Hit {
            point: hit,
            normal,
            is_front_face,
        }
    }
}

#[test]
fn test_intersection() {
    let sphere_1 = Sphere::new(Point3R::new(0.0, 0.0, 2.0), 1.0);
    let sphere_2 = Sphere::new(Point3R::new(0.0, 0.0, 5.0), 1.0);
    let sphere_3 = Sphere::new(Point3R::new(0.0, 0.0, -5.0), 1.0);
    let ray = Ray::new(
        Point3R::new(0.0, 0.0, -1.0),
        Vec3R::new(0.0, 0.0, 1.0).unit(),
    );
    let t1 = sphere_1.intersect(&ray, 0.0, Real::INFINITY);
    let t2 = sphere_2.intersect(&ray, 0.0, Real::INFINITY);
    let t3 = sphere_3.intersect(&ray, 0.0, Real::INFINITY);
    assert_eq!(t1, 2.0);
    assert_eq!(t2, 5.0);
    assert_eq!(t3, Real::INFINITY);
}

// ---- CUBE ------

#[derive(Deserialize, Debug)]
pub struct Cube {
    pub origin: Point3R,
    pub corner: Point2R,
}

impl Cube {
    pub fn new(origin: Point3R, corner: Point2R) -> Cube {
        Cube { origin, corner }
    }
}

impl Geometry for Cube {
    fn intersect(&self, _ray: &Ray, _t_min: Real, _t_max: Real) -> Real {
        Real::INFINITY
    }

    fn hit(&self, ray: &Ray, _time: Real) -> Hit {
        Hit {
            point: ray.origin,
            normal: ray.direction,
            is_front_face: true,
        }
    }
}

// ---- LINE ------

#[derive(Deserialize, Debug)]
pub struct Line {
    pub start: Point3R,
    pub end: Point3R,
    pub width: Real,
}

impl Line {
    pub fn new(start: Point3R, end: Point3R, width: Real) -> Line {
        Line { start, end, width }
    }
}

impl Geometry for Line {
    fn intersect(&self, _ray: &Ray, _t_min: Real, _t_max: Real) -> Real {
        Real::INFINITY
    }

    fn hit(&self, ray: &Ray, _time: Real) -> Hit {
        Hit {
            point: ray.origin,
            normal: ray.direction,
            is_front_face: true,
        }
    }
}
