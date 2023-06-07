#![allow(non_snake_case)]
use indicatif::{ProgressBar, ProgressStyle};
use rand::Rng;
use PespectiveCamera::PerspectiveCamera;

use std::sync::{Arc, Mutex};
use std::thread::{self};

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
//use crate::l_SpotLight::*;
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
    sc.root = Some(bvh);
    println!("CONSTRUCTED BVH");
    assert!(sc.root.is_some());
    let slice_width: u32 = w / args.num_threads;

    //THREADS
    let mut threads = vec![];
    let slices = Arc::new(Mutex::new(vec![]));
    
    println!("RENDERING WITH {} THREADS...",args.num_threads);
    //LOADING BAR
    let progress_bar = Arc::new(ProgressBar::new((args.height * args.num_threads) as u64));
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("[{bar:25}] {percent}%")
    );

    let start = std::time::Instant::now();
    for i in 0..args.num_threads 
    {
        let thread_sc = sc.clone();
        let slice_start = slice_width * i;
        let slices_clone = Arc::clone(&slices);
        let progress_bar_clone = progress_bar.clone();
        // Spawn a new thread for each iteration
        let thread = thread::spawn({
            move|| {
            let slice_to_push = render_slice(w,h, rpp, cam, &thread_sc, depth, slice_width*(i+1), slice_start, i,args.num_threads,&progress_bar_clone);
            let mut v = slices_clone.lock().unwrap();
            v.push(slice_to_push);
            }
        });
        // Store the thread handle in the vector
        threads.push(thread);
    }
    
    for thread in threads
    {
        thread.join().unwrap();
    }
    let end = std::time::Instant::now();
    progress_bar.finish();
    let slices_guard = slices.lock().unwrap();
    let slices_vec = &*slices_guard;
    for slice in slices_vec.iter()
    {
        fb.setFromSlice(slice.to_vec());
    }

    let filepath: String = "IMAGES/".to_owned() + &args.name + ".png";
    fb.exportAsPng(filepath);
    
    let elapsed_time = end - start;
    let rounded = (elapsed_time.as_secs_f64() * 100.0).round() / 100.0;
    println!("Time to render: {:.2}s", rounded);
}

fn render_slice(img_w: u32,img_h: u32, rpp: u32, cam: Camera::Camera, sc: &SceneContainer::SceneContainer, depth: i32, slice_width: u32, slice_start: u32, thread: u32,num_threads: u32,progress_bar: &ProgressBar) -> Vec<(u32,u32,Vec3)>
{
    //initialize hitstruct
    let hit_struct = &mut HStruct::new();
    let shape_refs: &[Shape::Shape] = &sc.allShapes[..];
    let light_refs: &[Light::Light] = &sc.allLights[..];
    hit_struct.setShapes(shape_refs.to_vec());
    hit_struct.setLights(light_refs.to_vec());
    hit_struct.setDepth(depth);
    hit_struct.setShaders(sc.allShaders.clone());
    hit_struct.setBackGroundColor(sc.background_color);
    hit_struct.setTextures(sc.allTextures.clone());
    let mut rng: rand::rngs::ThreadRng = rand::thread_rng();
    let mut slice: Vec<(u32,u32,Vec3)> = Vec::new();
    let min_t = 1.0;
    let max_t = INFINITY;
    let mut sw = slice_width;
    
    //renders rest of image on lst thread
    if thread == num_threads - 1
    {
        sw = img_w;
    }

    for j in 0..img_h{
        for i in slice_start..sw {
            let mut pixel_color = Vec3::newEmpty();
            //ANTI ALIASING
            for p in 0..rpp
            {
                for q in 0..rpp
                {
                    let off_i: f32 = (p as f32 + rng.gen::<f32>()) / rpp as f32;
                    let off_j: f32 = (q as f32 + rng.gen::<f32>()) / rpp as f32;
                    let r = cam.genRay(i as i32, j as i32, off_i, off_j);
                    pixel_color = pixel_color + sc.rayColor(r, min_t, max_t, hit_struct)
                }
            }
            pixel_color = pixel_color / (rpp*rpp) as f32;
            slice.push((i,j,pixel_color));
            }
            progress_bar.inc(1);  
        }
        
        slice
}