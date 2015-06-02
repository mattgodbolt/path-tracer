extern crate argparse;
extern crate image;
extern crate path_tracer;

use argparse::{ArgumentParser, Store, Collect};
use path_tracer::*;
use std::fs::File;
use std::io::{self, BufReader};
use std::io::prelude::*;

#[derive(Debug)]
enum ImageError {
    IoError(io::Error),
    BadFileError(String)
}
use ImageError::*;

impl From<io::Error> for ImageError {
    fn from(e: io::Error) -> ImageError {
        ImageError::IoError(e)
    }
}
type Result<T> = std::result::Result<T, ImageError>;

struct PartialImage {
    image : Vec<Vec<Vec3d>>,
    samples : i32,
}

impl PartialImage {
    fn empty() -> PartialImage {
        PartialImage { image: Vec::new(), samples: 0 }
    }

    // I'm totally unsure about whether this is taking additional copies.
    fn add(self, other: PartialImage) -> PartialImage {
        let image = if self.samples == 0 { other.image } else {
            // A nicer way to do this would be ideal. This may well be doing lots of boundchecks.
            let mut combined : Vec<Vec<Vec3d>> = self.image;
            for y in 0..combined.len() {
                for x in 0..combined[y].len() {
                    combined[y][x] = combined[y][x] + other.image[y][x];
                }
            }
            combined
        };
        let samples = self.samples + other.samples;
        PartialImage { image: image, samples: samples }
    }

    fn height(&self) -> usize {
        self.image.len()
    }
    fn width(&self) -> usize {
        self.image[0].len()
    }
}

fn load_file(name: &String) -> Result<PartialImage> {
    let mut result : Vec<Vec<Vec3d>> = Vec::new();
    let file = BufReader::new(try!(File::open(&name))); 
    println!("Loading '{}'", name);
    let mut line_iter = file.lines();
    let first_line = line_iter.next().unwrap().ok().unwrap();
    let mut first_line_words = first_line.split(' ').map(|x| x.parse::<i32>().unwrap());
    let width = first_line_words.next().unwrap() as usize;
    let height = first_line_words.next().unwrap() as usize;
    let samples = first_line_words.next().unwrap(); // this is terrible
    println!("Found {} samples in {}x{} image", samples, width, height);
    for line in line_iter.filter_map(|x| x.ok()) {
        let mut vecs : Vec<Vec3d> = Vec::new();
        let mut split = line.split(' ').map(|x| x.parse::<f64>().unwrap());
        loop {
            let x = match split.next() {
                None => break,
                Some(x) => x
            };
            let y : f64 = split.next().unwrap();
            let z : f64 = split.next().unwrap();
            vecs.push(Vec3d::new(x, y, z) * samples as f64);
        }
        if vecs.len() != width {
            return Err(BadFileError("Bad width".to_string()));
        }
        result.push(vecs);
    }
    if result.len() != height {
        return Err(BadFileError("Bad height".to_string()));
    }
    println!("Loaded ok");
    Ok(PartialImage { image: result, samples: samples })
}

fn main() {
    let mut to_merge : Vec<String> = Vec::new();
    let mut output_filename = "image.png".to_string();
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Combine several sample images into one PNG");
        ap.refer(&mut output_filename).add_option(&["-o", "--output"], Store, 
                                                  "Filename to output to");
        ap.refer(&mut to_merge).add_argument("files", Collect, "Files to merge")
            .required();
        ap.parse_args_or_exit();
    }
    let accum : PartialImage = to_merge.iter()
        .map(load_file)
        .map(|x| x.unwrap())
        .fold(PartialImage::empty(), |acc, item| { acc.add(item) });

    println!("Merged {} samples", accum.samples);
    println!("Writing output to '{}'", output_filename);
    let height = accum.height();
    let width = accum.width();
    let samples = accum.samples;
    let mut image = image::ImageBuffer::new(width as u32, height as u32);
    for y in 0..height {
        for x in 0..width {
            let sum = accum.image[y][x] / samples as f64;
            image.put_pixel(x as u32, y as u32, image::Rgb([to_int(sum.x), to_int(sum.y), to_int(sum.z)]));
        }
    }
    let mut output_file = File::create(output_filename).unwrap();
    image::ImageRgb8(image).save(&mut output_file, image::PNG).unwrap();
}
