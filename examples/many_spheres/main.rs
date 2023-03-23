extern crate fplot;
extern crate mini_ray;

use fplot::*;
use mini_ray::{Sphere, Vec3f, render, render_iter, render_pararell};
use std::time::{Instant, Duration};
use std::sync::Arc;

fn main() {
let mut spheres : Vec<Sphere<f64>> = Vec::new();

        spheres.push(Sphere {
                center: Vec3f::from_3_val(0.0, -10004.0, -20.0), radius: 10000.0, radius2: 100000000.0,
                surface_color:Vec3f::from_3_val(0.20, 0.20, 0.20), 
                reflection: 0.0, transparency:0.0, emission_color:Vec3f::new()
            }
        );
        let start = (3.0, -3.0, -15.0);
        for i in 0..3 { // x
                for j in 0..3 { // y
                        for k in 0..2 { //z
                            let x = start.0 - 3.0 * i as f64;
                            let y = start.1 + 3.0 * j as f64;
                            let z = start.2 - 3.0 * k as f64;


                            spheres.push(Sphere {
                                center:Vec3f::from_3_val(x, y, z), radius: 1.5, radius2:2.25,
                                surface_color:Vec3f::from_3_val(1.0, 0.32, 0.36),
                                reflection:1.0, transparency: 0.9, emission_color:Vec3f::new()
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
        /* single thread
        let now = Instant::now();
        println!("start at:{:?}", now);
        let image = render(&spheres,&640,&480);
        println!("time elapsed:{}", now.elapsed().as_millis());

        let bytes: Vec<u8> = image.into_iter().flat_map(|Vec3f {x, y, z}| {
            [(x * 255.0) as u8, 
             (y * 255.0) as u8,
             (z * 255.0) as u8
            ]
        }).collect();*/
        /* pararell 
        let now = Instant::now();
        println!("start prepare at:{:?}", now);
        let v = render_iter(Arc::new(spheres), 640, 480);
        println!("prepare time elapsed:{}", now.elapsed().as_millis());
        let now = Instant::now();
        println!("\nstart render at:{:?}", now);
        
        let bytes = (&v as &[_]).par_iter().flat_map(|f: &Box<dyn Fn() -> [u8; 3]> | (*f)()).collect();
        println!("time render elapsed:{}", now.elapsed().as_millis());
        */

        /* pararell2 */
        let now = Instant::now();
        println!("\nstart render at:{:?}", now);
        let bytes = render_pararell(Arc::new(spheres), 640, 480);
        println!("time render elapsed:{}", now.elapsed().as_millis());
        
        let s = Surface::new_with_pixel(String::from("many spheres"), 640, 480);
        s.draw_bytes(bytes);

}