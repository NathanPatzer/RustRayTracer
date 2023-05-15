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


pub use Vec3D::*;
pub use CoordSys::*;
pub use Triangle::*;
pub use HitStruct::*;

use crate::Shape::Hittable;
//INSTRUCTIONS
// ./exe [FILENAME]

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    let red= Vec3::new(1.0, 0.0, 0.0);
    let black= Vec3::new(0.0, 0.0, 0.0);
    let blue = Vec3::new(0.0,0.0,1.0);
    let coord = Coord::new(Vec3::new(0.0,0.0,-1.0), Vec3::new(0.0,1.0,0.0));
    let mut fb = Framebuffer::Framebuffer::new(1000, 1000);
    fb.setBackground(black);

    let tri1 = Tri::new(Vec3::new(0.4, -1.0, -2.5),Vec3::new(0.0,1.0,-5.0), Vec3::new(-1.0,-1.0,-6.0));
    let tri2 = Tri::new(Vec3::new(0.7, -1.0, -2.8),Vec3::new(0.0,1.0,-4.0), Vec3::new(-1.0,-1.0,-6.0));
    let mut sc = SceneContainer::SceneContainer::new();
    sc.addTriangle(tri1);
    sc.addTriangle(tri2);

    let cam = PerspectiveCamera::new(Vec3::new(0.0, 0.0, 0.0), 0.5 as f32, 0.5 as f32, fb.width as i32, fb.height as i32, coord);
    let start = std::time::Instant::now();
    let shape_refs = &sc.shapes[..];
    let h = &mut HStruct::new();


    //TEST
    let colvec: Vec<&Vec3> = vec![&red,&blue];
    let mut shapenum: i32 = 0;
    for j in 0..fb.height{
        for i in 0..fb.width
        {   
            let max_t = INFINITY; 
            let mut min_t = 1.0;
            let r = cam.genRay(i as i32, j as i32, 0.0, 0.0);
            for s in shape_refs
            {   
                if s.closestHit(&r, min_t, max_t, h)
                {
                    min_t = h.getT();
                    fb.setPixelColor(i, j, colvec[shapenum as usize]);
                }
                shapenum = shapenum + 1;
            }
            shapenum = 0;
            h.setT(1.0);
        }
    }

    fb.exportAsPng(args[1].clone() + ".png");
    let end = std::time::Instant::now();
    let elapsed_time = end - start;
    println!("Time to render: {:?}", elapsed_time);
}

