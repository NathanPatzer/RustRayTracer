use image::GenericImageView;
use crate::Vec3;
#[allow(non_camel_case_types)]
#[derive(Clone)]
pub struct Texture
{
    pub texture_array: Vec<Vec<Vec3>>,
    pub nx: u32,
    pub ny: u32
}

impl Texture
{
    pub fn create_empty() -> Texture
    {
        let e: Vec<Vec<Vec3>> = Vec::new();
        Texture { texture_array: e, nx: 0, ny: 0 }
    }

    pub fn new(image_path: String) -> Texture
    {
        let img = image::open(image_path).expect("Failed to open texture image");
        let (width,height) = img.dimensions();
        let mut texture_grid: Vec<Vec<Vec3>> = vec![vec![Vec3::newEmpty(); height as usize]; width as usize];
        for (x,y,pixel) in img.pixels()
        {
            let mut p: Vec3 = Vec3::newEmpty();
            p[0] = pixel[0] as f32 / 255.0; 
            p[1] = pixel[1] as f32 / 255.0;
            p[2] = pixel[2] as f32 / 255.0;
            texture_grid[x as usize][y as usize] = p;
        }
        Texture { texture_array: texture_grid, nx: width, ny: height }
    }
}

