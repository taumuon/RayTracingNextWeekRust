use std::sync::Arc;
use std::path::Path;

use crate::materials::{Lambertian, Dielectric, Metal, DiffuseLight};
use crate::material::Material;
use crate::hittable::Hittable;
use crate::hittables::Hittables;
use crate::vec3::Vec3;
use crate::rtweekend::{self, random_double_min_max};
use crate::sphere::Sphere;
use crate::bvhnode::BvhNode;
use crate::texture::Texture;
use crate::textures::{CheckerTexture, SolidColorTexture, NoiseTexture};
use crate::imagetexture::ImageTexture;
use crate::camera::Camera;
use crate::quad::Quad;
use crate::translate::Translate;
use crate::rotatey::RotateY;
use crate::constantmedium::ConstantMedium;

pub fn final_scene(texture_path: &str) -> (Camera, Box<dyn Hittable>) {
    let aspect_ratio = 1.0;
    let image_width = 300;
    let vfov = 40.0;
    let look_from = Vec3::new(478.0, 278.0, -600.0);
    let look_at = Vec3::new(278.0, 278.0, 0.0);
    let v_up = Vec3::new(0.0, 1.0, 0.0);
    let defocus_angle = 0.0;
    let camera = Camera::new(aspect_ratio, image_width, vfov, look_from, look_at, v_up, defocus_angle);

    let mut world: Vec<Arc<dyn Hittable>> = Vec::new();

    let ground: Arc<dyn Texture> = Arc::new(SolidColorTexture::new(Vec3::new(0.48, 0.83, 0.53)));
    let ground_material: Arc<dyn Material> = Arc::new(Lambertian::new(Arc::clone(&ground)));

    let mut boxes: Vec<Arc<dyn Hittable>> = Vec::new();

    let boxes_per_side = 20;
    let w = 100.0;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let x0 = -1000.0 + (i as f64 * w);
            let z0 = -1000.0 + (j as f64 * w);
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = random_double_min_max(1.0, 101.0);
            let z1 = z0 + w;

            let the_box: Arc<dyn Hittable> = Arc::new(box3(Vec3::new(x0, y0, z0), Vec3::new(x1, y1, z1), Arc::clone(&ground_material)));
            boxes.push(the_box);
        }
    }

    let boxes_bvh = BvhNode::new(&boxes[..]);
    let boxes_hittable: Arc<dyn Hittable> = Arc::new(boxes_bvh);

    world.push(boxes_hittable);

    let light_tex: Arc<dyn Texture> = Arc::new(SolidColorTexture::new(Vec3::new(7.0, 7.0, 7.0)));
    let light_mat: Arc<dyn Material> = Arc::new(DiffuseLight::new(Arc::clone(&light_tex)));

    let quad1: Arc<dyn Hittable> = Arc::new(Quad::new(Vec3::new(123.0, 554.0, 147.0), Vec3::new(300.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 265.0), Arc::clone(&light_mat)));
    world.push(quad1);

    let center1 = Vec3::new(400.0, 400.0, 200.0);
    let center2 = center1 + Vec3::new(30.0, 0.0, 0.0);

    let sphere_tex: Arc<dyn Texture> = Arc::new(SolidColorTexture::new(Vec3::new(0.7, 0.3, 0.1)));
    let sphere_material: Arc<dyn Material> = Arc::new(Lambertian::new(Arc::clone(&sphere_tex)));
    let sphere_1: Arc<dyn Hittable> = Arc::new(Sphere::new_moving(center1, center2, 50.0, Arc::clone(&sphere_material)));
    world.push(sphere_1);

    let dielectric_mat: Arc<dyn Material> = Arc::new(Dielectric::new(1.5));
    let sphere_2: Arc<dyn Hittable> = Arc::new(Sphere::new(Vec3::new(260.0, 150.0, 45.0), 50.0, Arc::clone(&dielectric_mat)));
    world.push(sphere_2);

    let sphere_material3: Arc<dyn Material> = Arc::new(Metal::new(Vec3::new(0.8, 0.8, 0.9),1.0));
    let sphere_3: Arc<dyn Hittable> = Arc::new(Sphere::new(Vec3::new(0.0, 150.0, 145.0), 50.0, Arc::clone(&sphere_material3)));
    world.push(sphere_3);

    let boundary: Arc<dyn Hittable> = Arc::new(Sphere::new(Vec3::new(360.0, 150.0, 145.0), 70.0, Arc::clone(&dielectric_mat)));
    let constant_medium: Arc<dyn Hittable> = Arc::new(ConstantMedium::new(Arc::clone(&boundary), Arc::new(SolidColorTexture::new(Vec3::new(0.2, 0.4, 0.9))), 0.2));
    world.push(boundary);
    world.push(constant_medium);
    let boundary_2: Arc<dyn Hittable> = Arc::new(Sphere::new(Vec3::new(0.0, 0.0, 0.0), 5000.0, Arc::clone(&dielectric_mat)));
    let constant_medium2: Arc<dyn Hittable> = Arc::new(ConstantMedium::new(Arc::clone(&boundary_2), Arc::new(SolidColorTexture::new(Vec3::new(1.0, 1.0, 1.0))), 0.0001));
    world.push(boundary_2);
    world.push(constant_medium2);

    let earth_texture_path = Path::new(texture_path).join("earthmap.jpg");

    let earth_texture: Arc<dyn Texture> = Arc::new(ImageTexture::new(earth_texture_path.to_str().unwrap()));
    let earth_material: Arc<dyn Material> = Arc::new(Lambertian::new(Arc::clone(&earth_texture)));
    let globe : Arc<dyn Hittable> = Arc::new(Sphere::new(Vec3::new(400.0, 200.0, 400.0), 100.0, Arc::clone(&earth_material)));
    world.push(globe);

    let per_tex: Arc<dyn Texture> = Arc::new(NoiseTexture::new(0.2));
    let per_material: Arc<dyn Material> = Arc::new(Lambertian::new(Arc::clone(&per_tex)));
    let sphere_per: Arc<dyn Hittable> = Arc::new(Sphere::new(Vec3::new(220.0, 280.0, 300.0), 80.0, Arc::clone(&per_material)));
    world.push(sphere_per);

    let mut spheres: Vec<Arc<dyn Hittable>> = Vec::new();
    let spheres_tex: Arc<dyn Texture> = Arc::new(SolidColorTexture::new(Vec3::new(0.73, 0.73, 0.73)));
    let spheres_material: Arc<dyn Material> = Arc::new(Lambertian::new(Arc::clone(&spheres_tex)));
    for i in 0..10 {
        for j in 0..10 {
            for k in 0..10 {
                let sph: Arc<dyn Hittable> = Arc::new(Sphere::new(Vec3::new(30.01 * (i as f64), 30.02 * (j as f64), 30.03 * (k as f64)), 10.0, Arc::clone(&spheres_material)));
                spheres.push(sph);
            }
        }
    }

    let spheres_hittables_bvh: Arc<dyn Hittable> = Arc::new(BvhNode::new(&spheres[..]));
    let spheres_hittables_bvh = Arc::new(RotateY::new(15.0, spheres_hittables_bvh));
    let spheres_hittables_bvh = Arc::new(Translate::new(Vec3::new(-100.0, 270.0, 395.0), spheres_hittables_bvh));
    world.push(spheres_hittables_bvh);

    // TODO: fix stackoverflow with empty world
    let hittables = Hittables::new(&world[..]);
    let world_hittable: Box<dyn Hittable> = Box::new(hittables);

    return (camera, world_hittable);
}

