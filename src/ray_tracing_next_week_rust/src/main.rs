use camera::Camera;
use std::time::Instant;
use std::env;
use crate::hittable::Hittable;

mod vec3;
mod ray;
mod sphere;
mod hittable;
mod hittables;
mod interval;
mod rtweekend;
mod camera;
mod hitrecord;
mod material;
mod materials;
mod scenes;
mod aabb;
mod bvhnode;
mod texture;
mod textures;
mod imagetexture;
mod perlin;
mod quad;
mod translate;
mod rotatey;
mod constantmedium;

fn main() {
    let file_name = "e:\\temp\\render\\test.ppm";
    
    let args: Vec<String> = env::args().collect();

    let camera: Camera;
    let world_hittable: Box<dyn Hittable>;
    
    let texture_path = "E:/temp";

    match args[1].as_str() {
        "1" => (camera, world_hittable) = scenes::bouncing_spheres(),
        "2" => (camera, world_hittable) = scenes::checkered_spheres(),
        "3" => (camera, world_hittable) = scenes::earth(texture_path),
        "4" => (camera, world_hittable) = scenes::perlin_spheres(),
        "5" => (camera, world_hittable) = scenes::quads(),
        "6" => (camera, world_hittable) = scenes::simple_light(),
        "7" => (camera, world_hittable) = scenes::cornell_box(),
        "8" => (camera, world_hittable) = scenes::cornell_smoke(),
        "9" => (camera, world_hittable) = scenes::final_scene(texture_path),
        _ => panic!("unrecognised {}", args[1].as_str())
    }

    let now = Instant::now();
    camera.render(&world_hittable, file_name);

    let elapsed_time = now.elapsed();
    println!("done in {}ms", elapsed_time.as_millis());
}
