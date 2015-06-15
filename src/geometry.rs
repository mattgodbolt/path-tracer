use material::Material;
use renderable::{Hit,Renderable};
use math::{Vec3d,F64Rng};
use std::f64::consts::PI;

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
    radius_squared: f64,
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
          radius_squared: radius * radius, 
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
        let determinant = b * b - op.dot(op) + self.radius_squared;
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
    fn random_emission(&self, from: Vec3d, normal: Vec3d, rng: &mut F64Rng) -> (Vec3d, Vec3d) {
        let pos_to_center = self.position - from;
        let dist_squared = pos_to_center.length_squared();
        let sw = pos_to_center.normalized();
        // todo make an ONB func
        let su = if sw.x.abs() > 0.1 { 
            Vec3d::new(0.0, 1.0, 0.0)
        } else {
            Vec3d::new(1.0, 0.0, 0.0)
        }.cross(sw).normalized();
        let sv = sw.cross(su);
        // radius / dist = opp / adjacent = sin(angle), we need cos(angle)
        // sin^2(a)+cos^2(a) = 1, so cos(a) = sqrt(1-sin^2(a)) = sqrt(1-opp^2/adj^2).
        let cos_a_max = (1.0 - self.radius_squared / dist_squared).sqrt();
        // Now the below is "just" a random cosine distribution, up to cos_a_max.
        let (eps1, eps2) = (rng.next(), rng.next());
        let cos_a = 1.0 - eps1 + eps1 * cos_a_max;
        let sin_a = (1.0 - cos_a * cos_a).sqrt();
        let phi = 2.0 * PI * eps2;
        let l = (su * phi.cos() * sin_a + sv * phi.sin() * sin_a + sw * cos_a).normalized();
        let omega = 2.0 * PI * (1.0 - cos_a_max);
        let emission = self.emission * l.dot(normal) * omega * (1.0 / PI);
        (l, emission)
    }
    fn identity(&self) -> u64 {
        self as *const Self as u64
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