pub fn cornell_smoke() -> (Camera, Box<dyn Hittable>) {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let vfov = 40.0;
    let look_from = Vec3::new(278.0, 278.0, -800.0);
    let look_at = Vec3::new(278.0, 278.0, 0.0);
    let v_up = Vec3::new(0.0, 1.0, 0.0);
    let defocus_angle = 0.0;
    let camera = Camera::new(aspect_ratio, image_width, vfov, look_from, look_at, v_up, defocus_angle);

    let mut world: Vec<Arc<dyn Hittable>> = Vec::new();

    let red: Arc<dyn Texture> = Arc::new(SolidColorTexture::new(Vec3::new(0.65, 0.05, 0.05)));
    let red_material: Arc<dyn Material> = Arc::new(Lambertian::new(Arc::clone(&red)));
    let white: Arc<dyn Texture> = Arc::new(SolidColorTexture::new(Vec3::new(0.73, 0.73, 0.73)));
    let white_material: Arc<dyn Material> = Arc::new(Lambertian::new(Arc::clone(&white)));
    let green: Arc<dyn Texture> = Arc::new(SolidColorTexture::new(Vec3::new(0.12, 0.45, 0.15)));
    let green_material: Arc<dyn Material> = Arc::new(Lambertian::new(Arc::clone(&green)));

    let light_tex: Arc<dyn Texture> = Arc::new(SolidColorTexture::new(Vec3::new(7.0, 7.0, 7.0)));
    let light_mat: Arc<dyn Material> = Arc::new(DiffuseLight::new(Arc::clone(&light_tex)));

    let quad1: Arc<dyn Hittable> = Arc::new(Quad::new(Vec3::new(555.0, 0.0, 0.0), Vec3::new(0.0, 555.0, 0.0), Vec3::new(0.0, 0.0, 555.0), Arc::clone(&green_material)));
    world.push(quad1);
    let quad2: Arc<dyn Hittable> = Arc::new(Quad::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 555.0, 0.0), Vec3::new(0.0, 0.0, 555.0), Arc::clone(&red_material)));
    world.push(quad2);
    let quad3: Arc<dyn Hittable> = Arc::new(Quad::new(Vec3::new(113.0, 554.0, 127.0), Vec3::new(333.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 305.0), Arc::clone(&light_mat)));
    world.push(quad3);
    let quad4: Arc<dyn Hittable> = Arc::new(Quad::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(555.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 555.0), Arc::clone(&white_material)));
    world.push(quad4);
    let quad5: Arc<dyn Hittable> = Arc::new(Quad::new(Vec3::new(555.0, 555.0, 555.0), Vec3::new(-555.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -555.0), Arc::clone(&white_material)));
    world.push(quad5);
    let quad6: Arc<dyn Hittable> = Arc::new(Quad::new(Vec3::new(0.0, 0.0, 555.0), Vec3::new(555.0, 0.0, 0.0), Vec3::new(0.0, 555.0, 0.0), Arc::clone(&white_material)));
    world.push(quad6);

    let box1: Arc<dyn Hittable> = Arc::new(box3(Vec3::new(0.0, 0.0, 0.0), Vec3::new(165.0, 330.0, 165.0), Arc::clone(&white_material)));
    let box1: Arc<dyn Hittable> = Arc::new(RotateY::new(15.0, box1));
    let box1: Arc<dyn Hittable> = Arc::new(Translate::new(Vec3::new(265.0, 0.0, 295.0), box1));
    let box1: Arc<dyn Hittable> = Arc::new(ConstantMedium::new(box1, Arc::new(SolidColorTexture::new(Vec3::new(0.0, 0.0, 0.0))), 0.01));
    world.push(box1);
    let box2: Arc<dyn Hittable> = Arc::new(box3(Vec3::new(0.0, 0.0, 0.0), Vec3::new(165.0, 165.0, 165.0), Arc::clone(&white_material)));
    let box2: Arc<dyn Hittable> = Arc::new(RotateY::new(-18.0, box2));
    let box2: Arc<dyn Hittable> = Arc::new(Translate::new(Vec3::new(130.0, 0.0, 65.0), box2));
    let box2: Arc<dyn Hittable> = Arc::new(ConstantMedium::new(box2, Arc::new(SolidColorTexture::new(Vec3::new(1.0, 1.0, 1.0))), 0.01));
    world.push(box2);

    // let hittables = Hittables::new(&world[..]);
    let hittables = BvhNode::new(&world[..]);
    let world_hittable: Box<dyn Hittable> = Box::new(hittables);

    return (camera, world_hittable);
}

