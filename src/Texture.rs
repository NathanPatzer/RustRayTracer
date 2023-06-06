use image::GenericImageView;
use crate::Vec3;
#[allow(non_camel_case_types)]
#[derive(Clone)]
pub struct Texture
{
    texture_array: Option<Vec<VecU8>>,
    pub nx: Option<u32>,
    pub ny: Option<u32>,
    pub color: Option<Vec3>,
    pub isTexture: bool
}

#[derive(Clone,Copy)]
struct VecU8 {
    r: u8,
    g: u8,
    b: u8,
}

impl Texture
{
    pub fn create_empty() -> Texture
    {
        let e: Vec<VecU8> = Vec::new();
        Texture { texture_array: Some(e), nx: Some(0), ny: Some(0),color: None,isTexture: false }
    }

    fn rotateUV(u: f32, v: f32, _angle: f32) -> (f32, f32)
    {
        let rotated_u = u;
        let rotated_v = 1.0 - v;
        (rotated_u, rotated_v)
    }

    pub fn get_texture_color(&self,u: f32, v: f32, t: &Texture) -> Vec3 
    {
        let width = t.nx.unwrap();
        let height = t.ny.unwrap();
        let (ru,rv) = self::Texture::rotateUV(u, v, 0.5);
        let x = (ru * (width - 1) as f32).floor() as usize;
        let y = (rv * (height - 1) as f32).floor() as usize;
        let index: usize = ((y * self.nx.unwrap() as usize ) + x) as usize;
        let color = self.texture_array.as_ref().unwrap()[index];
        Vec3::new(color.r as f32 / 255.0,color.g as f32 / 255.0,color.b as f32 / 255.0)
    }

    pub fn create_diffuse_texture(c: Vec3)->Texture
    {
        Texture { texture_array: None, nx: None, ny: None, color: Some(c), isTexture: false }
    }

    pub fn get_diffuse_color(&self) -> Vec3
    {
        self.color.unwrap()
    }

    pub fn new(image_path: String) -> Texture
    {
        let img = image::open(image_path).expect("Failed to open texture image");
        let (width,height) = img.dimensions();
        let mut texture_grid: Vec<VecU8> = vec![VecU8 { r: 0, g: 0, b: 0 }; (width * height) as usize];
        for (x,y,pixel) in img.pixels()
        {
           let color: VecU8 = VecU8 { r: pixel[0], g: pixel[1], b: pixel[2] };
           let index: usize = ((y as usize * width as usize ) + x as usize) as usize;
           texture_grid[index] = color;
        }
        
        Texture { texture_array: Some(texture_grid), nx: Some(width), ny: Some(height), color: None,isTexture: true }
    }
}