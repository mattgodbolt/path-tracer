use material::Material;
use renderable::{Hit,Renderable};
use math::{Vec3d,F64Rng};

#[derive(Debug,Clone,Copy)]
pub struct Ray {
    pub origin: Vec3d,
    pub direction: Vec3d
}

impl Ray {
    pub fn new(origin: Vec3d, direction: Vec3d) -> Ray {
        Ray { origin: origin, direction: direction }
    }
}

pub struct Sphere {
    material: Material,
    radius: f64,
    position: Vec3d,
    emission: Vec3d,
    colour: Vec3d,
    emissive: bool,
}

unsafe impl Sync for Sphere {}
unsafe impl Send for Sphere {}

impl Sphere {
    pub fn new(material: Material, radius: f64, position: Vec3d, emission: Vec3d, colour: Vec3d) -> Sphere {
        Sphere {
          material: material, 
          radius: radius, 
          position: position,
          emission: emission,
          colour: colour,
          emissive: emission.max_component() > 0.0
        }
    }
}

impl Renderable for Sphere {
    fn get_hit(&self, ray: &Ray, dist: f64) -> Hit {
        let pos = ray.origin + ray.direction * dist;
        let normal = (pos - self.position).normalized();
        Hit {
            pos: pos,
            normal: normal,
            material: &self.material,
            colour: self.colour,
            emission: self.emission
        }
    }
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        let op = self.position - ray.origin;
        let b = op.dot(ray.direction);
        let determinant = b * b - op.dot(op) + self.radius * self.radius;
        if determinant < 0.0 { return None; }
        let determinant = determinant.sqrt();
        let t1 = b - determinant;
        let t2 = b + determinant;
        const EPSILON : f64 = 0.0001;
        if t1 > EPSILON {
            Some(t1)
        } else if t2 > EPSILON {
            Some(t2)
        } else {
            None
        }
    }
    fn is_emissive(&self) -> bool { self.emissive }
    fn random_pos(&self, rng: &mut F64Rng) -> Vec3d {
        Vec3d::zero()
    }
}

#[test]
fn intersection() {
    let sphere = Sphere::new(
        Material::Diffuse, 
        100.0, 
        Vec3d::new(0.0, 0.0, 200.0), 
        Vec3d::zero(), 
        Vec3d::zero());
    let ray = Ray::new(Vec3d::zero(), Vec3d::new(0.0, 0.0, 1.0));
    match sphere.intersect(&ray) {
        Some(x) => assert_eq!(x, 100.0),
        None => panic!("unexpected")
    }
    let ray = Ray::new(Vec3d::zero(), Vec3d::new(0.0, 1.0, 0.0));
    match sphere.intersect(&ray) {
        Some(_) => panic!("unexpected"),
        None => {}
    }
}