pub fn cornell_box() -> (Camera, Box<dyn Hittable>) {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let vfov = 40.0;
    let look_from = Vec3::new(278.0, 278.0, -800.0);
    let look_at = Vec3::new(278.0, 278.0, 0.0);
    let v_up = Vec3::new(0.0, 1.0, 0.0);
    let defocus_angle = 0.0;
    let camera = Camera::new(aspect_ratio, image_width, vfov, look_from, look_at, v_up, defocus_angle);

    let mut world: Vec<Arc<dyn Hittable>> = Vec::new();

    let red: Arc<dyn Texture> = Arc::new(SolidColorTexture::new(Vec3::new(0.65, 0.05, 0.05)));
    let red_material: Arc<dyn Material> = Arc::new(Lambertian::new(Arc::clone(&red)));
    let white: Arc<dyn Texture> = Arc::new(SolidColorTexture::new(Vec3::new(0.73, 0.73, 0.73)));
    let white_material: Arc<dyn Material> = Arc::new(Lambertian::new(Arc::clone(&white)));
    let green: Arc<dyn Texture> = Arc::new(SolidColorTexture::new(Vec3::new(0.12, 0.45, 0.15)));
    let green_material: Arc<dyn Material> = Arc::new(Lambertian::new(Arc::clone(&green)));

    let light_tex: Arc<dyn Texture> = Arc::new(SolidColorTexture::new(Vec3::new(15.0, 15.0, 15.0)));
    let light_mat: Arc<dyn Material> = Arc::new(DiffuseLight::new(Arc::clone(&light_tex)));

    let quad1: Arc<dyn Hittable> = Arc::new(Quad::new(Vec3::new(555.0, 0.0, 0.0), Vec3::new(0.0, 555.0, 0.0), Vec3::new(0.0, 0.0, 555.0), Arc::clone(&green_material)));
    world.push(quad1);
    let quad2: Arc<dyn Hittable> = Arc::new(Quad::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 555.0, 0.0), Vec3::new(0.0, 0.0, 555.0), Arc::clone(&red_material)));
    world.push(quad2);
    let quad3: Arc<dyn Hittable> = Arc::new(Quad::new(Vec3::new(343.0, 554.0, 332.0), Vec3::new(-130.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -105.0), Arc::clone(&light_mat)));
    world.push(quad3);
    let quad4: Arc<dyn Hittable> = Arc::new(Quad::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(555.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 555.0), Arc::clone(&white_material)));
    world.push(quad4);
    let quad5: Arc<dyn Hittable> = Arc::new(Quad::new(Vec3::new(555.0, 555.0, 555.0), Vec3::new(-555.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -555.0), Arc::clone(&white_material)));
    world.push(quad5);
    let quad6: Arc<dyn Hittable> = Arc::new(Quad::new(Vec3::new(0.0, 0.0, 555.0), Vec3::new(555.0, 0.0, 0.0), Vec3::new(0.0, 555.0, 0.0), Arc::clone(&white_material)));
    world.push(quad6);

    let box1: Arc<dyn Hittable> = Arc::new(box3(Vec3::new(0.0, 0.0, 0.0), Vec3::new(165.0, 330.0, 165.0), Arc::clone(&white_material)));
    let box1: Arc<dyn Hittable> = Arc::new(RotateY::new(15.0, box1));
    let box1: Arc<dyn Hittable> = Arc::new(Translate::new(Vec3::new(265.0, 0.0, 295.0), box1));
    world.push(box1);
    let box2: Arc<dyn Hittable> = Arc::new(box3(Vec3::new(0.0, 0.0, 0.0), Vec3::new(165.0, 165.0, 165.0), Arc::clone(&white_material)));
    let box2: Arc<dyn Hittable> = Arc::new(RotateY::new(-18.0, box2));
    let box2: Arc<dyn Hittable> = Arc::new(Translate::new(Vec3::new(130.0, 0.0, 65.0), box2));
    world.push(box2);

    // let hittables = Hittables::new(&world[..]);
    let hittables = BvhNode::new(&world[..]);
    let world_hittable: Box<dyn Hittable> = Box::new(hittables);

    return (camera, world_hittable);
}

