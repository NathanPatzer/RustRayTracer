#![allow(non_snake_case)]
use indicatif::{ProgressBar, ProgressStyle};
use PespectiveCamera::PerspectiveCamera;
use rand::Rng;
use rand::thread_rng;
use std::sync::{Arc, Mutex};
pub const INFINITY: f32 = f32::INFINITY; // +Inff32

mod Vec3D;
mod Framebuffer;
mod CoordSys;
mod Ray;
mod PespectiveCamera;
mod Triangle;
mod SceneContainer;
mod Shape;
mod HitStruct;
mod Sphere;
mod JsonParser;
mod s_Lambertian;
mod Shader;
mod Light;
mod l_PointLight;
mod Camera;
mod s_BlinnPhong;
mod sBox;
mod ArgsChecker;
mod s_mirror;
mod AABoundingBox;
mod BVHNode;
mod s_Toon;
mod Texture;
mod objParser;
mod l_SpotLight;
mod l_area;
mod lookAtCam;
mod s_Glaze;

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
use crate::AABoundingBox::*;
use crate::s_Toon::*;
use crate::objParser::*;
use rayon::prelude::*;
use crate::s_Glaze::*;
use crate::l_area::*;

fn main() {
    let plainargs: Vec<String> = std::env::args().collect();
    let args: Args = Args::new(plainargs);
    let w = args.width;
    let h = args.height;
    let mut fb = Framebuffer::Framebuffer::new(args.width, args.height);
    
    let mut sc = SceneContainer::SceneContainer::new();

    //sc.background_color = Vec3::new(0.53,0.81,0.92);
    sc.background_color = Vec3::newEmpty();
    assert!(args.json.len() > 0, "NO JSON SUPPLIED");
    let parser = JsonParser::JsonParser::new(args.json, args.width as i32, args.height as i32,args.interpolation);
    parser.Parse(&mut sc);
    let cam = sc.getCamera();
    
    let rpp = args.rpp;
    
    let depth = args.depth;

    let shape_refs: &[Shape::Shape] = &sc.allShapes[..];

    let bvh = BVHNode::BVHNode::new(shape_refs.iter().map(|shape| Arc::new(shape.clone())).collect::<Vec<_>>().as_slice(), 0);
    sc.root = bvh;
    println!("CONSTRUCTED BVH");
    
    println!("RENDERING...");
    //LOADING BAR
    let progress_bar = Arc::new(ProgressBar::new((h) as u64));
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("[{bar:25}] {percent}%")
    );

    let hit_struct = &mut HStruct::new();
    
    hit_struct.setDepth(depth);
    hit_struct.setBackGroundColor(sc.background_color);
    hit_struct.setRoot(sc.root.clone());
    
    //Rayon
    let img: Arc<Mutex<Vec<Vec3>>> = Arc::new(Mutex::new(Vec::with_capacity((h*w) as usize)));
    img.lock().unwrap().resize_with((w * h) as usize, Vec3::newEmpty);
    let start = std::time::Instant::now();
    (0..h).into_par_iter().for_each(|j| {
        let mut t_h = hit_struct.clone();
        let mut rng = thread_rng();
        for i in 0..w {
            let mut pixel_color = Vec3::newEmpty();
            //ANTI ALIASING
            for p in 0..rpp
            {
                for q in 0..rpp
                {
                    let off_i: f32 =(p as f32 + rng.gen::<f32>()) / rpp as f32;
                    let off_j: f32 = (q as f32 + rng.gen::<f32>()) / rpp as f32;
                    let random_i = i as f32 + off_i;
                    let random_j = j as f32 + off_j;
                    let u: f32 = (random_i) / (w - 1) as f32;
                    let v: f32 = (random_j) / (h - 1) as f32;
                    let r = cam.genRay(u, v);
                    pixel_color = pixel_color + sc.rayColor(r, 1.0, INFINITY, &mut t_h,(u,v));
                    t_h.setDepth(depth);
                }
            }
            pixel_color = pixel_color / (rpp*rpp) as f32;
            let index: usize = ((j * w) + i) as usize;
            let mut img_lock = img.lock().expect("Failed to acquire lock on image");
            img_lock[index] = pixel_color;
            }
            progress_bar.inc(1);  
        });
    let end = std::time::Instant::now();
    let rendered = Arc::try_unwrap(img).unwrap().into_inner().unwrap();
    progress_bar.finish();
    fb.setAll(rendered);
    let filepath: String = "IMAGES/".to_owned() + &args.name + ".png";
    fb.exportAsPng(filepath);
    
    let mut elapsed_time = (end - start).as_secs_f32();
    if elapsed_time > 60.0
    {
        elapsed_time = elapsed_time / 60.0;
        println!("Time to render {:.2}m",elapsed_time);
    }
    else {
        println!("Time to render {:.2}s", elapsed_time);
    }
    
    
}