extern crate rayon;
use rayon::prelude::*;

use std::ops::{Mul, Add, Sub, AddAssign, MulAssign, Neg};
use std::iter;
use std::fmt::Display;
use std::sync::Arc;

const MAX_RAY_DEPTH:usize = 15;

/* Vec3 */
#[derive(Clone, PartialEq, Debug)]
pub struct Vec3<T>
{
    pub x: T,
    pub y: T,
    pub z: T
}


pub struct Val<T>(T);

impl<T> Display for Vec3<T> where T: Display {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[Vec3 x:{}, y:{}, z:{}]", self.x, self.y, self.z)
    }
}

impl<T> Vec3<T> 
where T: Copy + Default + Mul<Output = T> + Add<Output = T>
{
    pub fn new() -> Self {
        Vec3{
            x: T::default(), y: T::default(), z: T::default()
        }
    }

    pub fn from_val(v: T) -> Self {
        Vec3{
            x: v, y: v, z: v
        }
    }

    pub fn from_3_val(x: T, y: T, z: T) -> Self {
        Vec3{x, y, z}
    }

    pub fn length2(&self) -> T {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn dot(&self, other:&Self) -> T {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl Vec3<f64> {
    pub fn length(&self) -> f64 {
        self.length2().sqrt()
    }

    pub fn normalize(&mut self){
        let nor2 = self.length2();
        if nor2 > 0_f64 {
            let inv_nor = 1_f64 / nor2.sqrt();
            self.x *= inv_nor;
            self.y *= inv_nor;
            self.z *= inv_nor;
        }
    }
}

impl Vec3<f32> {
    pub fn length(&self) -> f32 {
        self.length2().sqrt()
    }
}

impl<T> Sub for Vec3<T>
    where T: Sub<Output = T> + Copy
{
    type Output = Self;

    fn sub(self, other:Self) ->Self::Output {
        Vec3{x: self.x - other.x, 
             y: self.y - other.y, 
             z: self.z - other.z }
    }
}

impl<T> Sub for &Vec3<T>
    where T: Sub<Output = T> + Copy
{
    type Output = Vec3<T>;

    fn sub(self, other:Self) -> Self::Output {
        Vec3{x: self.x - other.x, 
             y: self.y - other.y, 
             z: self.z - other.z}
    }
}

impl<T> Add for Vec3<T>
    where T: Add<Output = T>  + Copy
{
    type Output = Self;

    fn add(self, other:Self) -> Self::Output {
        Vec3{x: self.x + other.x, 
             y: self.y + other.y, 
             z: self.z + other.z }
    }
}

impl<T> Add for &Vec3<T>
    where T: Add<Output = T> + Copy
{
    type Output = Vec3<T>;

    fn add(self, other:Self) -> Self::Output {
        Vec3{x: self.x + other.x, 
             y: self.y + other.y, 
              z:self.z + other.z }
    }
}

impl<T> Mul<T> for Vec3<T>
    where T: Mul<Output =T> + Copy {
    type Output = Self;

    fn mul(self, other: T) -> Self::Output {
        Vec3{ x: self.x * other, 
              y: self.y * other, 
              z: self.z * other }
    }
}

impl<T> Mul<T> for &Vec3<T>
where T: Mul<Output=T> + Copy {
    type Output = Vec3<T>;

    fn mul(self, other: T) -> Self::Output {
        Vec3{ x: self.x * other, 
              y: self.y * other, 
              z: self.z * other}
    }
}

impl<T> Mul for Vec3<T>
    where T: Mul<Output = T>  + Copy
{
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Vec3{x: self.x * other.x, 
             y: self.y * other.y, 
             z:self.z * other.z}
    }
}

impl<T> Mul for &Vec3<T>
where T: Mul<Output = T> + Copy
{
    type Output = Vec3<T>;

    fn mul(self, other: Self) -> Self::Output {
        Vec3{x: self.x * other.x,
             y:self.y * other.y, 
             z:self.z * other.z}
    }
}

impl<T> AddAssign for Vec3<T>
    where T:Add<Output = T> + Copy
{ 
    fn add_assign(&mut self, rhs:Self) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
        self.z = self.z + rhs.z;
    }
}

impl<'a, T> AddAssign<&Vec3<T>> for Vec3<T>
where T:Add<Output = T> + Copy
{
    fn add_assign(&mut self, rhs:&Self) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
        self.z = self.z + rhs.z;
    }
}

impl<T> MulAssign for Vec3<T>
    where T:Mul<Output = T> + Copy
{
    fn mul_assign(&mut self, rhs:Self) {
        *self = Vec3 {x: self.x * rhs.x, 
                      y: self.y * rhs.y, 
                    z: self.z * rhs.z };
    }
}

