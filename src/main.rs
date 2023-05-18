use rand::Rng;
use PespectiveCamera::PerspectiveCamera;
use std::sync::{Arc, Mutex};
use std::thread::{self};

pub const INFINITY: f32 = f32::INFINITY; // +Inff32
pub const NUM_THREADS: u32 = 16;

//use indicatif::ProgressBar;
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
    let w = args.width;
    let h = args.height;
    let mut fb = Framebuffer::Framebuffer::new(args.width, args.height);

    let mut sc = SceneContainer::SceneContainer::new();
    sc.background_color = Vec3::new(0.53,0.81,0.92);
    assert!(args.json.len() > 0, "NO JSON SUPPLIED");
    let parser = JsonParser::JsonParser::new(args.json, args.width as i32, args.height as i32);

    parser.Parse(&mut sc);
    let cam = sc.getCamera();
    
    let rpp = args.rpp;
    
    let depth = args.depth;
    let hit_struct = &mut HStruct::new();
    let shape_refs: &[Shape::Shape] = &sc.allShapes[..];
    let light_refs: &[Light::Light] = &sc.allLights[..];
    hit_struct.setShapes(shape_refs.to_vec());
    hit_struct.setLights(light_refs.to_vec());
    hit_struct.setDepth(depth);
    hit_struct.setShaders(sc.allShaders.clone());
    hit_struct.setBackGroundColor(sc.background_color);
    
    let slice_width: u32 = w / NUM_THREADS;
    
    //THREADS
    let mut threads = vec![];
    let slices = Arc::new(Mutex::new(vec![]));

    let start = std::time::Instant::now();
    for i in 0..NUM_THREADS 
    {
        let thread_sc = sc.clone();
        let slice_start = slice_width * i;

        // Spawn a new thread for each iteration
        let thread = thread::spawn({
            let slices_clone = Arc::clone(&slices);
            move|| {
            let slice_to_push = render_slice(w,h, rpp, cam, &thread_sc, depth, slice_width*(i+1), slice_start, i);
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
    let slices_guard = slices.lock().unwrap();
    let slices_vec = &*slices_guard;
    for slice in slices_vec.iter()
    {
        fb.setFromSlice(slice.to_vec());
    }

    let filepath: String = "IMAGES/".to_owned() + &args.name + ".png";
    fb.exportAsPng(filepath);
    
    let elapsed_time = end - start;
    println!("Time to render: {:?}", elapsed_time);
}

fn render_slice(img_w: u32,img_h: u32, rpp: u32, cam: Camera::Camera, sc: &SceneContainer::SceneContainer, depth: i32, slice_width: u32, slice_start: u32, thread: u32) -> Vec<(u32,u32,Vec3)>
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

    let mut rng: rand::rngs::ThreadRng = rand::thread_rng();
    let mut slice: Vec<(u32,u32,Vec3)> = Vec::new();
    let min_t = 1.0;
    let max_t = INFINITY;
    let mut sw = slice_width;
    
    //renders rest of image on last thread
    if thread == NUM_THREADS - 1
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
                    pixel_color = pixel_color + sc.rayColor(r, min_t, max_t, depth, hit_struct)
                }
            }
            pixel_color = pixel_color / (rpp*rpp) as f32;
            slice.push((i,j,pixel_color));
            }  
        }
        slice
}