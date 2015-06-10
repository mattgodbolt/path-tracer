// Based on smallpt, http://www.kevinbeason.com/smallpt/ which is also licensed under
// the MIT license.
extern crate argparse;
extern crate image;
extern crate num_cpus;
extern crate rand;
extern crate threadpool;

use argparse::{ArgumentParser, Store, StoreTrue};
use rand::{XorShiftRng, SeedableRng};
use threadpool::ThreadPool;

use std::fs::File;
use std::io::{self, BufWriter};
use std::io::prelude::*;
use std::sync::Arc;
use std::sync::mpsc::channel;

extern crate path_tracer;
use path_tracer::*;


fn main() {
    let mut samps = 1;
    let mut width = 1024;
    let mut height = 768;
    let mut output_filename = "".to_string();
    let mut num_threads = num_cpus::get();
    let mut seed = 0x193a6754;
    let mut partial = false;
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Render a simple image");
        ap.refer(&mut samps).add_option(&["-s", "--samples"], Store, "Number of samples");
        ap.refer(&mut height).add_option(&["-h", "--height"], Store, "Height");
        ap.refer(&mut width).add_option(&["-w", "--width"], Store, "Width");
        ap.refer(&mut output_filename).add_option(&["-o", "--output"], Store, 
                                                  "Filename to output to");
        ap.refer(&mut num_threads).add_option(&["--num-threads"], Store,
                                              "Number of threads to use");
        ap.refer(&mut seed).add_option(&["--seed"], Store, "Random seed");
        ap.refer(&mut partial).add_option(&["--partial"], StoreTrue,
                                          "Output a partial render");
        ap.parse_args_or_exit();
    }
    samps = samps / 4;
    if samps < 1 { samps = 1; }
    if output_filename == "" {
        output_filename = if partial { "image.part" } else { "image.png" }.to_string();
    }
    const BLACK : Vec3d = Vec3d { x: 0.0, y: 0.0, z: 0.0 };
    const RED : Vec3d = Vec3d { x: 0.75, y: 0.25, z: 0.25 };
    const BLUE : Vec3d = Vec3d { x: 0.25, y: 0.25, z: 0.75 };
    const GREY : Vec3d = Vec3d { x: 0.75, y: 0.75, z: 0.75 };
    const WHITE : Vec3d = Vec3d { x: 0.999, y: 0.999, z: 0.999 };
    let mut scene = Scene::new();
    scene.add(Box::new(Sphere::new(Material::Diffuse, 1e5, 
                                   Vec3d::new(1e5+1.0, 40.8, 81.6),
                                   BLACK, RED)));
    scene.add(Box::new(Sphere::new(Material::Diffuse, 1e5, 
                                   Vec3d::new(-1e5+99.0, 40.8, 81.6),
                                   BLACK, BLUE)));
    scene.add(Box::new(Sphere::new(Material::Diffuse, 1e5, 
                                   Vec3d::new(50.0, 40.8, 1e5),
                                   BLACK, GREY)));
    scene.add(Box::new(Sphere::new(Material::Diffuse, 1e5, 
                                   Vec3d::new(50.0, 40.8, -1e5 + 170.0),
                                   BLACK, BLACK)));
    scene.add(Box::new(Sphere::new(Material::Diffuse, 1e5, 
                                   Vec3d::new(50.0, 1e5, 81.6),
                                   BLACK, GREY)));
    scene.add(Box::new(Sphere::new(Material::Diffuse, 1e5, 
                                   Vec3d::new(50.0, -1e5 + 81.6, 81.6),
                                   BLACK, GREY)));
    scene.add(Box::new(Sphere::new(Material::Specular, 16.5, 
                                   Vec3d::new(27.0, 16.5, 47.0),
                                   BLACK, WHITE)));
    scene.add(Box::new(Sphere::new(Material::Refractive, 16.5, 
                                   Vec3d::new(73.0, 16.5, 78.0),
                                   BLACK, WHITE)));
    scene.add(Box::new(Sphere::new(Material::Diffuse, 600.0, 
                                   Vec3d::new(50.0, 681.6 - 0.27, 81.6),
                                   Vec3d::new(12.0, 12.0, 12.0), BLACK)));
    let scene = Arc::new(scene);

    let camera_pos = Vec3d::new(50.0, 52.0, 295.6);
    let camera_dir = Vec3d::new(0.0, -0.042612, -1.0).normalized();
    let camera_x = Vec3d::new(width as f64 * 0.5135 / height as f64, 0.0, 0.0);
    let camera_y = camera_x.cross(camera_dir).normalized() * 0.5135;

    println!("Using {} threads", num_threads);
    let pool = ThreadPool::new(num_threads);
    let (tx, rx) = channel();

    for y in 0..height {
        let tx = tx.clone();
        let scene = scene.clone();
        pool.execute(move || {
            let mut line = Vec::with_capacity(width);
            let mut rng = XorShiftRng::from_seed([1 + (y * y) as u32, seed, 0x15aac60d, 0xb017f00d]);
            for x in 0..width {
                let mut sum = Vec3d::zero();
                for sx in 0..2 {
                    for sy in 0..2 {
                        let mut r = Vec3d::zero();
                        for _samp in 0..samps {
                            let dx = random_samp(&mut rng);
                            let dy = random_samp(&mut rng);
                            let sub_x = (sx as f64 + 0.5 + dx) / 2.0;
                            let dir_x = (sub_x + x as f64) / width as f64 - 0.5;
                            let sub_y = (sy as f64 + 0.5 + dy) / 2.0;
                            let dir_y = (sub_y + (height - y - 1) as f64) / height as f64 - 0.5;
                            let dir = (camera_x * dir_x + camera_y * dir_y + camera_dir).normalized();
                            let jittered_ray = Ray::new(camera_pos + dir * 140.0, dir);
                            let sample = radiance(&scene, &jittered_ray, 0, &mut rng, true);
                            r = r + (sample / samps as f64);
                        }
                        sum = sum + r.clamp() * 0.25;
                    }
                }
                line.push(sum);
            }
            tx.send((y, line)).unwrap();
        });
    }
    let mut left = height;
    let mut screen : Vec<Vec<Vec3d>> = Vec::new();
    for _y in 0..height {
        screen.push(Vec::new());
    }
    while left > 0 {
        print!("Rendering ({} spp) {:.4}%...\r", samps * 4, 100.0 * (height - left) as f64 / height as f64);
        io::stdout().flush().ok().expect("Could not flush stdout");
        let (y, line) = rx.recv().unwrap();
        screen[y] = line;
        left -= 1;
    }
    if !partial {
        println!("\nWriting output to '{}'", output_filename);
        let mut image = image::ImageBuffer::new(width as u32, height as u32);
        for y in 0..height {
            for x in 0..width {
                let sum = screen[y][x];
                image.put_pixel(x as u32, y as u32, image::Rgb([to_int(sum.x), to_int(sum.y), to_int(sum.z)]));
            }
        }
        let mut output_file = File::create(output_filename).unwrap();
        image::ImageRgb8(image).save(&mut output_file, image::PNG).unwrap();
    } else {
        println!("\nWriting partial output to '{}'", output_filename);
        let mut writer = BufWriter::new(File::create(output_filename).unwrap());
        write!(&mut writer, "{} {} {}\n", width, height, samps).unwrap();
        for y in 0..height {
            for x in 0..width {
                let sum = screen[y][x];
                if x != 0 { write!(&mut writer, " ").unwrap(); }
                write!(&mut writer, "{} {} {}", sum.x, sum.y, sum.z).unwrap();
            }
            write!(&mut writer, "\n").unwrap();
        }
    }
}