pub fn simple_light() -> (Camera, Box<dyn Hittable>) {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let vfov = 20.0;
    let look_from = Vec3::new(26.0, 3.0, 6.0);
    let look_at = Vec3::new(0.0, 2.0, 0.0);
    let v_up = Vec3::new(0.0, 1.0, 0.0);
    let defocus_angle = 0.0;
    let camera = Camera::new(aspect_ratio, image_width, vfov, look_from, look_at, v_up, defocus_angle);

    let mut world: Vec<Arc<dyn Hittable>> = Vec::new();

    let per_tex: Arc<dyn Texture> = Arc::new(NoiseTexture::new(4.0));
    let per_material: Arc<dyn Material> = Arc::new(Lambertian::new(Arc::clone(&per_tex)));
    let sphere_1: Arc<dyn Hittable> = Arc::new(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, Arc::clone(&per_material)));
    let sphere_2: Arc<dyn Hittable> = Arc::new(Sphere::new(Vec3::new(0.0, 2.0, 0.0), 2.0, Arc::clone(&per_material)));
    world.push(sphere_1);
    world.push(sphere_2);

    let light_tex: Arc<dyn Texture> = Arc::new(SolidColorTexture::new(Vec3::new(4.0, 4.0, 4.0)));
    let light_mat: Arc<dyn Material> = Arc::new(DiffuseLight::new(Arc::clone(&light_tex)));
    let quad: Arc<dyn Hittable> = Arc::new(Quad::new(Vec3::new(3.0, 1.0, -2.0), Vec3::new(2.0, 0.0, 0.0), Vec3::new(0.0, 2.0, 0.0), Arc::clone(&light_mat)));
    world.push(quad);

    // let hittables = Hittables::new(&world[..]);
    let hittables = BvhNode::new(&world[..]);
    let world_hittable: Box<dyn Hittable> = Box::new(hittables);

    return (camera, world_hittable);
}

