use rand::Rng;
use PespectiveCamera::PerspectiveCamera;
pub const INFINITY: f32 = f32::INFINITY; // +Inff32

//use image::{Rgb, RgbImage};
#[allow(non_snake_case)]
mod Vec3D;
#[allow(non_snake_case)]
mod Framebuffer;
#[allow(non_snake_case)]
mod CoordSys;
#[allow(non_snake_case)]
mod Ray;
#[allow(non_snake_case)]
mod PespectiveCamera;
#[allow(non_snake_case)]
mod Triangle;
#[allow(non_snake_case)]
mod SceneContainer;
#[allow(non_snake_case)]
mod Shape;
#[allow(non_snake_case)]
mod HitStruct;
#[allow(non_snake_case)]
mod Sphere;
#[allow(non_snake_case)]
mod JsonParser;
#[allow(non_snake_case)]
mod s_Lambertian;
#[allow(non_snake_case)]
mod Shader;
#[allow(non_snake_case)]
mod Light;
#[allow(non_snake_case)]
mod l_PointLight;
#[allow(non_snake_case)]
mod Camera;
#[allow(non_snake_case)]
mod s_BlinnPhong;
#[allow(non_snake_case)]
mod sBox;
#[allow(non_snake_case)]
mod ArgsChecker;
mod s_mirror;

pub use Vec3D::*;
pub use CoordSys::*;
pub use Triangle::*;
pub use HitStruct::*;
pub use Sphere::*;
pub use Shader::*;
pub use s_Lambertian::*;
pub use l_PointLight::*;
pub use s_BlinnPhong::*;
pub use sBox::*;
use crate::Camera::CanGenRay;
use crate::ArgsChecker::*;
use crate::s_mirror::*;

fn main() {
    let plainargs: Vec<String> = std::env::args().collect();
    let args: Args = Args::new(plainargs);

    let mut fb = Framebuffer::Framebuffer::new(args.width, args.height);

    let mut sc = SceneContainer::SceneContainer::new();
    sc.background_color = Vec3::new(0.2, 0.2, 0.2);
    assert!(args.json.len() > 0, "NO JSON SUPPLIED");
    let parser = JsonParser::JsonParser::new(args.json, args.width as i32, args.height as i32);

    parser.Parse(&mut sc);
    let cam = sc.getCamera();
    let min_t = 1.0;
    let rpp = args.rpp;
    let mut rng: rand::rngs::ThreadRng = rand::thread_rng();
    let depth = args.depth;
    let hit_struct = &mut HStruct::new();
    let shape_refs: &[Shape::Shape] = &sc.allShapes[..];
    let light_refs: &[Light::Light] = &sc.allLights[..];
    hit_struct.setShapes(shape_refs.to_vec());
    hit_struct.setLights(light_refs.to_vec());
    hit_struct.setDepth(depth);
    hit_struct.setShaders(sc.allShaders.clone());
    hit_struct.setBackGroundColor(sc.background_color);
    let max_t = INFINITY;

    let start = std::time::Instant::now();
    for j in 0..fb.height{
        for i in 0..fb.width
        {
            let mut pixel_color = Vec3::newEmpty();
            //ANTI ALIASING
            for p in 0..rpp
            {
                for q in 0..rpp
                {
                    let off_i: f32 = (p as f32 + rng.gen::<f32>()) / rpp as f32;
                    let off_j: f32 = (q as f32 + rng.gen::<f32>()) / rpp as f32;
                    let r = cam.genRay(i as i32, j as i32, off_i, off_j);
                    pixel_color = pixel_color + sc.rayColor(r, min_t, max_t, depth, hit_struct)
                }
            }
            pixel_color = pixel_color / (rpp*rpp) as f32;
            fb.setPixelColor(i, j, &pixel_color)
        }
    }
    let end = std::time::Instant::now();
    
    let filepath: String = "IMAGES/".to_owned() + &args.name + ".png";
    fb.exportAsPng(filepath);
    
    let elapsed_time = end - start;
    println!("Time to render: {:?}", elapsed_time);
}