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

    pub fn exportAsPng(&mut self,filename: String)
    {
        let mut img = RgbImage::new(self.width, self.height);
        for i in 0..self.pixelArray.len()
        {
            let x:u32 = i as u32 % self.width;
            let y:u32 = (i as f32 / self.width as f32).floor() as u32;
            if self.pixelArray[i][0] > 1.0
            {
                self.pixelArray[i][0] = 1.0;
            }
            if self.pixelArray[i][1] > 1.0
            {
                self.pixelArray[i][1] = 1.0;
            }
            if self.pixelArray[i][2] > 1.0
            {
                self.pixelArray[i][2] = 1.0;
            }
            let pixel = Rgb([(255 as f32 * self.pixelArray[i][0]) as u8,(255 as f32 * self.pixelArray[i][1]) as u8,(255 as f32 * self.pixelArray[i][2]) as u8]);
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
        for i in 0..self.pixelArray.len()
        {
            self.pixelArray[i] = color;
        }
    }

    pub fn setAll(&mut self,img: Vec<Vec3D>)
    {
        assert!(img.len() == self.pixelArray.len(), "Invalid Array");
        for i in 0..img.len()
        {
            self.pixelArray[i] = img[i];
        }
    }
}
