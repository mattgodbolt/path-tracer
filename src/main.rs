use std::fs::File;
use std::io::prelude::*;
use std::ops::{Add,Sub,Mul,Div};

extern crate rand;
use rand::{Rng, XorShiftRng, SeedableRng};

#[derive(Debug,Clone,Copy)]
struct Vec3d {
    x: f64,
    y: f64,
    z: f64
}

impl Vec3d {
    fn normalized(self) -> Vec3d {
        self / self.dot(self).sqrt()
    }
    fn dot(self, other: Vec3d) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    fn new(x: f64, y: f64, z: f64) -> Vec3d {
        Vec3d { x: x, y: y, z: z }
    }
    fn zero() -> Vec3d {
        Vec3d { x: 0.0, y: 0.0, z: 0.0 }
    }
    fn cross(self, other: Vec3d) -> Vec3d {
        Vec3d { 
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x
        }
    }
    fn maxComponent(self) -> f64 {
        if self.x > self.y && self.x > self.z { self.x }
        else if self.y > self.x && self.y > self.z { self.y }
        else { self.z }
    }
    fn neg(self) -> Vec3d {
        Vec3d { x: -self.x, y: -self.y, z: -self.z }
    }
}


impl Add for Vec3d {
    type Output = Vec3d;

    fn add(self, other: Vec3d) -> Vec3d {
        Vec3d { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
    }
}

impl Sub for Vec3d {
    type Output = Vec3d;

    fn sub(self, other: Vec3d) -> Vec3d {
        Vec3d { x: self.x - other.x, y: self.y - other.y, z: self.z - other.z }
    }
}

impl Mul for Vec3d {
    type Output = Vec3d;

    fn mul(self, other: Vec3d) -> Vec3d {
        Vec3d { x: self.x * other.x, y: self.y * other.y, z: self.z * other.z }
    }
}

impl Mul<f64> for Vec3d {
    type Output = Vec3d;

    fn mul(self, other: f64) -> Vec3d {
        Vec3d { x: self.x * other, y: self.y * other, z: self.z * other }
    }
}

impl Div<f64> for Vec3d {
    type Output = Vec3d;

