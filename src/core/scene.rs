use super::camera::Camera;
use super::defs::Real;
use super::geometry::*;
use super::material::*;
use super::object::Object;
use super::primitives::*;
use super::renderer::renderer_buffer::*;
use super::renderer::*;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
struct DesScene {
    materials: HashMap<String, DesMaterial>,
    geometries: HashMap<String, GeometryType>,
    objects: Vec<ObjectEntry<String>>,
    camera: DesCamera,
    width: u16,
    height: u16,
    max_bounces: u8,
    debug_surfaces: bool,
    debug_error: Option<bool>,
}

#[derive(Deserialize, Debug)]
struct DesCamera {
    origin: Vec3R,
    rotation: Vec2R,
    fov: Real, // degrees
}

#[derive(Deserialize, Debug)]
struct ObjectEntry<T> {
    geometry: T,
    material: T,
}

pub struct Scene {
    pub camera: Camera,
    objects_map: Vec<ObjectEntry<usize>>,
    materials: Vec<Box<dyn Material + Send + Sync>>,
    geometries: Vec<Box<dyn Geometry + Send + Sync>>,
    width: usize,
    height: usize,
    pub max_bounces: usize,
    pub debug_surfaces: bool,
    pub debug_error: bool,
}

impl std::convert::TryFrom<&str> for Scene {
    type Error = String;

    fn try_from(data: &str) -> Result<Self, Self::Error> {
        match serde_json::from_str::<DesScene>(&data) {
            Ok(des_scene) => Scene::try_from(des_scene),
            Err(err) => Err(format!("Error parsing data: {}", err.to_string())),
        }
    }
}

impl std::convert::TryFrom<DesScene> for Scene {
    type Error = String;

    fn try_from(des_scene: DesScene) -> Result<Self, Self::Error> {
        let mut materials_indices: HashMap<String, usize> =
            HashMap::with_capacity(des_scene.materials.len());
        let mut materials: Vec<Box<dyn Material + Send + Sync>> =
            Vec::with_capacity(des_scene.materials.len());

        for (name, des_mat) in des_scene.materials {
            materials_indices.insert(name, materials.len());
            materials.push(des_mat.into());
        }

        let mut geometries_indices: HashMap<String, usize> =
            HashMap::with_capacity(des_scene.geometries.len());
        let mut geometries: Vec<Box<dyn Geometry + Send + Sync>> =
            Vec::with_capacity(des_scene.geometries.len());

        for (name, des_geo) in des_scene.geometries {
            geometries_indices.insert(name, geometries.len());
            geometries.push(des_geo.into());
        }

        let mut objects_map: Vec<ObjectEntry<usize>> = Vec::with_capacity(des_scene.objects.len());

        for obj_entry in des_scene.objects {
            if let Some(&mat_index) = materials_indices.get(&obj_entry.material) {
                if let Some(&geo_index) = geometries_indices.get(&obj_entry.geometry) {
                    objects_map.push(ObjectEntry {
                        material: mat_index,
                        geometry: geo_index,
                    })
                } else {
                    return Err(format!(
                        "cannot find geometry '{}' for object",
                        obj_entry.geometry
                    ));
                }
            } else {
                return Err(format!(
                    "cannot find material '{}' for object",
                    obj_entry.material
                ));
            }
        }

        let mut camera = Camera::new(
            (des_scene.width as Real) / (des_scene.height as Real),
            des_scene.camera.fov.to_radians(),
        );
        camera.origin = des_scene.camera.origin;
        camera.rotate(des_scene.camera.rotation);

        Ok(Scene {
            camera: camera,
            objects_map,
            materials,
            geometries,
            width: des_scene.width as usize,
            height: des_scene.height as usize,
            max_bounces: des_scene.max_bounces as usize,
            debug_surfaces: des_scene.debug_surfaces,
            debug_error: des_scene.debug_error.unwrap_or(false),
        })
    }
}

pub struct ObjectsIterator<'a> {
    scene: &'a Scene,
    index: usize,
}

impl<'a> Iterator for ObjectsIterator<'a> {
    type Item = Object<'a, 'a>;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(object_entry) = self.scene.objects_map.get(self.index) {
            self.index += 1;
            Some(Object {
                geometry: self.scene.geometries[object_entry.geometry].as_ref(),
                material: self.scene.materials[object_entry.material].as_ref(),
            })
        } else {
            None
        }
    }
}

impl Scene {
    pub fn objects_iter<'a>(&'a self) -> ObjectsIterator {
        ObjectsIterator {
            scene: self,
            index: 0,
        }
    }
    pub fn new_pixel_buffer(&self) -> impl RendererBuffer {
        PixelBuffer::new(self.width, self.height)
    }
    pub fn ratio(&self) -> f64 {
        (self.width as f64) / (self.height as f64)
    }
    pub fn render(&self, buffer: &mut impl RendererBuffer) {
        render(self, buffer);
    }
    pub fn width(&self) -> usize {
        self.width
    }
    pub fn height(&self) -> usize {
        self.height
    }
}
