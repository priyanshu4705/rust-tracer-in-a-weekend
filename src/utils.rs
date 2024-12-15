use std::{
    fs::File,
    io::{BufWriter, Write},
};

use vec3D::Vec3D;

pub fn write_ppm(out: &mut BufWriter<&mut File>, color: Vec3D) {
    let r = (color.x * 255.999) as u32;
    let g = (color.y * 255.999) as u32;
    let b = (color.z * 255.999) as u32;

    out.write_fmt(format_args!("{r} {g} {b}\n")).unwrap();
}

pub struct Ray {
    pub point: Vec3D,
    pub direction: Vec3D,
}

impl Ray {
    pub fn at(&self, t: f64) -> Vec3D {
        self.point + self.direction * t
    }
}
