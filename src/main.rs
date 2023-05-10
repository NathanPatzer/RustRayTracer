use PespectiveCamera::PerspectiveCamera;



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

//INSTRUCTIONS
// ./exe [FILENAME]

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let red= Vec3D::Vec3D::new(1.0, 0.0, 0.0);
    let black= Vec3D::Vec3D::new(0.0, 0.0, 0.0);

    let coord = CoordSys::CoordSys::new(Vec3D::Vec3D::new(0.0,0.0,-1.0), Vec3D::Vec3D::new(0.0,1.0,0.0));
    let mut fb = Framebuffer::Framebuffer::new(1000, 1000);
    fb.setBackground(black);

    let tri = Triangle::Triangle::new(Vec3D::Vec3D::new(0.0, -1.0, -3.0),Vec3D::Vec3D::new(0.0,1.0,-5.0), Vec3D::Vec3D::new(-1.0,-1.0,-6.0));
    let cam = PerspectiveCamera::new(Vec3D::Vec3D::new(0.0, 0.0, 0.0), 0.5 as f32, 0.5 as f32, fb.width as i32, fb.height as i32, coord);
    let start = std::time::Instant::now();
    for j in 0..fb.height{
        for i in 0..fb.width
        {
            let r = cam.genRay(i as i32, j as i32, 0.0, 0.0);
            if tri.closestHit(r, 0.1, 1000000.0)
            {
                fb.setPixelColor(i, j, red.clone());
            }

        }
    }

    fb.exportAsPng(args[1].clone() + ".png");
    let end = std::time::Instant::now();
    let elapsed_time = end - start;
    println!("Time to render: {:?}", elapsed_time);
}

