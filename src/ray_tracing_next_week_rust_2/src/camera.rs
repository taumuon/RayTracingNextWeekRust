use std::fs::File;
use std::io::prelude::*;
use std::io::Result;
use std::cmp::max;
use std::sync::Arc;
use rayon::prelude::*;

use crate::ray::Ray;
use crate::hittable::Hittable;
use crate::material::Material;
use crate::rtweekend::{degrees_to_radians, random_in_unit_disc};
use crate::vec3;
use crate::vec3::Vec3;
use crate::interval::Interval;
use crate::rtweekend;

#[derive(Copy, Clone, Debug)]
pub struct Camera {
    pub image_width: u64,     // Rendered image width in pixel count

    samples_per_pixel: u64,
    image_height: u64,        // Rendered image height
    camera_center: Vec3,
    pixel00_loc: Vec3,        // Location of pixel 0, 0
    pixel_delta_u: Vec3,      // Offset of pixel to the right
    pixel_delta_v: Vec3,      // Offset to pixel below
    pixel_samples_scale: f64, // Color scale factor for a sum of pixel samples
    max_depth: u16,
    defocus_angle: f64,
    defocus_disc_u: Vec3, // Defocus disk horizontal radius
    defocus_disc_v: Vec3 // Defocus disk vertical radius
}

impl Camera {
    pub fn new
    (
        aspect_ratio: f64, // Ratio of image width over height
        image_width: u64,  // Rendered image width in pixel count
        vfov: f64,         // Vertical view angle (field of view)
        look_from: Vec3,
        look_at: Vec3,
        v_up: Vec3,
        defocus_angle: f64 // Variation angle of rays through each pixel
    ) -> Self {
        let image_height = max(1, (image_width as f64 / aspect_ratio) as u64);

        let focus_dist = 10.0;  // Distance from camera lookfrom point to plane of perfect focus

        // Determine viewport dimensions.
        let theta = rtweekend::degrees_to_radians(vfov);
        let h = f64::tan(theta / 2.0);

        let camera_center = look_from;

        let viewport_height = 2.0 * h * focus_dist;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        // Calculate the u,v,w unit basis vectors for the camera coordinate frame.
        let w = (look_from - look_at).normalized();
        let u = vec3::cross(&v_up, &w).normalized();
        let v = vec3::cross(&w, &u);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = viewport_width * u;  // Vector across viewport horizontal edge
        let viewport_v = viewport_height * -v;  // Vector down viewport vertical edge

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left = camera_center
        - (focus_dist * w)
        - viewport_u / 2.0
        - viewport_v / 2.0;

        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        let samples_per_pixel = 80; // Count of random samples for each pixel
        let pixel_samples_scale = 1.0 / samples_per_pixel as f64;

        let defocus_radius = focus_dist * f64::tan(degrees_to_radians(defocus_angle / 2.0));
        let defocus_disc_u = u * defocus_radius;
        let defocus_disc_v = v * defocus_radius;

        Self {
            samples_per_pixel: samples_per_pixel,
            image_width: image_width,
            image_height: image_height,
            camera_center: camera_center,
            pixel00_loc: pixel00_loc,
            pixel_delta_u: pixel_delta_u,
            pixel_delta_v: pixel_delta_v,
            pixel_samples_scale : pixel_samples_scale,
            max_depth: 2,  // Maximum number of ray bounces into scene
            defocus_angle,
            defocus_disc_u,
            defocus_disc_v
        }
    }

    pub fn render (&self, world: &Box<dyn Hittable>, file_name: &str, mats: &Vec<Arc<dyn Material>>) {
        let mut contents = String::new();

        contents.push_str(&format!("P3\n{} {}\n255\n", self.image_width, self.image_height));

        let mut rows =(0..self.image_height).into_par_iter().map(|row|
            {
                let mut row_pixels: Vec<Vec3> = Vec::new();
                for col in 0 .. self.image_width {
                    let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);

                    for _sample in 0 .. self.samples_per_pixel {
                        let ray_r = Self::get_ray(&self, col as f64, row as f64);
                        pixel_color = pixel_color + Self::ray_color(&ray_r, self.max_depth, &world, mats);
                    }

                    row_pixels.push(pixel_color);
                }
                return (row, row_pixels);
            }).collect::<Vec<(u64, Vec<Vec3>)>>();

