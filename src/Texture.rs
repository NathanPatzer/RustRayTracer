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

    fn rotateUV(u: f32, v: f32, _angle: f32) -> (f32, f32)
    {
        let rotated_u = u;  // Apply rotation to U coordinate
        let rotated_v = 1.0 - v;  // Keep V coordinate unchanged
        (rotated_u, rotated_v)
    }

    pub fn get_texture_color(u: f32, v: f32, t: &Texture) -> Vec3 
    {
        let width = t.nx;
        let height = t.ny;
        let (ru,rv) = self::Texture::rotateUV(u, v, 0.5);
        let x = (ru * (width - 1) as f32).floor() as usize;
        let y = (rv * (height - 1) as f32).floor() as usize;
        t.texture_array[x][y]
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

