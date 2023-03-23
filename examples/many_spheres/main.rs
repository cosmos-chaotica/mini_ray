extern crate fplot;
extern crate mini_ray;
use fplot::*;
use mini_ray::{Sphere, Vec3f, render};

fn main() {
let mut spheres : Vec<Sphere<f64>> = Vec::new();

        spheres.push(Sphere {
                center: Vec3f::from_3_val(0.0, -10004.0, -20.0), radius: 10000.0, radius2: 100000000.0,
                surface_color:Vec3f::from_3_val(0.20, 0.20, 0.20), 
                reflection: 0.0, transparency:0.0, emission_color:Vec3f::new()
            }
        );/*
        spheres.push(Sphere {
                center: Vec3f::from_3_val(0.0, 0.0, -20.0), radius: 4.0, radius2: 16.0,
                surface_color:Vec3f::from_3_val(1.0, 0.32, 0.36), 
                reflection: 1.0, transparency: 0.5, emission_color:Vec3f::new()
            }
        );
        spheres.push(Sphere {
                center:Vec3f::from_3_val(5.0, -1.0, -15.0), radius: 2.0, radius2: 4.0,
                surface_color:Vec3f::from_3_val(0.90, 0.76, 0.46), 
                reflection:1.0, transparency:0.0, emission_color:Vec3f::new()
            }
        );
        spheres.push(Sphere {
                center:Vec3f::from_3_val(5.0, 0.0, -25.0), radius: 3.0, radius2: 9.0,
                surface_color:Vec3f::from_3_val(0.65, 0.77, 0.97), 
                reflection:1.0, transparency:0.0, emission_color:Vec3f::new()
            }
        );
        spheres.push(Sphere {
                center:Vec3f::from_3_val(-5.5, 0.0, -15.0), radius:3.0, radius2: 9.0,
                surface_color:Vec3f::from_3_val(0.90, 0.90, 0.90), 
                reflection:1.0, 
                transparency:0.0, emission_color:Vec3f::new()
            }
        );*/
        let start = (4.0, -2.0, -15.0);
        for i in 0..4 { // x
                for j in 0..4 { // y
                        for k in 0..4 { //z
                            let x = start.0 - 1.7 * i as f64;
                            let y = start.1 + 1.7 * j as f64;
                            let z = start.2 - 1.7 * k as f64;


                            spheres.push(Sphere {
                                center:Vec3f::from_3_val(x, y, z), radius: 1.5, radius2:2.25,
                                surface_color:Vec3f::from_3_val(1.0, 0.32, 0.36),
                                reflection:1.0, transparency: 0.3, emission_color:Vec3f::new()
                            });

                        }
                }
        }

        //light
        spheres.push(Sphere {
                center:Vec3f::from_3_val(0.0, 20.0, -30.0), radius:3.0, radius2:9.0,
                surface_color:Vec3f::from_3_val(0.00, 0.00, 0.00), 
                reflection:0.0, 
                transparency:0.0, emission_color:Vec3f::from_val(3.0)
            }
        );

        let image = render(&spheres,&640,&480);

        let bytes: Vec<u8> = image.into_iter().flat_map(|Vec3f {x, y, z}| {
            [(x * 255.0) as u8, 
             (y * 255.0) as u8,
             (z * 255.0) as u8
            ]
        }).collect();

        let s = Surface::new_with_pixel(String::from("many spheres"), 640, 480);
        s.draw_bytes(bytes);

}