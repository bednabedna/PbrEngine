use super::geometry::Geometry;
use super::material::Material;

pub struct Object<'a, 'b> {
    pub geometry: &'b dyn Geometry,
    pub material: &'a dyn Material,
}
