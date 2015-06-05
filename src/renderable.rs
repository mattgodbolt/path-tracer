use geometry::Ray;
use material::Material;
use math::Vec3d;

pub struct Hit<'a> {
    pub pos: Vec3d,
    pub normal: Vec3d,
    pub material: &'a Material, 
    pub emission: Vec3d,
    pub colour: Vec3d
}

pub trait Renderable : Send + Sync {
    fn intersect(&self, ray: &Ray) -> Option<f64>;
    fn get_hit(&self, ray: &Ray, dist: f64) -> Hit;
}
