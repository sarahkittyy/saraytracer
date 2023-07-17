use std::sync::{Arc, RwLock};
use std::time::Instant;

use image;
use rand::prelude::*;
use rayon::prelude::*;

mod math;
mod rt;
use math::*;
use rt::*;

fn ray_color(ray: Ray, world: &World, max_depth: u32) -> Color {
    if max_depth <= 0 {
        return Color::BLACK;
    }

    if let Some(contact) = world.hit(ray, 0.001..f64::INFINITY) {
        return match contact.material.scatter(ray, &contact) {
            Some(RayScatter { ray, attenuation }) => {
                attenuation * ray_color(ray, world, max_depth - 1)
            }
            None => Color::BLACK,
        };
    }
    // background
    let dir = ray.direction.normalize();
    let t = (dir.y + 1.) / 2.;
    (1. - t) * Color::WHITE + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    const ASPECT_RATIO: f64 = 16.0 / 9.0; // width / height
    const IMAGE_WIDTH: u32 = 400;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
    const SAMPLES_PER_PIXEL: u32 = 100;

    // camera
    let camera = FixedCamera::new(Vec3::new(0., 0., 0.), -Vec3::Z, Vec3::Y, ASPECT_RATIO, 90.);

    // image storage
    let mut imgbuf = image::RgbImage::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    let ground: Diffuse = Color::new(0.8, 0.8, 0.0).into();
    let pink: Diffuse = Color::new(0.7, 0.3, 0.3).into();
    let silver = Metal {
        color: Color::new(0.8, 0.8, 0.8),
        fuzz: 0.0,
    };
    let gold = Metal {
        color: Color::new(0.8, 0.6, 0.2),
        fuzz: 1.0,
    };

    // world
    let world = Arc::new(RwLock::new(World::new()));
    {
        let mut world = world.write().unwrap();
        world.insert(Sphere::new(Vec3::new(0., -25.5, -1.), 25., ground));
        world.insert(Sphere::new(-Vec3::Z, 0.5, pink));
        world.insert(Sphere::new(Vec3::X - Vec3::Z, 0.5, gold));
        world.insert(Sphere::new(-Vec3::X - Vec3::Z, 0.5, silver));
    }

    let now = Instant::now();
    let colors: Vec<(u32, u32, Vec3)> = (0..(IMAGE_HEIGHT * IMAGE_WIDTH))
        .into_par_iter()
        .map_with(world, |world, i| {
            let x = i % IMAGE_WIDTH;
            let y = i / IMAGE_WIDTH;
            let mut pixel_color = Color::WHITE;
            for _ in 0..SAMPLES_PER_PIXEL {
                let (px, py) = (x as f64, y as f64);
                // 			vv random sampling
                let rx: f64 = random();
                let ry: f64 = random();
                let dx = (px + rx) / ((IMAGE_WIDTH - 1) as f64);
                let dy = (py + ry) / ((IMAGE_HEIGHT - 1) as f64);
                let r = camera.get_screen_ray(dx, dy);
                pixel_color += ray_color(r, &world.read().unwrap(), 50);
            }
            let color = pixel_color / SAMPLES_PER_PIXEL as f64;
            (x, y, color)
        })
        .collect();
    let elapsed = now.elapsed();
    println!("Raytracer computed in {:.2}s", elapsed.as_secs_f64());

    colors.iter().for_each(|&(x, y, color)| {
        let rgb = &mut imgbuf.get_pixel_mut(x, IMAGE_HEIGHT - y - 1).0;
        *rgb = color.into_rgb8_array();
    });
    imgbuf.save("output.png").unwrap();
}