    fn div(self, other: f64) -> Vec3d {
        Vec3d { x: self.x / other, y: self.y / other, z: self.z / other }
    }
}

struct Ray {
    origin: Vec3d,
    direction: Vec3d
}

impl Ray {
    fn new(origin: Vec3d, direction: Vec3d) -> Ray {
        Ray { origin: origin, direction: direction }
    }
}

enum Material {
    Diffuse,
    Specular,
    Refractive
}

struct Sphere {
    material: Material,
    radius: f64,
    position: Vec3d,
    emission: Vec3d,
    colour: Vec3d
}

impl Sphere {
    fn new(material: Material, radius: f64, position: Vec3d, emission: Vec3d, colour: Vec3d) -> Sphere {
        Sphere {
          material: material, 
          radius: radius, 
          position: position,
          emission: emission,
          colour: colour
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
        const Epsilon : f64 = 0.0001;
        if t1 > Epsilon {
            Some(t1)
        } else if t2 > Epsilon {
            Some(t2)
        } else {
            None
        }
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

struct HitRecord<'a> {
    sphere: &'a Sphere,
    dist: f64
}

fn intersect<'a>(scene: &'a Vec<Sphere>, ray: &Ray) -> Option<HitRecord<'a>> {
    let mut result : Option<HitRecord<'a>> = None;
    for sph in scene.iter() {
        if let Some(dist) = sph.intersect(&ray) {
            // TODO: what's the difference between Some(&x) and Some(ref x) ? the latter works
            if match result { None => true, Some(ref x) => dist < x.dist } {
                result = Some(HitRecord { sphere: &sph, dist: dist });
            }
        }
    }
    result
}

fn radiance<R: Rng>(scene: &Vec<Sphere>, ray: &Ray, depth: i32, rng: &mut R) -> Vec3d {
    if let Some(hit) = intersect(&scene, &ray) {
        let hitPos = ray.origin + ray.direction * hit.dist;
        let hitNormal = (hitPos - hit.sphere.position).normalized();
        let n1 = if hitNormal.dot(ray.direction) < 0.0 { hitNormal } else { hitNormal.neg() };
        let mut colour = hit.sphere.colour;
        let maxReflectance = colour.maxComponent();
        let depth = depth + 1;
        if depth > 5 {
            let rand = rng.gen::<f64>();
            if rand < maxReflectance {
               colour = colour * (1.0 / maxReflectance);
            } else {
                return hit.sphere.emission;
            }
        }
        match hit.sphere.material {
            Material::Diffuse => {
            }, 
            Material::Specular => {
                //let reflection = ray.direction - hitNormal * 2.0 * hitNormal.dot(ray.direction);
                //colour = hit.sphere.emission + colour * radiance(scene, &Ray::new(hitPos, reflection), depth, rng);
            },
            Material::Refractive => {
            }
        }
        colour
    } else {
        Vec3d::zero()
    }
}

fn randomSamp<T: Rng>(rng: &mut T) -> f64 {
    let r = 2.0 * rng.gen::<f64>();
    if r < 1.0 { r.sqrt() - 1.0 } else { 1.0 - (2.0 - r).sqrt() }
}

fn to_int(v: f64) -> u8 {
    if v < 0.0 {
        0
    } else if v > 1.0 {
        1
    } else {
        (v * 255.0) as u8
    }
}

fn main() {
    const Black : Vec3d = Vec3d { x: 0.0, y: 0.0, z: 0.0 };
    const Red : Vec3d = Vec3d { x: 0.75, y: 0.25, z: 0.25 };
    const Blue : Vec3d = Vec3d { x: 0.25, y: 0.25, z: 0.75 };
    const Grey : Vec3d = Vec3d { x: 0.75, y: 0.75, z: 0.75 };
    const White : Vec3d = Vec3d { x: 0.999, y: 0.999, z: 0.999 };
    let scene = vec!{
        Sphere::new(Material::Diffuse, 1e5, 
            Vec3d::new(1e5+1.0, 40.8, 81.6),
            Black, Red),
        Sphere::new(Material::Diffuse, 1e5, 
            Vec3d::new(-1e5+99.0, 40.8, 81.6),
            Black, Blue),
        Sphere::new(Material::Diffuse, 1e5, 
            Vec3d::new(50.0, 40.8, 1e5),
            Black, Grey),
        Sphere::new(Material::Diffuse, 1e5, 
            Vec3d::new(50.0, 40.8, -1e5 + 170.0),
            Black, Black),
        Sphere::new(Material::Diffuse, 1e5, 
            Vec3d::new(50.0, 1e5, 81.6),
            Black, Grey),
        Sphere::new(Material::Diffuse, 1e5, 
            Vec3d::new(50.0, -1e5 + 81.6, 81.6),
            Black, Red),
        Sphere::new(Material::Specular, 16.5, 
            Vec3d::new(27.0, 16.5, 47.0),
            Black, White),
        Sphere::new(Material::Refractive, 16.5, 
            Vec3d::new(73.0, 16.5, 78.0),
            Black, White),
        Sphere::new(Material::Diffuse, 600.0, 
            Vec3d::new(50.0, 681.6 - 0.27, 81.6),
            Vec3d::new(12.0, 12.0, 12.0), Black),
    };

    const Width: usize = 256;//1024;
    const Height: usize = 192;//768;
    let Samps = 1;
    let cameraPos = Vec3d::new(50.0, 52.0, 295.6);
    let cameraDir = Vec3d::new(0.0, -0.042612, -1.0);
    let camera = Ray::new(cameraPos, cameraDir.normalized());
    let cameraX = Vec3d::new(Width as f64 * 0.5135 / Height as f64, 0.0, 0.0);
    let cameraY = cameraX.cross(camera.direction).normalized() * 0.5135;

    let mut rng = XorShiftRng::new_unseeded();
    let mut outputFile = File::create("image.ppm").unwrap();
    write!(&mut outputFile, "P3\n{} {}\n255\n", Width, Height);

    for y in 0..Height {
        println!("Rendering ({} spp) {:.4}%...", Samps * 4, 100.0 * y as f64 / Height as f64);
        for x in 0..Width {
            rng.reseed([10, 200, 999999, y as u32]);
            let mut sum = Vec3d::zero();
            for sx in 0..2 {
                for sy in 0..2 {
                    for s in 0..Samps {
                        let dx = randomSamp(&mut rng);
                        let dy = randomSamp(&mut rng);
                        let dir = cameraX * (((sx as f64 + 0.5 + dx)/2.0 + x as f64) / Width as f64 - 0.5) +
                            cameraY * (((sy as f64 + 0.5 + dy)/2.0 + y as f64) / Height as f64  - 0.5) + cameraDir;
                        let jitteredRay = Ray::new(cameraPos + dir * 140.0, dir.normalized());
                        sum = sum + radiance(&scene, &jitteredRay, 0, &mut rng) / (Samps * 4) as f64;
                    }
                }
            }
            write!(&mut outputFile, "{} {} {} ", to_int(sum.x), to_int(sum.y), to_int(sum.z));
        }
    }
}
