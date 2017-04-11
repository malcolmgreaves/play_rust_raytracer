#[macro_use]
extern crate serde_derive;
extern crate image;
extern crate serde;

use image::DynamicImage;


#[derive(Copy, Clone, Debug, Deserialize)]
#[repr(C)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

pub struct Sphere {
    pub center: Point,
    pub radius: f64,
    pub color: Color,
}

pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub fov: f64,
    pub sphere: Sphere,
}


pub fn render(scene: &Scene) -> DynamicImage {
    DynamicImage::new_rgb8(scene.width, scene.height)
}


// pub struct Ray {
//     pub origin: Point,
//     pub direction: Vector3,
// }

// impl Ray {
//     pub fn create_prime(x: u32, y: u32, scene: &Scene) -> Ray {
//         let sensor_x = ((x as f64 + 0.5) / scene.width as f64) * 2.0 - 1.0;
//         let sensor_y = 1.0 - ((y as f64 + 0.5) / scene.height as f64) * 2.0;

//         Ray {
//             origin: Point::zero(),
//             direction: Vector3 {
//                     x: sensor_x,
//                     y: sensor_y,
//                     z: -1.0,
//                 }
//                 .normalize(),
//         }
//     }
// }

#[test]
fn test_can_render_scene() {

    use image::GenericImage;

    let scene = Scene {
        width: 800,
        height: 600,
        fov: 90.0,
        sphere: Sphere {
            center: Point {
                x: 0.0,
                y: 0.0,
                z: -5.0,
            },
            radius: 1.0,
            color: Color {
                red: 0.4,
                green: 1.0,
                blue: 0.4,
            },
        },
    };

    let img: DynamicImage = render(&scene);
    assert_eq!(scene.width, img.width());
    assert_eq!(scene.height, img.height());
}