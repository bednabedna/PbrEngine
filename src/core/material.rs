use super::primitives::*;
use rand::thread_rng;
use rand::Rng;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum DesMaterial {
    Diffuse(Diffuse),
    Metal(Metal),
    Dieletric(Dieletric),
}

impl<'a> From<DesMaterial> for Box<dyn Material + Send + Sync> {
    fn from(src: DesMaterial) -> Box<dyn Material + Send + Sync> {
        match src {
            DesMaterial::Diffuse(mat) => Box::new(mat),
            DesMaterial::Metal(mat) => Box::new(mat),
            DesMaterial::Dieletric(mat) => Box::new(mat),
        }
    }
}

pub trait Material {
    fn bounce(&self, ray: &Ray, hit: &Hit) -> Ray;
}

// ------- DIFFUSE -------

#[derive(Deserialize, Debug)]
pub struct Diffuse {
    pub albedo: Vec3R,
}

impl Diffuse {
    pub fn new(albedo: Vec3R) -> Diffuse {
        Diffuse { albedo }
    }
}

impl Material for Diffuse {
    fn bounce(&self, ray: &Ray, hit: &Hit) -> Ray {
        Ray::with_color(
            hit.point,
            (hit.normal.vec() + Unit3R::random().vec()).unit(),
            ray.color * self.albedo,
        )
    }
}

// ------- METAL -------

#[derive(Deserialize, Debug)]
pub struct Metal {
    pub albedo: Vec3R,
    #[serde(default)]
    pub fuzz: Real,
}

impl Metal {
    pub fn new(albedo: Vec3R, fuzz: Real) -> Metal {
        Metal {
            albedo,
            fuzz: fuzz.min(1.0),
        }
    }

    fn reflect(incoming: &Unit3R, normal: &Unit3R) -> Vec3R {
        incoming.vec() - 2.0 * incoming.vec().dot(normal.vec()) * normal.vec()
    }
}
impl Material for Metal {
    fn bounce(&self, ray: &Ray, hit: &Hit) -> Ray {
        let mut reflected = Metal::reflect(&ray.direction, &hit.normal);
        if self.fuzz > 0.0 {
            reflected += Unit3R::random().vec() * self.fuzz;
        }
        let reflected = reflected.unit();
        let color = if reflected.vec().dot(hit.normal.vec()) > 0.0 {
            ray.color * self.albedo
        } else {
            Vec3R::new(0.0, 0.0, 0.0)
        };
        Ray::with_color(hit.point, reflected, color)
    }
}

// ------- DIELETRIC -------
#[derive(Deserialize, Debug)]
pub struct Dieletric {
    pub albedo: Vec3R,
    pub refraction: Real,
}

impl Dieletric {
    pub fn new(albedo: Vec3R, refraction: Real) -> Dieletric {
        Dieletric { albedo, refraction }
    }
    fn refract(incoming: &Unit3R, normal: &Unit3R, etai_over_etat: Real, cos_theta: Real) -> Vec3R {
        let incoming = incoming.vec();
        let normal = normal.vec();
        let perpendicular = etai_over_etat * (incoming + cos_theta * normal);
        let parallel = -(((1.0 - perpendicular.length_squared()).abs()).sqrt()) * normal;
        perpendicular + parallel
    }
    /// schlick_approx
    fn reflection_probability(cosine: Real, refraction: Real) -> Real {
        let r0 = (1.0 - refraction) / (1.0 + refraction);
        let r02 = r0 * r0;
        r02 + (1.0 - r02) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dieletric {
    fn bounce(&self, ray: &Ray, hit: &Hit) -> Ray {
        let etai_over_etat = if hit.is_front_face {
            1.0 / self.refraction
        } else {
            self.refraction
        };
        let cos_theta = (-ray.direction.vec()).dot(hit.normal.vec());
        let cos_theta_min = cos_theta.min(1.0);
        let sin_theta = (1.0 - cos_theta_min * cos_theta_min).sqrt();
        if etai_over_etat * sin_theta > 1.0
            || Dieletric::reflection_probability(cos_theta_min, etai_over_etat)
                > thread_rng().gen_range(0.0, 1.0)
        {
            // reflect
            let reflected = Metal::reflect(&ray.direction, &hit.normal);
            Ray::with_color(hit.point, reflected.unit(), ray.color * self.albedo)
        } else {
            // refract
            let refracted =
                Dieletric::refract(&ray.direction, &hit.normal, etai_over_etat, cos_theta).unit();
            Ray::with_color(hit.point, refracted, ray.color * self.albedo)
        }
    }
}
