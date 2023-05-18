use crate::Vec3D::Vec3D;
use image::{Rgb, RgbImage};

pub struct Framebuffer
{
    pub width: u32,
    pub height: u32,
    pixelArray: Vec<Vec3D>    
}

impl Framebuffer
{
    pub fn new(w: u32, h: u32) -> Self
    {
        let arraySize = (w * h) as usize;
        Self 
        {
            width: w,
            height: h,
            pixelArray: vec![Vec3D::newEmpty(); arraySize]
        }
    }

    pub fn setPixelColor(&mut self ,locX: u32 ,locY: u32 , rgb: &Vec3D)
    {
        let index: usize = ((locY * self.width) + locX) as usize;
        assert!(index < self.pixelArray.len(), "Index out of range");
        self.pixelArray[index] = *rgb;
        
    }

    pub fn exportAsPng(&mut self,filename: String)
    {
        let mut img = RgbImage::new(self.width, self.height);
        for i in 0u32..self.pixelArray.len() as u32
        {
            let x:u32 = i % self.width;
            let y:u32 = (i as f32 / self.width as f32).floor() as u32;
            if self.pixelArray[i as usize][0] > 1.0
            {
                self.pixelArray[i as usize][0] = 1.0;
            }
            if self.pixelArray[i as usize][1] > 1.0
            {
                self.pixelArray[i as usize][1] = 1.0;
            }
            if self.pixelArray[i as usize][2] > 1.0
            {
                self.pixelArray[i as usize][2] = 1.0;
            }
            let pixel = Rgb([(255 as f32 * self.pixelArray[i as usize][0]) as u8,(255 as f32 * self.pixelArray[i as usize][1]) as u8,(255 as f32 * self.pixelArray[i as usize][2]) as u8]);
            img.put_pixel(x, self.height - 1 - y, pixel);
        }

        if let Err(e) = img.save(filename)
        {
            eprintln!("Error saving image: {}", e);
        }
    }

    #[allow(dead_code)]
    pub fn setBackground(&mut self,color: Vec3D)
    {
        for i in 0u32..self.pixelArray.len() as u32
        {
            self.pixelArray[i as usize] = color;
        }
    }
}