pub fn quads() -> (Camera, Box<dyn Hittable>) {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let vfov = 80.0;
    let look_from = Vec3::new(0.0, 0.0, 9.0);
    let look_at = Vec3::new(0.0, 0.0, 0.0);
    let v_up = Vec3::new(0.0, 1.0, 0.0);
    let defocus_angle = 0.0;
    let camera = Camera::new(aspect_ratio, image_width, vfov, look_from, look_at, v_up, defocus_angle);

    let mut world: Vec<Arc<dyn Hittable>> = Vec::new();

    let red: Arc<dyn Texture> = Arc::new(SolidColorTexture::new(Vec3::new(1.0, 0.2, 0.2)));
    let left_material: Arc<dyn Material> = Arc::new(Lambertian::new(Arc::clone(&red)));
    let green: Arc<dyn Texture> = Arc::new(SolidColorTexture::new(Vec3::new(0.2, 1.0, 0.2)));
    let back_material: Arc<dyn Material> = Arc::new(Lambertian::new(Arc::clone(&green)));
    let blue: Arc<dyn Texture> = Arc::new(SolidColorTexture::new(Vec3::new(0.2, 0.2, 1.0)));
    let right_material: Arc<dyn Material> = Arc::new(Lambertian::new(Arc::clone(&blue)));
    let orange: Arc<dyn Texture> = Arc::new(SolidColorTexture::new(Vec3::new(1.0,0.5, 0.0)));
    let upper_material: Arc<dyn Material> = Arc::new(Lambertian::new(Arc::clone(&orange)));
    let teal: Arc<dyn Texture> = Arc::new(SolidColorTexture::new(Vec3::new(0.2,0.8, 0.8)));
    let lower_material: Arc<dyn Material> = Arc::new(Lambertian::new(Arc::clone(&teal)));

    let quad1: Arc<dyn Hittable> = Arc::new(Quad::new(Vec3::new(-3.0, -2.0, 5.0), Vec3::new(0.0, 0.0, -4.0), Vec3::new(0.0, 4.0, 0.0), Arc::clone(&left_material)));
    let quad2: Arc<dyn Hittable> = Arc::new(Quad::new(Vec3::new(-2.0, -2.0, 0.0), Vec3::new(4.0, 0.0, 0.0), Vec3::new(0.0, 4.0, 0.0), Arc::clone(&back_material)));
    let quad3: Arc<dyn Hittable> = Arc::new(Quad::new(Vec3::new(3.0, -2.0, 1.0), Vec3::new(0.0, 0.0, 4.0), Vec3::new(0.0, 4.0, 0.0), Arc::clone(&right_material)));
    let quad4: Arc<dyn Hittable> = Arc::new(Quad::new(Vec3::new(-2.0, 3.0, 1.0), Vec3::new(4.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 4.0), Arc::clone(&upper_material)));
    let quad5: Arc<dyn Hittable> = Arc::new(Quad::new(Vec3::new(-2.0, -3.0, 5.0), Vec3::new(4.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -4.0), Arc::clone(&lower_material)));

    world.push(quad1);
    world.push(quad2);
    world.push(quad3);
    world.push(quad4);
    world.push(quad5);

    // let hittables = Hittables::new(&world[..]);
    let hittables = BvhNode::new(&world[..]);
    let world_hittable: Box<dyn Hittable> = Box::new(hittables);

    return (camera, world_hittable);
}

