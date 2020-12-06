use super::super::primitives::*;
use super::super::scene::*;
use super::renderer_buffer::*;
use rand::prelude::*;

fn background_color(ray: &Ray) -> Vec3R {
    let top_color = Vec3R::new(0.3, 0.5, 1.0);
    let bottom_color = Vec3R::new(1.0, 1.0, 1.0);
    let k = (ray.direction.vec().y + 1.0) * 0.5;
    let color = bottom_color.lerp(&top_color, k);
    debug_assert!(color.max_component() <= 1.0);
    ray.color * color
}

const MIN_HIT_DISTANCE: Real = 10000.0 * Real::EPSILON;

fn bounce_ray(ray: &Ray, scene: &Scene) -> Option<Ray> {
    let mut max_time = Real::INFINITY;
    let mut maybe_object = None;
    for object in scene.objects_iter() {
        let time = object.geometry.intersect(&ray, MIN_HIT_DISTANCE, max_time);
        if time < max_time {
            maybe_object = Some(object);
            max_time = time;
        }
    }
    if let Some(object) = maybe_object {
        let hit = object.geometry.hit(ray, max_time);
        Some(object.material.bounce(ray, &hit))
    } else {
        None
    }
}

fn ray_color(ray: &Ray, scene: &Scene, bounces: usize) -> Vec3R {
    debug_assert!(ray.color.max_component() <= 1.0);
    if bounces > scene.max_bounces {
        return Vec3R::default();
    }
    if let Some(bounced_ray) = bounce_ray(ray, scene) {
        if bounced_ray.color.max_component() < 0.1 / 256.0 {
            bounced_ray.color
        } else {
            ray_color(&bounced_ray, scene, bounces + 1)
        }
    } else {
        background_color(ray)
    }
}

fn debug_surfaces_bounce_ray(ray: &Ray, scene: &Scene) -> Option<Ray> {
    let mut max_time = Real::INFINITY;
    let mut maybe_object = None;
    for object in scene.objects_iter() {
        let time = object.geometry.intersect(&ray, MIN_HIT_DISTANCE, max_time);
        if time < max_time {
            maybe_object = Some(object);
            max_time = time;
        }
    }
    if let Some(object) = maybe_object {
        let hit = object.geometry.hit(ray, max_time);
        let mut bounced_ray = object.material.bounce(ray, &hit);
        if bounced_ray.color.max_component() > 1.0 / 256. {
            bounced_ray.color = if hit.is_front_face {
                Vec3R::new(0.0, 0.0, 1.0)
            } else {
                Vec3R::new(1.0, 0.0, 0.0)
            };
            Some(bounced_ray)
        } else {
            None
        }
    } else {
        None
    }
}

fn debug_surfaces(ray: &Ray, scene: &Scene, bounces: usize) -> Vec3R {
    debug_assert!(ray.color.max_component() <= 1.0);
    if bounces > scene.max_bounces {
        return ray.color;
    }
    if let Some(bounced_ray) = debug_surfaces_bounce_ray(ray, scene) {
        debug_surfaces(&bounced_ray, scene, bounces + 1)
    } else {
        Vec3R::new(0.0, 0.0, 0.0)
    }
}

pub fn render(scene: &Scene, buffer: &mut impl RendererBuffer) {
    //let rendering_start = std::time::Instant::now();
    let width = scene.width() as Real;
    let height = scene.height() as Real;

    let camera = &scene.camera;

    buffer.sample_pixels(|row_index, col_index| {
        let w = col_index as Real / width;
        let h = (scene.height() - 1 - row_index) as Real / height;
        let mut rng = rand::thread_rng();
        let mut color = Vec3R::default();
        let w = w + rng.gen::<Real>() / width;
        let h = h + rng.gen::<Real>() / height;
        if scene.debug_surfaces {
            color += debug_surfaces(&camera.ray_at(w, h), scene, 1);
        } else {
            color += ray_color(&camera.ray_at(w, h), scene, 1);
        }
        (color.x, color.y, color.z)
    });

    //println!("image rendered in {:.3?}", rendering_start.elapsed());
}
