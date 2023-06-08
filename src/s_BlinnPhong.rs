use std::collections::HashMap;

use rand::thread_rng;

use crate::Texture::Texture;
use crate::{Vec3, HStruct};
use crate::Shader::{Shading, Shader};
use crate::Light::{IsLight, Light};

#[allow(non_camel_case_types)]
#[derive(Clone,Copy)]
pub struct s_BlinnPhong
{   
    specular: Vec3,
    phongExp: f32
}

impl s_BlinnPhong
{
    pub fn new(specular: Vec3, phongExp: f32) -> s_BlinnPhong
    {
        s_BlinnPhong {specular: specular, phongExp: phongExp }
    }
}

impl Shading for s_BlinnPhong
{
    fn apply(&self,h_struct: &mut HStruct,color_to_shade: &Vec3,lights: &Vec<Light>,_shaders: &HashMap<String,Shader>,_textures: &HashMap<String,Texture>) -> Vec3 
    {
        let mut finalColor = Vec3::newEmpty();
        let mut rng = thread_rng();
        let intersect = h_struct.getIntersect();
        let normal = h_struct.getNormal();
        let ray = h_struct.getRay();
        let ambient = Vec3::new(0.1, 0.1, 0.1) * color_to_shade;
        for light in lights.iter()
        {
            let mut lcolor = Vec3::newEmpty();
            
            //CALCULATE SPECULAR COMPONENET
            let mut specular = Vec3::newEmpty();
            specular[0] = self.specular[0] * light.getIntensity()[0];
            specular[1] = self.specular[1] * light.getIntensity()[1];
            specular[2] = self.specular[2] * light.getIntensity()[2];
            specular = specular * light.getSpecularContribution(intersect, normal, self.phongExp, ray);

            //CALCULATE DIFFUSE COMPONENET
            lcolor[0] = light.getIntensity()[0] * color_to_shade[0];
            lcolor[1] = light.getIntensity()[1] * color_to_shade[1];
            lcolor[2] = light.getIntensity()[2] * color_to_shade[2];

            finalColor = finalColor + ((lcolor + specular) * light.getContribution(h_struct,intersect,normal,&mut rng));

        }
       finalColor + ambient
    }
}

pub type BlinnPhong = s_BlinnPhong;