pub fn perlin_spheres() -> (Camera, Box<dyn Hittable>) {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let vfov = 20.0;
    let look_from = Vec3::new(13.0, 2.0, 3.0);
    let look_at = Vec3::new(0.0, 0.0, 0.0);
    let v_up = Vec3::new(0.0, 1.0, 0.0);
    let defocus_angle = 0.0;
    let camera = Camera::new(aspect_ratio, image_width, vfov, look_from, look_at, v_up, defocus_angle);

    let mut world: Vec<Arc<dyn Hittable>> = Vec::new();

    let perlin_texture: Arc<dyn Texture> = Arc::new(NoiseTexture::new(1.0));
    let perlin_material: Arc<dyn Material> = Arc::new(Lambertian::new(Arc::clone(&perlin_texture)));

    let sphere_1 : Arc<dyn Hittable> = Arc::new(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, Arc::clone(&perlin_material)));
    world.push(sphere_1);
    let sphere_2 : Arc<dyn Hittable> = Arc::new(Sphere::new(Vec3::new(0.0, 2.0, 0.0), 2.0, Arc::clone(&perlin_material)));
    world.push(sphere_2);

    let hittables = Hittables::new(&world[..]);
    let world_hittable: Box<dyn Hittable> = Box::new(hittables);

    return (camera, world_hittable);
}

pub fn earth(texture_path: &str) -> (Camera, Box<dyn Hittable>) {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let vfov = 20.0;
    let look_from = Vec3::new(0.0, 0.0, 12.0);
    let look_at = Vec3::new(0.0, 0.0, 0.0);
    let v_up = Vec3::new(0.0, 1.0, 0.0);
    let defocus_angle = 0.0;
    let camera = Camera::new(aspect_ratio, image_width, vfov, look_from, look_at, v_up, defocus_angle);

    let mut world: Vec<Arc<dyn Hittable>> = Vec::new();

    let earth_texture_path = Path::new(texture_path).join("earthmap.jpg");
    let earth_texture: Arc<dyn Texture> = Arc::new(ImageTexture::new(earth_texture_path.to_str().unwrap()));
    let earth_material: Arc<dyn Material> = Arc::new(Lambertian::new(Arc::clone(&earth_texture)));

    let globe : Arc<dyn Hittable> = Arc::new(Sphere::new(Vec3::new(0.0, 0.0, 0.0), 2.0, Arc::clone(&earth_material)));
    world.push(globe);

    // let hittables = Hittables::new(&world[..]);
    let hittables = BvhNode::new(&world[..]);
    let world_hittable: Box<dyn Hittable> = Box::new(hittables);

    return (camera, world_hittable);
}