impl<T> MulAssign<&Vec3<T>> for Vec3<T>
where T:Mul<Output = T> + Copy
{
    fn mul_assign(&mut self, rhs:&Self) {
        *self = Vec3 {x: self.x * rhs.x, 
                      y: self.y * rhs.y, 
                      z: self.z * rhs.z };
    }
}

impl<T> Neg for Vec3<T>
    where T:Neg<Output = T> + Copy
{
    type Output = Self;
    fn neg(self) -> Self::Output {
        Vec3{x: -self.x, 
             y: -self.y, 
             z: -self.z}
    }
}

pub type Vec3f = Vec3<f64>;

/* Sphere */
pub struct Sphere<T>  where T: Copy{
    pub center : Vec3<T>,
    pub radius : T,
    pub radius2 : T,
    pub surface_color : Vec3<T>,
    pub emission_color : Vec3<T>,
    pub transparency : T,
    pub reflection : T,

}

impl Sphere<f64> {
    pub fn intersect(&self, rayorig: &Vec3f, raydir: &Vec3f) -> Option<(f64, f64)> {
        let l = &self.center - rayorig;
        let tca = l.dot(raydir);
        if tca < 0 as f64 {
            return None;
        }
        let d2 = l.dot(&l) - tca * tca;
        if d2 > self.radius2 {
            return None;
        } 
        let thc = (self.radius2 - d2).sqrt();
        Some((tca - thc, tca + thc))

    }
}

impl Sphere<f32> {
    pub fn intersect(&self, rayorig: &Vec3<f32>, raydir: &Vec3<f32>) -> Option<(f32, f32)> {
        let l = &self.center - rayorig;
        let tca = l.dot(raydir);
        if tca < 0 as f32 {
            return None;
        }
        let d2 = l.dot(&l) - tca * tca;
        if d2 > self.radius2 {
            return None;
        } 
        let thc = (self.radius2 - d2).sqrt();
        Some((tca - thc, tca + thc))

    }
}

impl<T> Sub<T> for Val<T> 
where T: Sub<Output = T> {
    type Output = T;
    fn sub(self, rhs: T) -> Self::Output {
        self.0 - rhs
    }
}

fn mix(a: f64, b: f64, mix: f64) -> f64 {
    b * mix + a * (1.0_f64 - mix)
}

pub fn trace(
    rayorig: &Vec3f,
    raydir: &Vec3f,
    spheres: &[Sphere<f64>],
    depth: usize) -> Vec3f {
    let mut tnear = f64::INFINITY;
    let mut sphere:Option<&Sphere<f64>> = None;
    
    for s in spheres {
        if let Some((mut t0, t1)) = s.intersect(rayorig, raydir) {
            if t0 < 0.0 {
                t0 = t1;
            }
            if t0 < tnear {
                tnear = t0;
                sphere = Some(s);
            }
        }
    }

    match sphere {
        None => Vec3f::from_val(2.0),
        Some(sphere) => {
            let mut surface_color = Vec3f::from_val(0_f64);
            let phit = rayorig + &(raydir * tnear);
            let mut nhit = &phit - &sphere.center;
            nhit.normalize();

            let bias = 1e-4_f64;
            let mut inside = false;
            if raydir.dot(&nhit) > 0_f64 {
                nhit = -nhit;
                inside = true;
            }

            if (sphere.transparency > 0_f64 || sphere.reflection > 0_f64)
                && depth < MAX_RAY_DEPTH {
                let facingratio = -raydir.dot(&nhit);
                let fresneleffect = mix((1_f64 - facingratio).powf(3.0_f64), 1_f64, 0.1_f64);
                let mut refldir = raydir - &(&nhit * 2_f64 * raydir.dot(&nhit));
                refldir.normalize();
                let reflection = trace(&(&phit + &(&nhit * bias)), &refldir, spheres, depth + 1);
                let mut refraction = Vec3f::from_val(0_f64);
                if sphere.transparency != 0_f64 {
                    let ior = 1.1_f64;
                    let eta = match inside {
                        true => ior,
                        false => 1_f64 / ior,
                    };
                    let cosi = -nhit.dot(raydir);
                    let k = 1_f64 - eta * eta * (1_f64 - cosi * cosi);
                    let mut refrdir = raydir * eta + &nhit * (eta * cosi - k.sqrt());
                    refrdir.normalize();
                    refraction = trace(&(&phit - &(&nhit * bias)), &refrdir, spheres,depth + 1);
                }
                surface_color = &(reflection * fresneleffect +
                    refraction * (1_f64 - fresneleffect) * sphere.transparency) * &sphere.surface_color;

            } else {
                //println!("else");
                for (i, sphere1) in spheres.into_iter().enumerate() {
                    if sphere1.emission_color.x > 0_f64 {
                        let mut transmission = Vec3f::from_val(1_f64);
                        let mut light_direction = &sphere1.center - &phit;
                        light_direction.normalize();
                        for (j, sphere2) in spheres.into_iter().enumerate() {
                            if i != j {
                                //println!("i:{} != j:{}", i, j);
                                if sphere2.intersect(&(&phit + &(&nhit * bias)), &light_direction).is_some() {
                                    //println!("transmission");
                                    transmission = Vec3f::from_val(0_f64);
                                    break;
                                }
                            }
                        }
                        surface_color += &(&sphere.surface_color * &transmission * 
                        0_f64.max(nhit.dot(&light_direction))) * &sphere1.emission_color;
                    }
                }
            }
            //println!("surface_color:{}\nemission_color:{}\n", &surface_color, &sphere.emission_color);
            &surface_color + &sphere.emission_color
        }
    }

}

