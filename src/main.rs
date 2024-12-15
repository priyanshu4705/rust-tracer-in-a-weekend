#![allow(non_upper_case_globals)]

mod utils;

use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::{
    fs::File,
    io::{BufWriter, Write},
};
use utils::Ray;
use vec3D::Vec3D;

fn hit_sphere(center: Vec3D, radius: f64, ray: Ray) -> bool {
    let oc = ray.point - center;
    let a = ray.direction.dot(ray.direction);
    let b = oc.dot(ray.direction) * -2.0;
    let c = oc.dot(oc) - radius * radius;
    let discriminant = b * b - (4.0 * a * c);

    discriminant >= 0.0
}

fn ray_color(ray: Ray) -> Vec3D {
    if hit_sphere(Vec3D::new(0.0, 0.0, -1.0), 0.5, ray) {
        return Vec3D::new(1.0, 0.0, 0.0);
    }

    let unit_direction = ray.direction.unit();
    let a = 0.5 * (unit_direction.y + 1.0);
    (Vec3D::ones() * (1.0 - a)) + (Vec3D::new(0.5, 0.7, 1.0) * a)
}

pub fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() != 2 {
        println!("Usage: rust-tracer-in-a-weekend <image-number>");
        return;
    }

    let image_number = &args[1];

    // image
    const aspect_ratio: f64 = 16.0 / 9.0;
    const image_width: f64 = 400.0;

    const ih: f64 = image_width / aspect_ratio;
    const image_height: f64 = if ih < 1.0 { 1.0 } else { ih };

    let mut out = File::create(format!("output/image_{image_number}.ppm")).unwrap();
    let mut writer = BufWriter::new(&mut out);

    // camera
    const focal_length: f64 = 1.0;
    const viewport_height: f64 = 2.0;
    const viewport_width: f64 = (image_width / image_height) * viewport_height;

    let camera_center: Vec3D = Vec3D::zeros();

    // vectors across horizontal and vertical edges from camera center
    let viewport_u = Vec3D::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3D::new(0.0, -viewport_height, 0.0);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_u = viewport_u / image_width;
    let pixel_delta_v = viewport_v / image_height;

    // Calculate the location of the upper left pixel.
    let viewport_upper_left = camera_center
        - Vec3D::new(0.0, 0.0, focal_length)
        - (viewport_u / 2.0)
        - (viewport_v / 2.0);
    let pixel00_loc = viewport_upper_left + ((pixel_delta_u + pixel_delta_v) * 0.5);

    writer
        .write_fmt(format_args!("P3\n{} {}\n255\n", image_width, image_height))
        .unwrap();

    let pixels: Vec<Vec3D> = (0..image_height as u32)
        .into_par_iter()
        .flat_map(|j| {
            (0..image_width as u32).into_par_iter().map(move |i| {
                let pixel_center =
                    pixel00_loc + (pixel_delta_u * i as f64) + (pixel_delta_v * j as f64);
                let ray_direction = pixel_center - camera_center;
                let ray = Ray {
                    point: camera_center,
                    direction: ray_direction,
                };

                ray_color(ray)
            })
        })
        .collect();

    for pixel in pixels {
        utils::write_ppm(&mut writer, pixel);
    }

    writer.flush().unwrap();
}