pub fn checkered_spheres() -> (Camera, Box<dyn Hittable>) {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let vfov = 20.0;
    // let look_from = Vec3::new(13.0, 2.0, 3.0);
    // let look_at = Vec3::new(0.0, 0.0, 0.0);
    let look_from = Vec3::new(0.0, 0.0, 12.0);
    let look_at = Vec3::new(0.0, 0.0, 0.0);
    let v_up = Vec3::new(0.0, 1.0, 0.0);
    let defocus_angle = 0.0;
    let camera = Camera::new(aspect_ratio, image_width, vfov, look_from, look_at, v_up, defocus_angle);

    let mut world: Vec<Arc<dyn Hittable>> = Vec::new();

    let even = Arc::new(SolidColorTexture::new(Vec3::new(0.2, 0.3, 0.1)));
    let odd = Arc::new(SolidColorTexture::new(Vec3::new(0.9, 0.9, 0.9)));
    let checker: Arc<dyn Texture> = Arc::new(CheckerTexture::new(0.32, even, odd));
    let checker_material: Arc<dyn Material> = Arc::new(Lambertian::new(Arc::clone(&checker)));

    let checker_sphere_1 : Arc<dyn Hittable> = Arc::new(Sphere::new(Vec3::new(0.0, -2.0, 0.0), 2.0, Arc::clone(&checker_material)));
    world.push(checker_sphere_1);

    let checker_sphere_2 : Arc<dyn Hittable> = Arc::new(Sphere::new(Vec3::new(0.0, 2.0, 0.0), 2.0, Arc::clone(&checker_material)));
    world.push(checker_sphere_2);

    // let hittables = Hittables::new(&world[..]);
    let hittables = BvhNode::new(&world[..]);
    let world_hittable: Box<dyn Hittable> = Box::new(hittables);

    return (camera, world_hittable);
}

pub fn bouncing_spheres() -> (Camera, Box<dyn Hittable>) {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let vfov = 20.0;
    let look_from = Vec3::new(13.0, 2.0, 3.0);
    let look_at = Vec3::new(0.0, 0.0, 0.0);
    let v_up = Vec3::new(0.0, 1.0, 0.0);
    let defocus_angle = 0.6;
    let camera = Camera::new(aspect_ratio, image_width, vfov, look_from, look_at, v_up, defocus_angle);

    let mut materials: Vec<Arc<dyn Material>> = Vec::new();

    let material_ground: Arc<dyn Material> = Arc::new(Lambertian::new(Arc::new(SolidColorTexture::new(Vec3::new(0.5, 0.5, 0.5)))));
    let material_1: Arc<dyn Material> = Arc::new(Dielectric::new(1.5));
    let material_2: Arc<dyn Material> = Arc::new(Lambertian::new(Arc::new(SolidColorTexture::new(Vec3::new(0.4, 0.2, 0.1)))));
    let material_3: Arc<dyn Material> = Arc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0));

    let mut world: Vec<Arc<dyn Hittable>> = Vec::new();

    let even = Arc::new(SolidColorTexture::new(Vec3::new(0.2, 0.3, 0.1)));
    let odd = Arc::new(SolidColorTexture::new(Vec3::new(0.9, 0.9, 0.9)));
    let checker: Arc<dyn Texture> = Arc::new(CheckerTexture::new(0.32, even, odd));
    let checker_material: Arc<dyn Material> = Arc::new(Lambertian::new(Arc::clone(&checker)));
    let checker_sphere : Arc<dyn Hittable> = Arc::new(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, Arc::clone(&checker_material)));
    world.push(checker_sphere);

    let hittable: Arc<dyn Hittable> = Arc::new(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, Arc::clone(&material_ground)));
    world.push(hittable);

    let hittable1: Arc<dyn Hittable> = Arc::new(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, Arc::clone(&material_1)));
    world.push(hittable1);

    let hittable2: Arc<dyn Hittable> = Arc::new(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, Arc::clone(&material_2)));
    world.push(hittable2);

    let hittable3: Arc<dyn Hittable> = Arc::new(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, Arc::clone(&material_3)));
    world.push(hittable3);

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rtweekend::random_double();
            let center = Vec3::new((a as f64) + 0.9 * rtweekend::random_double(), 0.2, (b as f64) + 0.9 * rtweekend::random_double());

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = rtweekend::random_vec3() * rtweekend::random_vec3();
                    let mat: Arc<dyn Material> = Arc::new(Lambertian::new(Arc::new(SolidColorTexture::new(albedo))));
                    materials.push(Arc::clone(&mat));

                    let center_2 = center + Vec3::new(0.0, random_double_min_max(0.0, 0.5), 0.0);
                    let sphere = Sphere::new_moving(center, center_2, 0.2, Arc::clone(&mat));
                    let hittable: Arc<dyn Hittable> = Arc::new(sphere);
                    world.push(hittable);
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = rtweekend::random_vec3_min_max(0.5, 1.0);
                    let fuzz = rtweekend::random_double_min_max(0.0, 0.5);
                    let mat: Arc<dyn Material> = Arc::new(Metal::new(albedo, fuzz));
                    materials.push(Arc::clone(&mat));

                    let hittable: Arc<dyn Hittable> = Arc::new(Sphere::new(center, 0.2, Arc::clone(&mat)));
                    world.push(hittable);
                } else {
                    // glass
                    let mat: Arc<dyn Material> = Arc::new(Dielectric::new(1.5));
                    materials.push(Arc::clone(&mat));

                    let hittable: Arc<dyn Hittable> = Arc::new(Sphere::new(center, 0.2, Arc::clone(&mat)));
                    world.push(hittable);
                }
            }
        }
    }

    // let hittables = Hittables::new(&world[..]);
    let hittables = BvhNode::new(&world[..]);
    let world_hittable: Box<dyn Hittable> = Box::new(hittables);

    return (camera, world_hittable);
}

