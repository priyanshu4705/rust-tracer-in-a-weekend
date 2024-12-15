#![allow(non_upper_case_globals)]

mod utils;

use std::{
    fs::File,
    io::{BufWriter, Write},
};
use vec3D::Vec3D;

pub fn main() {
    // image
    const aspect_ratio: f64 = 16.0 / 9.0;
    const image_width: u32 = 400;

    const image_height: u32 = (image_width as f64 / aspect_ratio) as u32;

    let mut out = File::create("output/image_01.ppm").unwrap();
    let mut writer = BufWriter::new(&mut out);

    writer
        .write_fmt(format_args!("P3\n{} {}\n255\n", image_width, image_height))
        .unwrap();

    for y in 0..image_height {
        for x in 0..image_width {
            let color = Vec3D::new(
                x as f64 / image_width as f64,
                y as f64 / image_height as f64,
                0.0,
            );
            utils::write_ppm(&mut writer, color);
        }
    }

    writer.flush().unwrap();
}
