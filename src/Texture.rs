use image::GenericImageView;
use crate::Vec3;

#[allow(non_camel_case_types)]
#[derive(Clone)]
pub struct Texture
{
    pub texture_array: Option<Vec<Vec<Vec3>>>,
    pub nx: Option<u32>,
    pub ny: Option<u32>,
    pub color: Option<Vec3>,
    pub isTexture: bool
}

impl Texture
{
    pub fn create_empty() -> Texture
    {
        let e: Vec<Vec<Vec3>> = Vec::new();
        Texture { texture_array: Some(e), nx: Some(0), ny: Some(0),color: None,isTexture: false }
    }

    fn rotateUV(u: f32, v: f32, _angle: f32) -> (f32, f32)
    {
        let rotated_u = u;
        let rotated_v = 1.0 - v;
        (rotated_u, rotated_v)
    }

    pub fn get_texture_color(u: f32, v: f32, t: &Texture) -> Vec3 
    {
        let width = t.nx.unwrap();
        let height = t.ny.unwrap();
        let (ru,rv) = self::Texture::rotateUV(u, v, 0.5);
        let x = (ru * (width - 1) as f32).floor() as usize;
        let y = (rv * (height - 1) as f32).floor() as usize;
        t.texture_array.as_ref().unwrap()[x][y]
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
        let mut texture_grid: Vec<Vec<Vec3>> = vec![vec![Vec3::newEmpty(); height as usize]; width as usize];
        for (x,y,pixel) in img.pixels()
        {
            
            let mut p: Vec3 = Vec3::newEmpty();
            p[0] = pixel[0] as f32 / 255.0; 
            p[1] = pixel[1] as f32 / 255.0;
            p[2] = pixel[2] as f32 / 255.0;
            texture_grid[x as usize][y as usize] = p;
            
        }
        Texture { texture_array: Some(texture_grid), nx: Some(width), ny: Some(height), color: None,isTexture: true }
    }
}