fn box3(a: Vec3, b: Vec3, mat: Arc<dyn Material>) -> Hittables {
    // Returns the 3D box (six sides) that contains the two opposite vertices a & b.

    let mut sides: Vec<Arc<dyn Hittable>> = Vec::new();
    
    // Construct the two opposite vertices with the minimum and maximum coordinates.
    let min = Vec3::new(a.x.min(b.x), a.y.min(b.y), a.z.min(b.z));
    let max = Vec3::new(a.x.max(b.x), a.y.max(b.y), a.z.max(b.z));

    let dx = Vec3::new(max.x - min.x, 0.0, 0.0);
    let dy = Vec3::new(0.0, max.y - min.y, 0.0);
    let dz = Vec3::new(0.0, 0.0, max.z - min.z);

    let quad1: Arc<dyn Hittable> = Arc::new(Quad::new(Vec3::new(min.x, min.y, max.z), dx, dy, Arc::clone(&mat)));  // front
    let quad2: Arc<dyn Hittable> = Arc::new(Quad::new(Vec3::new(max.x, min.y, max.z), -dz, dy, Arc::clone(&mat))); // right
    let quad3: Arc<dyn Hittable> = Arc::new(Quad::new(Vec3::new(max.x, min.y, min.z), -dx, dy, Arc::clone(&mat))); // back
    let quad4: Arc<dyn Hittable> = Arc::new(Quad::new(Vec3::new(min.x, min.y, min.z), dz, dy, Arc::clone(&mat)));  // left
    let quad5: Arc<dyn Hittable> = Arc::new(Quad::new(Vec3::new(min.x, max.y, max.z), dx, -dz, Arc::clone(&mat))); // top
    let quad6: Arc<dyn Hittable> = Arc::new(Quad::new(Vec3::new(min.x, min.y, min.z), dx, dz, Arc::clone(&mat)));  // bottom

    sides.push(quad1);
    sides.push(quad2);
    sides.push(quad3);
    sides.push(quad4);
    sides.push(quad5);
    sides.push(quad6);

    let hittables = Hittables::new(&sides[..]);
    return hittables;
}