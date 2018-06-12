use std::fs::File;
use std::io::prelude::*;
use std::ops::{Add, Sub, Mul, Div, Neg};

const WIDTH: u32 = 1200;
const HEIGHT: u32 = 800;

struct V {
    x: f32,
    y: f32,
    z: f32,
}

impl Add for V {
    type Output = V;

    fn add(self, other: V) -> V {
        V {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for V {
    type Output = V;

    fn sub(self, other: V) -> V {
        V {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul for V {
    type Output = V;

    fn mul(self, other: V) -> V {
        V {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl Div for V {
    type Output = V;

    fn div(self, other: V) -> V {
        V {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

impl Neg for V {
    type Output = V;

    fn neg(self) -> V {
        V {
            x: - self.x,
            y: - self.y,
            z: - self.z,
        }
    }
}

pub fn dot(a: V, b: V) -> f32 {
    a.x * b.x + a.y * b.y + a.z * b.z
}

pub fn cross(a: V, b: V) -> V {
    V {
        x: a.y * b.z - a.z * b.y,
        y: a.z * b.x - a.x * b.z,
        z: a.x * b.y - a.y * b.x,
    }
}

pub fn normalize(v: V) -> V {
    v / dot(v, v).sqrt()
}

struct Ray {
    o: V,
    d: V,
}

struct Sphere {
    p: V,
    d: f32,
}

struct Hit {
    t: f32,
    sphere: *
}

struct Scene {
    spheres: Vec<Sphere>,
}
    

fn main() -> std::io::Result<()> {

    let mut f = File::create("result.ppm")?;
    let mut output_str = "P3\n".to_owned()
        + &WIDTH.to_string() + &"\n".to_owned()
        + &HEIGHT.to_string() + &"\n".to_owned()
        + "255\n";
    for i in 0..WIDTH*HEIGHT {
        let x: u32 = i % WIDTH;
        let y: u32 = i / WIDTH;
        let ray: Ray = Ray {
            o: V {
                x: 2. * (x / WIDTH as f32) - 1.,
                y: 2. * (y / HEIGHT as f32) - 1.,
                z: 5.,
            },
            d: V {
                x: 0.,
                y: 0.,
                z: -1.,
            }
        };
        output_str += "0 255 255\n";
    }
    f.write_all(output_str.as_bytes());
    Ok(())
}