        rows.sort_by(|a, b| {
            return a.0.partial_cmp(&b.0).unwrap();
        });

        for row in rows.into_iter() {
            for pixel_color in row.1.into_iter() {
                write_color(&mut contents, &(self.pixel_samples_scale * pixel_color));
            }
        }

        write_file(file_name, &contents).expect("Unable to write to file");
    }

    fn get_ray(&self, i: f64, j: f64) -> Ray {
        // Construct a camera ray originating from the defocus disk and directed at randomly sampled
        // point around the pixel location i, j.

        let offset = Self::sample_square();
        let pixel_sample = self.pixel00_loc
                          + ((i + offset.x) * self.pixel_delta_u)
                          + ((j + offset.y) * self.pixel_delta_v);

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.camera_center
        } else {
            self.defocus_disc_sample()
        };
        let ray_direction = pixel_sample - ray_origin;
        let ray_time = rtweekend::random_double();

        return Ray::new2(ray_origin, ray_direction, ray_time);
    }

    fn sample_square() -> Vec3 {
        // Returns the vector to a random point in the [-.5,-.5]-[+.5,+.5] unit square.
        return Vec3::new(rtweekend::random_double() - 0.5, rtweekend::random_double() - 0.5, 0.0);
    }

    fn defocus_disc_sample(&self) -> Vec3 {
        // Returns a random point in the camera defocus disk.
        let p = random_in_unit_disc();
        return self.camera_center + (p.x * self.defocus_disc_u) + (p.y * self.defocus_disc_v);
    }

    fn ray_color(ray: &Ray, depth: u16, world: &Box<dyn Hittable>, mats: &Vec<Arc<dyn Material>>) -> Vec3 {
        // If we've exceeded the ray bounce limit, no more light is gathered.
        if depth <= 0 {
            return Vec3::new(0.0, 0.0, 0.0);
        }

        // NOTE: start interval at 0.0001 to avoid shadow acne from intersecting with same object
        let hit_record = world.hit(ray, &Interval::new(0.0001, std::f64::INFINITY));
        if let Some(hit) = hit_record {
            let mat = &mats[hit.material as usize];
            let color_from_emission = mat.emitted(hit.u, hit.v, hit.p);

            let scatter_result = mat.scatter(ray, &hit);
            if let Some(scattered) = scatter_result {
                let attenuation = scattered.attenuation;
                let color_from_scatter = attenuation * Camera::ray_color(&scattered.scattered, depth - 1, world, mats);
                return color_from_emission + color_from_scatter;
            }
            return color_from_emission;
        }

        // TODO: background for most scenes is = color(0.70, 0.80, 1.00);
        return Vec3::new(0.0, 0.0, 0.0); // TODO: background
    }
}

fn write_file(file_path: &str, contents: &str) -> Result<()> {
    let mut file = File::create(file_path)?;
    file.write_all(contents.as_bytes())
}

fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        return linear_component.sqrt();
    }

    return 0.0;
}

fn write_color(contents: &mut String, pixel_color: &Vec3) {
    let r = pixel_color.x;
    let g = pixel_color.y;
    let b = pixel_color.z;

    // Apply a linear to gamma transform for gamma 2
    let r_gamma= linear_to_gamma(r);
    let g_gamma= linear_to_gamma(g);
    let b_gamma= linear_to_gamma(b);

    // Translate the [0, 1] component value to byte range [0, 255]
    let intensity = Interval::new(0.0, 0.999);
    let r_byte = (256.0 * intensity.clamp(r_gamma)) as u8;
    let g_byte = (256.0 * intensity.clamp(g_gamma)) as u8;
    let b_byte = (256.0 * intensity.clamp(b_gamma)) as u8;

    contents.push_str(&format!("{} {} {}\n", r_byte, g_byte, b_byte));
}