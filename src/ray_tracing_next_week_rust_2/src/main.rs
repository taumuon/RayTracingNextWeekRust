use std::time::Instant;
use std::env;
use std::sync::Arc;

use camera::Camera;
use crate::hittable::Hittable;
use crate::material::Material;

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
    let mats: Vec<Arc<dyn Material>>;

    let texture_path = "E:/temp";

    match args[1].as_str() {
        "1" => (camera, world_hittable, mats) = scenes::bouncing_spheres(),
        "2" => (camera, world_hittable, mats) = scenes::checkered_spheres(),
        "3" => (camera, world_hittable, mats) = scenes::earth(texture_path),
        "4" => (camera, world_hittable, mats) = scenes::perlin_spheres(),
        "5" => (camera, world_hittable, mats) = scenes::quads(),
        "6" => (camera, world_hittable, mats) = scenes::simple_light(),
        "7" => (camera, world_hittable, mats) = scenes::cornell_box(),
        "8" => (camera, world_hittable, mats) = scenes::cornell_smoke(),
        "9" => (camera, world_hittable, mats) = scenes::final_scene(texture_path),
        _ => panic!("unrecognised {}", args[1].as_str())
    }

    let now = Instant::now();
    camera.render(&world_hittable, file_name, &mats);

    let elapsed_time = now.elapsed();
    println!("done in {}ms", elapsed_time.as_millis());
}
