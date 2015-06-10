use geometry::*;
use math::*;
use renderable::{Hit, Renderable};

use std::f64;

pub struct Scene {
    objects: Vec<Box<Renderable>>
}

impl Scene {
    pub fn new() -> Scene {
        Scene { objects: Vec::new() }
    }
    pub fn add(&mut self, object: Box<Renderable>) {
        self.objects.push(object);
    }
    pub fn intersect<'a>(&'a self, ray: &Ray) -> Option<Hit<'a>> {
        let mut hit_dist = f64::INFINITY;
        let mut hit_obj : Option<&Box<Renderable>> = None;
        for obj in self.objects.iter() {
            if let Some(dist) = obj.intersect(&ray) {
                if dist < hit_dist {
                    hit_dist = dist;
                    hit_obj = Some(&obj);
                }
            }
        }

        match hit_obj {
            None => { None },
            Some(obj) => {
                Some(obj.get_hit(&ray, hit_dist))
            }
        }
    }

    fn shadow_cast(&self, ray: &Ray, light: &Renderable) -> bool {
        let mut hit_obj : Option<&Renderable> = None;
        let mut hit_dist = f64::INFINITY;
        for obj in self.objects.iter() {
            if let Some(dist) = obj.intersect(&ray) {
                if dist < hit_dist {
                    hit_dist = dist;
                    hit_obj = Some(&**obj);
                }
            }
        }
        match hit_obj {
            None => { false },
            Some(obj) => {
                if obj as *const Renderable == light as *const Renderable { true } else { false }
            }
        }
    }

    pub fn sample_lights(&self, ray: &Ray, rng: &mut F64Rng) -> Vec3d {
        let emission = Vec3d::zero();
        for obj in self.objects.iter() {
            if !obj.is_emissive() { continue; }
            let light_pos = obj.random_pos(rng);
            let dir = (light_pos - ray.origin).normalized();
            let ray = Ray::new(ray.origin, dir);
            if self.shadow_cast(&ray, &**obj) {
            }
        }
        emission
    }
}
