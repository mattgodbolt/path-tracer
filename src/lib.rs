extern crate rand;
mod geometry;
mod material;
mod math;
mod renderable;
mod scene;

pub use self::geometry::*;
pub use self::material::Material;
pub use self::math::*;
pub use self::scene::*;

use self::renderable::Renderable;


use std::f64::consts::PI;

pub fn radiance(scene: &Scene, ray: &Ray, depth: i32, rng: &mut F64Rng, emit: bool) -> Vec3d {
    scene.intersect(&ray).map_or(Vec3d::zero(), |hit| {
        let n1 = if hit.normal.dot(ray.direction) < 0.0 { hit.normal } else { hit.normal.neg() };
        let mut emission = if emit { hit.emission } else { Vec3d::zero() };
        let mut colour = hit.colour;
        let max_reflectance = colour.max_component();
        let depth = depth + 1;
        if depth > 5 {
            let rand = rng.next();
            if rand < max_reflectance && depth < 500 { // Rust's stack blows up ~600 on my machine
               colour = colour * (1.0 / max_reflectance);
            } else {
                return emission;
            }
        }
        match *hit.material {
            Material::Diffuse => {
                // Get a random polar coordinate.
                let r1 = rng.next() * 2.0 * PI;
                let r2 = rng.next();
                let r2s = r2.sqrt();
                // Create a coordinate system u,v,w local to the point, where the w is the normal
                // pointing out of the sphere and the u and v are orthonormal to w.
                let w = n1;
                // Pick an arbitrary non-zero preferred axis for u
                let u = if n1.x.abs() > 0.1 { Vec3d::new(0.0, 1.0, 0.0) } else { Vec3d::new(1.0, 0.0, 0.0) }.cross(w);
                let v = w.cross(u);
                // construct the new direction
                let new_dir = u * r1.cos() * r2s + v * r1.sin() * r2s + w * (1.0 - r2).sqrt();
                let new_ray = Ray::new(hit.pos, new_dir.normalized());
                emission = emission + colour * scene.sample_lights(hit.pos, hit.normal, rng);
                colour = colour * radiance(scene, &new_ray, depth, rng, false);
            }, 
            Material::Specular => {
                let reflection = ray.direction - hit.normal * 2.0 * hit.normal.dot(ray.direction);
                let reflected_ray = Ray::new(hit.pos, reflection);
                colour = colour * radiance(scene, &reflected_ray, depth, rng, true);
            },
            Material::Refractive => {
                let reflection = ray.direction - hit.normal * 2.0 * hit.normal.dot(ray.direction);
                let reflected_ray = Ray::new(hit.pos, reflection);
                let into = hit.normal.dot(n1) > 0.0;
                let nc = 1.0;
                let nt = 1.5;
                let nnt = if into { nc/nt } else { nt/nc };
                let ddn = ray.direction.dot(n1);
                let cos2t = 1.0 - nnt * nnt * (1.0 - ddn * ddn);
                if cos2t < 0.0 {
                    // Total internal reflection
                    colour = colour * radiance(scene, &reflected_ray, depth, rng, true);
                } else {
                    let tbd = ddn * nnt + cos2t.sqrt();
                    let tbd = if into { tbd } else { -tbd };
                    let tdir = (ray.direction * nnt - hit.normal * tbd).normalized();
                    let transmitted_ray = Ray::new(hit.pos, tdir);
                    let a = nt - nc;
                    let b = nt + nc;
                    let r0 = (a * a) / (b * b);
                    let c = 1.0 - if into { -ddn } else { tdir.dot(hit.normal) };
                    let re = r0 + (1.0 - r0) * c * c * c * c * c;
                    let tr = 1.0 - re;
                    let p = 0.25 + 0.5 * re;
                    let rp = re / p;
                    let tp = tr / (1.0 - p);
                    colour = colour * if depth > 2 {
                        if rng.next() < p {
                            radiance(scene, &reflected_ray, depth, rng, true) * rp
                        } else {
                            radiance(scene, &transmitted_ray, depth, rng, true) * tp
                        }
                    } else {
                        radiance(scene, &reflected_ray, depth, rng, true) * re +
                            radiance(scene, &transmitted_ray, depth, rng, true) * tr
                    }
                }
            }
        }
        emission + colour
    })
}

pub fn random_samp<T: F64Rng>(rng: &mut T) -> f64 {
    let r = 2.0 * rng.next();
    if r < 1.0 { r.sqrt() - 1.0 } else { 1.0 - (2.0 - r).sqrt() }
}

pub fn to_int(v: f64) -> u8 {
    let ch = (v.powf(1.0/2.2) * 255.0 + 0.5) as i64;
    if ch < 0 { 0u8 } else if ch > 255 { 255u8 } else { ch as u8 }
}
