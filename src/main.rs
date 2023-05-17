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
use crate::{Shape::Hittable, Camera::CanGenRay};
//INSTRUCTIONS
// ./exe [FILENAME]

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut fb = Framebuffer::Framebuffer::new(1000, 1000);
    fb.setBackground(Vec3::new(0.2, 0.2, 0.2));

    let mut sc = SceneContainer::SceneContainer::new();
    
    let parser = JsonParser::JsonParser::new("SceneData/phongExp.json".to_string(), fb.width as i32, fb.height as i32);

    parser.Parse(&mut sc);
    let cam = sc.getCamera();
    let shape_refs: &[Shape::Shape] = &sc.allShapes[..];
    let light_refs: &[Light::Light] = &sc.allLights[..];
    let start = std::time::Instant::now();
    let min_t = 1.0;
    for j in 0..fb.height{
        for i in 0..fb.width
        {
            let h: &mut HStruct = &mut HStruct::new();   
            h.setShapes(shape_refs.to_vec());
            h.setLights(light_refs.to_vec());
            let mut max_t = INFINITY; 
            
            let r = cam.genRay(i as i32, j as i32, 0.0, 0.0);
            for s in shape_refs
            {   
                if s.closestHit(&r, min_t, max_t, h)
                {   
                    let shader_name = s.getShaderName();
                    let shader = sc.allShaders.get(&shader_name).expect("INVALID SHADER");
                    max_t = h.getT();
                    fb.setPixelColor(i, j, &shader.apply(h));
                }
            }
        }
    }

    let end = std::time::Instant::now();
    let filepath: String = "IMAGES/".to_owned() + &args[1] + ".png";
    fb.exportAsPng(filepath);
    
    let elapsed_time = end - start;
    println!("Time to render: {:?}", elapsed_time);
}

