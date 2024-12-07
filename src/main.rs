#[macro_use] extern crate lazy_static;

use std::ops::{Add, Sub, Mul, Rem};
use std::io::Write;
use std::fs;
use rayon::prelude::*;

const EPS:f64=1.0e-4;
const INF:f64=1.0e20;

fn random() -> f64 {
    rand::random::<f64>()
}

#[derive(Copy, Clone, Debug)]
struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 {x, y, z}
    }
    fn zero() -> Vec3 {
        Vec3::new(0.0, 0.0, 0.0)
    }
    fn mult(&self, b: &Vec3) -> Vec3 {
        Vec3::new(self.x * b.x, self.y * b.y, self.z * b.z)
    }
    fn norm(mut self) -> Vec3 {
        let l = 1.0 / (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        self.x = self.x * l;
        self.y = self.y * l;
        self.z = self.z * l;
        self
    }
    fn dot(&self, b: &Vec3) -> f64 {
        return self.x * b.x + self.y * b.y + self.z * b.z;
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Self) -> Self {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Self) -> Self {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Self {
        Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Rem for Vec3 {
    type Output = Vec3;
    fn rem(self, rhs: Self) -> Self {
        Vec3::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x
        )
    }
}

#[derive(Debug)]
struct Ray {
    o: Vec3,
    d: Vec3,
}

impl Ray {
    fn new(o: Vec3, d: Vec3) -> Ray {
        Ray { o, d }
    }
}

enum Refl {
    Diff,
    Spec,
    Refr,
}

struct Sphere {
    rad: f64,
    p: Vec3,
    e: Vec3,
    c: Vec3,
    refl: Refl,
}

type Color = Vec3;



impl Sphere {
    fn intersect(&self, r: &Ray) -> f64 {
        let op = self.p - r.o;
        let b = op.dot(&r.d);
        let mut det = b * b - op.dot(&op) + self.rad * self.rad;
        if det < 0.0 {
            return INF;
        }
        det = det.sqrt();
        let t = b - det;
        if t > EPS {
            return t;
        }
        let t = b + det;
        if t > EPS {
            return t;
        } else {
            return INF;
        }
    }
}

lazy_static! {
    static ref SPHERES: [Sphere; 9] = [
        Sphere { rad: 1e5,   p: Vec3::new( 1e5 + 1.0,      40.8, 81.6),e: Vec3::zero(),               c: Vec3::new(0.75, 0.25, 0.25), refl: Refl::Diff },//left
        Sphere { rad: 1e5,   p: Vec3::new(-1e5 + 99.0,    40.8, 81.6),e: Vec3::zero(),                c: Vec3::new(0.25, 0.25, 0.75), refl: Refl::Diff },//right
        Sphere { rad: 1e5,   p: Vec3::new(50.0,            40.8, 1e5),e: Vec3::zero(),                c: Vec3::new(0.75, 0.75, 0.75), refl: Refl::Diff },//front
        Sphere { rad: 1e5,   p: Vec3::new(50.0,    40.8,-1e5 + 170.0),e: Vec3::zero(),                c: Vec3::zero(), refl: Refl::Diff },//back
        Sphere { rad: 1e5,   p: Vec3::new(50.0,            1e5, 81.6),e: Vec3::zero(),                c: Vec3::new(0.75, 0.75, 0.75), refl: Refl::Diff },//bottom
        Sphere { rad: 1e5,   p: Vec3::new(50.0,-1e5 + 81.6+4.0, 81.6),e: Vec3::zero(),                c: Vec3::new(0.75, 0.75, 0.75), refl: Refl::Diff },//top
        Sphere { rad: 16.5,  p: Vec3::new(27.0,           16.5, 47.0),e: Vec3::zero(),                c: Vec3::new(1.0, 1.0, 1.0) * 0.999, refl: Refl::Spec },
        Sphere { rad: 16.5,  p: Vec3::new(73.0,           16.5, 78.0),e: Vec3::zero(),                c: Vec3::new(1.0, 1.0, 1.0) * 0.999, refl: Refl::Refr },
        Sphere { rad: 600.0, p: Vec3::new(50.0, 681.6-0.27+4.0, 81.6),e: Vec3::new(12.0, 12.0, 12.0), c: Vec3::zero(), refl: Refl::Diff },
	];
}

fn clamp(x: f64) -> f64 {
    if x < 0.0 {
        0.0
    } else if x > 1.0 {
        1.0
    } else {
        x
    }
}


fn to_int(x: f64) -> u8 {
    (clamp(x).powf(1.0 / 2.2) * 255.0 + 0.5) as u8
}

//fn save_ppm_file(filename: &str, image: Vec<Color>, width: usize, height: usize) {
//    let mut f = fs::File::create(filename).unwrap();
//    writeln!(f, "P3\n{} {}\n{}", width, height, 255).unwrap();
//    for i in 0..(width * (height)) {
//        write!(f, "{} {} {} ", to_int(image[i as usize].x), to_int(image[i as usize].y), to_int(image[i as usize].z)).unwrap();
//    }
//}

fn save_png_file(filename:&str, out_image: Vec<Color>, width: usize, height: usize) {

// Create a new ImgBuf with width: imgx and height: imgy
let mut imgbuf = image::ImageBuffer::new(width as u32, height as u32);

// Iterate over the coordinates and pixels of the image
	for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
		let i:usize=(x as usize)+(y as usize)*width;
		let r = to_int(out_image[i].x );
        let g = to_int(out_image[i].y);
        let b = to_int(out_image[i].z );
		*pixel = image::Rgb([r, g, b]);
	}

// Save the image t is deduced from the path
	imgbuf.save(filename).unwrap();
}

fn intersect(r: &Ray, t: &mut f64, id: &mut usize) -> bool {
    let n = SPHERES.len();
    *t = INF+20.0;
    for i in (0..n).rev() {
        let d = SPHERES[i].intersect(r);
        if d < *t {
            *t = d;
            *id = i;
        }
    }
    return *t < INF;
}

fn radiance(r: &Ray, depth: u8) -> Vec3 {
    let mut t: f64 = 0.0;
    let mut id = 0;
    if !intersect(r, &mut t, &mut id) {
        return Vec3::zero();
    }
    let obj = &SPHERES[id];
    let x = r.o + r.d * t;
    let n = (x - obj.p).norm();
    let nl = if n.dot(&r.d) < 0.0 { n } else { n * -1.0 };
    let mut f = obj.c;
    let p = if f.x > f.y && f.x > f.z { f.x } else if f.y > f.z { f.y } else { f.z };
    let depth = depth + 1;
    if depth > 5 {
        if depth < 127 && random() < p {
            f = f * (1.0 / p);
        } else {
            return obj.e;
        }
    }

    return match obj.refl {
        Refl::Diff => {
            let r1 = 2.0 * std::f64::consts::PI * random();
            let r2 = random();
            let r2s = r2.sqrt();
            let w = nl;
            let u = ((if w.x.abs() > 0.1 { Vec3::new(0.0, 1.0, 0.0) } else { Vec3::new(1.0, 0.0, 0.0) }) % w).norm();
            let v = w % u;
            let d = (u * f64::cos(r1) * r2s + v * f64::sin(r1) * r2s + w * (1.0 - r2).sqrt()).norm();
            obj.e + f.mult(&radiance(&Ray::new(x, d), depth))
        },
        Refl::Spec => {
            obj.e + f.mult(&radiance(&Ray::new(x, r.d - n * 2.0 * n.dot(&r.d)), depth))
        },
        _ => { // Refl.Refr
            let refl_ray = Ray::new(x, r.d - n * 2.0 * n.dot(&r.d));
            let into = n.dot(&nl) > 0.0;
            let nc = 1.0;
            let nt = 1.5;
            let nnt = if into { nc / nt } else { nt / nc };
            let ddn = r.d.dot(&nl);
            let cos2t = 1.0 - nnt * nnt * (1.0 - ddn * ddn);
            if cos2t < 0.0 {
                obj.e + f.mult(&radiance(&refl_ray, depth))
            } else {
                let tdir = r.d * nnt - n * ((if into { 1.0 } else { -1.0 }) * (ddn * nnt + cos2t.sqrt()));
                tdir.norm();
                let a = nt - nc;
                let b = nt + nc;
                let r0 = a * a / (b * b);
                let c = 1.0 - (if into { -ddn } else {tdir.dot(&n)});
                let re = r0 + (1.0 - r0) * c * c * c * c * c;
                let tr = 1.0 - re;
                let p = 0.25 + 0.5 * re;
                let rp = re / p;
                let tp = tr / (1.0 - p);
                obj.e + f.mult(&(
                    if depth > 2 {
                        if random() < p {
                            radiance(&refl_ray, depth) * rp
                        } else {
                            radiance(&Ray::new(x, tdir), depth) * tp
                        }
                    } else {
                        radiance(&refl_ray, depth) * re + radiance(&Ray::new(x, tdir), depth) * tr
                    }
                ))
            }
        }
    }

}

fn main() {
    let w:usize = 640;
    let h:usize = 480;
    let samps = if std::env::args().len() == 2 { std::env::args().skip(1).next().unwrap().parse().unwrap() } else { 1 };
    let cam = Ray::new(Vec3::new(50.0, 52.0, 295.6), Vec3::new(0.0, -0.042612, -1.0).norm());

    let cx = Vec3::new((w as f64) * 0.5135 / (h as f64), 0.0, 0.0);
    let cy = (cx % cam.d).norm() * 0.5135;
    let mut image = vec![Color::zero(); (w * h ) as usize];

    let bands: Vec<(usize, &mut [Color])> = image.chunks_mut(w as usize).enumerate().collect();
    bands.into_par_iter().for_each(|(y, band)| {
        let y2 = h - (y as usize)-1;
		if (y % 10)==0 {writeln!(std::io::stderr(), "Rendering ({} spp) {:5.2}%", samps * 4, 100.0 * (y as f64) / ((h as f64) - 1.0)).unwrap();}
        for x in 0..w {
            let mut r = Vec3::zero();
            for sy in 0..2 {
                for sx in 0..2 {
                    for _s in 0..samps {
                        let r1 = 2.0 * random();
                        let dx = if r1 < 1.0 { r1.sqrt() - 1.0 } else { 1.0 - (2.0 - r1).sqrt() };
                        let r2 = 2.0 * random();
                        let dy = if r2 < 1.0 { r2.sqrt() - 1.0 } else { 1.0 - (2.0 - r2).sqrt() }; 
                        let d = cx * ((((sx as f64) + 0.5 + dx) / 2.0 + (x as f64)) / (w as f64) - 0.5)
                              + cy * ((((sy as f64) + 0.5 + dy) / 2.0 + (y2 as f64)) / (h as f64) - 0.5) + cam.d;
                        r = r + radiance(&(Ray::new(cam.o + d * 140.0, d.norm())), 0) * (1.0 / (samps as f64));
                    }
                    band[x as usize] = band[x as usize] + r*(1.0/4.0 as f64);
					r=Vec3::zero();
                }
            }
        }
    });

//    save_ppm_file("image.ppm", image, w, h);
    save_png_file("image.png", image, w, h);
}
