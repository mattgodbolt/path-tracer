use path_tracer::Vec3d;

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
