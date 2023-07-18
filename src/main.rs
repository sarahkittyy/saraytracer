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

fn create_scene(world: &mut World) {
    let ground: Diffuse = Color::new(0.8, 0.5, 0.9).into();

    world.insert(Sphere::new(Vec3::new(0., -1000., -1.), 1000., ground));

    let mut rng = thread_rng();
    for x in -8..8 {
        for z in -8..8 {
            let pos = Vec3 {
                x: (rng.gen::<f64>() * 0.9) + (x as f64),
                y: 0.2,
                z: (rng.gen::<f64>() * 0.9) + (z as f64),
            };

            if (pos - Vec3::new(4., 0.2, 0.)).length() > 0.9 {
                let choose_mat: f64 = rng.gen();

                if choose_mat < 0.8 {
                    // diffuse
                    let mat: Diffuse = Color::random().into();
                    world.insert(Sphere::new(pos, 0.2, mat));
                } else if choose_mat < 0.95 {
                    // metal
                    let mat = Metal {
                        color: Color::random() * 0.5 + Color::GRAY,
                        fuzz: rng.gen::<f64>() * 0.3,
                    };
                    world.insert(Sphere::new(pos, 0.2, mat));
                } else {
                    // dielectric
                    let mat = Dielectric {
                        refraction_index: 1.5,
                    };
                    world.insert(Sphere::new(pos, 0.2, mat));
                }
            }
        }
    }

    world.insert(Sphere::new(
        Vec3::new(0., 1., 0.),
        1.0,
        Dielectric {
            refraction_index: 1.5,
        },
    ));

    world.insert(Sphere::new(
        Vec3::new(4., 1., 0.),
        1.0,
        Metal {
            color: Color::new(0.8, 0.8, 0.8),
            fuzz: 0.0,
        },
    ));

    world.insert(Sphere::new(
        Vec3::new(-4., 1., 0.),
        1.0,
        Diffuse {
            color: Color::new(0.8, 0.5, 0.2),
        },
    ));
}

fn main() {
    const ASPECT_RATIO: f64 = 16.0 / 9.0; // width / height
    const IMAGE_WIDTH: u32 = 400;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
    const IMAGE_PIXELS: u32 = IMAGE_WIDTH * IMAGE_HEIGHT;
    const SAMPLES_PER_PIXEL: u32 = 50;

    // camera
    let eye = Vec3::new(13., 2., 3.);
    let camera = FixedCamera::new(
        eye,
        Vec3::ZERO,
        Vec3::Y,
        ASPECT_RATIO,
        20.,
        0.01,
        eye.length(),
    );

    // image storage
    let mut imgbuf = image::RgbImage::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    // world
    let world = Arc::new(RwLock::new(World::new()));
    create_scene(&mut world.write().unwrap());

    let now = Instant::now();
    let px: Vec<(u32, u32, Color)> = (0..IMAGE_PIXELS)
        .into_par_iter()
        .map_with(world, |world, i| {
            let mut rng = thread_rng();
            let x = i % IMAGE_WIDTH;
            let y = i / IMAGE_WIDTH;
            let mut pixel_color = Color::WHITE;
            for _ in 0..SAMPLES_PER_PIXEL {
                let (px, py) = (x as f64, y as f64);
                // 			vv random sampling
                let rx: f64 = rng.gen();
                let ry: f64 = rng.gen();
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

    for (x, y, color) in px.into_iter() {
        let px = &mut imgbuf.get_pixel_mut(x, IMAGE_HEIGHT - y - 1).0;
        *px = color.into_rgb8_array();
    }

    imgbuf.save("output.png").unwrap();
}
