use geometry::*;
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
        for sph in self.objects.iter() {
            if let Some(dist) = sph.intersect(&ray) {
                if dist < hit_dist {
                    hit_dist = dist;
                    hit_obj = Some(&sph);
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
}
