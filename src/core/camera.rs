use super::defs::*;
use super::primitives::*;
//use rand::Rng;

#[derive(Clone)]
pub struct Camera {
    pub origin: Point3R,
    lower_left_corner: Point3R,
    horizontal: Vec3R,
    vertical: Vec3R,
    direction: Unit3R,
    aspect_ratio: Real,
    pub fov_radians: Real,
    rotation: Vec2R,
    //pub lens_radius: Real,
}

impl Camera {
    pub fn new(aspect_ratio: Real, fov_radians: Real) -> Camera {
        let focal_length = 1.0;

        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        // camera

        let origin = Point3R::default(); // 0,0,0
        let horizontal = Point3R::new(viewport_width, 0.0, 0.0);
        let vertical = Point3R::new(0.0, viewport_height, 0.0);
        let lower_left_corner =
            origin - 0.5 * horizontal - 0.5 * vertical - Vec3R::new(0.0, 0.0, focal_length);
        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            direction: Unit3R::UP,
            fov_radians,
            rotation: Vec2R::default(),
            aspect_ratio,
            //lens_radius: 0.0,
        }
    }

    pub fn set_aspect_ratio(&mut self, width: usize, height: usize) {
        self.aspect_ratio = width as Real / height as Real;
    }

    pub fn look_at(&mut self, target: Vec3R) {
        self.direction = target.unit();
        self.rotation.x = self.direction.vec().y.asin();
        self.rotation.y = (self.direction.vec().x / self.rotation.x.cos()).asin();
        self.update_viewport();
    }

    fn clamp_angle(angle: Real) -> Real {
        if angle < 0.0 {
            2.0 * PI - angle
        } else if angle >= 2.0 * PI {
            angle - 2.0 * PI
        } else {
            angle
        }
    }

    pub fn rotate(&mut self, rotation_rads: Vec2R) {
        self.rotation.x = Camera::clamp_angle(self.rotation.x + rotation_rads.x);
        self.rotation.y = Camera::clamp_angle(self.rotation.y + rotation_rads.y);
        let b = self.rotation.x.cos();
        self.direction = Unit3R::normalized(Vec3R::new(
            self.rotation.y.sin() * b,
            self.rotation.x.sin(),
            -self.rotation.y.cos() * b,
        ));
    }

    pub fn update_viewport(&mut self) {
        let h = (self.fov_radians * 0.5).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = self.aspect_ratio * viewport_height;

        let vup = Unit3R::UP;
        let u = vup.vec().cross(self.direction.vec()).unit();
        let v = self.direction.vec().cross(u.vec());

        self.horizontal = viewport_width * u.vec();
        self.vertical = viewport_height * v;
        self.lower_left_corner =
            self.origin - self.horizontal * 0.5 - self.vertical * 0.5 - self.direction.vec();
    }

    pub fn move_forward(&mut self, distance: Real) {
        self.move_backward(-distance);
    }

    pub fn move_right(&mut self, distance: Real) {
        let movement = self.horizontal.normalize() * distance;
        self.origin += movement;
        self.lower_left_corner += movement;
    }

    pub fn move_left(&mut self, distance: Real) {
        self.move_right(-distance);
    }

    pub fn move_backward(&mut self, distance: Real) {
        let movement = self.direction.vec() * distance;
        self.origin += movement;
        self.lower_left_corner += movement;
    }

    pub fn move_down(&mut self, distance: Real) {
        self.move_up(-distance);
    }

    pub fn move_up(&mut self, distance: Real) {
        let vup = Unit3R::UP;
        let movement = vup.vec() * distance;
        self.origin += movement;
        self.lower_left_corner += movement;
    }

    /*fn random_in_unit_disk() -> Vec3R {
        loop {
            let mut rng = rand::thread_rng();
            let p = Vec3R::new(rng.gen_range(-1.0, 1.0), rng.gen_range(-1.0, 1.0), 0.0);
            if p.length_squared() < 1.0 {
                break p;
            }
        }
    }*/

    pub fn ray_at(&self, x: Real, y: Real) -> Ray {
        debug_assert!(x >= 0.0 && x <= 1.0);
        debug_assert!(y >= 0.0 && y <= 1.0);
        /*let mut h = self.horizontal;
        let mut v = self.vertical;
        if self.lens_radius != 0.0 {
            let distorsion = self.lens_radius * Camera::random_in_unit_disk();
            h *= distorsion;
            v *= distorsion;
        }*/
        let h = self.horizontal;
        let v = self.vertical;
        let ray_origin = self.lower_left_corner + h * x + v * y;
        Ray::new(self.origin, (ray_origin - self.origin).unit())
    }
}
