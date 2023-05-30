use crate::{Vec3, HStruct};
use crate::Shader::Shading;
use crate::Light::IsLight;
use crate::Shader::Shader;
use crate::Texture::Texture;
#[allow(non_camel_case_types)]
#[derive(Clone)]
pub struct s_Lambertian
{
    diffuse: Option<Vec3>,
    texture: Option<String>
}

impl s_Lambertian
{
    pub fn new(diffuse: Option<Vec3>,texture: Option<String>) -> s_Lambertian
    {
        s_Lambertian{diffuse: diffuse, texture: texture}
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
        let (ru,rv) = self::s_Lambertian::rotateUV(u, v, 0.5);
        let x = (ru * (width - 1) as f32).floor() as usize;
        let y = (rv * (height - 1) as f32).floor() as usize;
        t.texture_array[x][y]
    }
}

impl Shading for s_Lambertian
{
    fn apply(&self,h_struct: &mut HStruct) -> Vec3 
    {
        let mut finalColor = Vec3::newEmpty();
        let lights = h_struct.getLights();
        let intersect = h_struct.getIntersect();
        let normal = h_struct.getNormal();
        let coords = h_struct.getCoords();
        for light in lights
        {
            let L = (light.getPos() - intersect).normalize();
            let ndotl = normal.dot(&L);
            let max: f32 = 0.0_f32.max(ndotl);
            let mut lcolor = Vec3::newEmpty();

            if self.diffuse.is_some()
            {
                lcolor[0] = light.getIntensity()[0] * self.diffuse.unwrap()[0];
                lcolor[1] = light.getIntensity()[1] * self.diffuse.unwrap()[1];
                lcolor[2] = light.getIntensity()[2] * self.diffuse.unwrap()[2];
            }
            else if self.texture.is_some()
            {  
                let t_color = self::Lambertian::get_texture_color(coords.0, coords.1, h_struct.getTexture(self.texture.clone().unwrap()));
                lcolor[0] = light.getIntensity()[0] * t_color[0];
                lcolor[1] = light.getIntensity()[1] * t_color[1];
                lcolor[2] = light.getIntensity()[2] * t_color[2];
            }
            else {
                panic!("No diffuse or texture provided!");
            }
            
            //SHADOWS
            let shadowRay = Shader::shadowRay(light, intersect);
            assert!(h_struct.scene.root.is_some(),"ROOT IS NONE");
            if Shader::anyHit(shadowRay, 0.0001, 1.0, h_struct) == false
            {
                finalColor = finalColor + (lcolor * max);
            }
        }
        finalColor
    }
}


pub type Lambertian = s_Lambertian;