pub type Image = Vec<Vec3f>;

pub fn render(spheres: &[Sphere<f64>], width: &usize, height: &usize) -> Image {
    let mut image: Vec<Vec3f> = iter::repeat(Vec3f::new()).take(width * height).collect();
    let inv_width = 1_f64 / *width as f64;
    let inv_height = 1_f64 / *height as f64;
    let fov = 30_f64;
    let aspectratio = *width as f64 / *height as f64;
    let angle = (std::f64::consts::PI * 0.5_f64 * fov / 180.0).tan();
    let mut idx:usize = 0;
    for y in 0..*height {
        for x in 0..*width {
            let xx = (2_f64* ((x as f64 + 0.5_f64) * inv_width) -1_f64) * angle * aspectratio;
            let yy = (1_f64 - 2_f64 * ((y as f64 + 0.5) * inv_height)) * angle;
            let mut raydir = Vec3f::from_3_val(xx, yy, -1_f64);
            raydir.normalize();
            image[idx] = trace(&Vec3f::new(), &raydir, spheres, 0);
            idx += 1;

        }
    }
    image
}

pub fn render_iter(spheres: Arc<Vec<Sphere<f64>>>, width:usize, height:usize) ->  Vec<Box<dyn Fn() -> Vec<u8>>> 

{
    let mut image: Vec<Vec3f> = iter::repeat(Vec3f::new()).take(width * height).collect();
    let inv_width = 1_f64 / width as f64;
    let inv_height = 1_f64 / height as f64;
    let fov = 30_f64;
    let aspectratio = width as f64 / height as f64;
    let angle = (std::f64::consts::PI * 0.5_f64 * fov / 180.0).tan();
    let mut idx:usize = 0;
    let spheres = spheres.clone();
   (0..height).flat_map(move |y| {(0..width).map(move |x| {(x, y)})}).map( move |(x, y)| {
        let spheres = spheres.clone();
        Box::new(move ||{
            let idx = y * width + x;
            let xx = (2_f64* ((x as f64 + 0.5_f64) * inv_width) -1_f64) * angle * aspectratio;
            let yy = (1_f64 - 2_f64 * ((y as f64 + 0.5) * inv_height)) * angle;
            let mut raydir = Vec3f::from_3_val(xx, yy, -1_f64);
            raydir.normalize();
            match trace(&Vec3f::new(), &raydir, &spheres, 0) {
                Vec3f{x, y, z} =>
                vec![(x * 255.0) as u8, 
                     (y * 255.0) as u8,
                     (z * 255.0) as u8
                ] 
            }
            
        }) as Box<dyn Fn() -> Vec<u8>>
        
    }).collect()
}

pub fn render_pararell(spheres: Arc<Vec<Sphere<f64>>>, width:usize, height:usize) -> Vec<u8> {
    let mut image: Vec<Vec3f> = iter::repeat(Vec3f::new()).take(width * height).collect();
    let inv_width = 1_f64 / width as f64;
    let inv_height = 1_f64 / height as f64;
    let fov = 40_f64;
    let aspectratio = width as f64 / height as f64;
    let angle = (std::f64::consts::PI * 0.5_f64 * fov / 180.0).tan();
    let mut idx:usize = 0;
    let spheres = spheres.clone();
    (0..height).flat_map(move |y| {(0..width).map(move |x| {(x, y)})}).collect::<Vec<(usize, usize)>>().par_iter().flat_map(
    //(0..height).collect::<Vec<usize>>().par_iter().flat_map(move |y| {(0..width).map(move |x| {(x, *y)})}).flat_map(
        move |(x, y)| {
            let x = *x;
            let y = *y;
            let idx = y * width + x;
            let xx = (2_f64* ((x as f64 + 0.5_f64) * inv_width) -1_f64) * angle * aspectratio;
            let yy = (1_f64 - 2_f64 * ((y as f64 + 0.5) * inv_height)) * angle;
            let mut raydir = Vec3f::from_3_val(xx, yy, -1_f64);
            raydir.normalize();
            match trace(&Vec3f::new(), &raydir, &spheres, 0) {
                Vec3f{x, y, z} =>
                vec![(x * 255.0) as u8, 
                     (y * 255.0) as u8,
                     (z * 255.0) as u8
                ] 
            }
        }
        ).collect()
}
