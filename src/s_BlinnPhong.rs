use crate::{Vec3, HStruct};
use crate::Shader::Shading;
use crate::Light::IsLight;

#[allow(non_camel_case_types)]
#[derive(Clone)]
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
    fn apply(&self,h_struct: &mut HStruct,color_to_shade: &Vec3) -> Vec3 
    {
        let mut finalColor = Vec3::newEmpty();
        let lights = h_struct.getLights();
        let intersect = h_struct.getIntersect();
        let normal = h_struct.getNormal();
        let ray = h_struct.getRay();
        for light in lights
        {
            let mut lcolor = Vec3::newEmpty();

            //CALCULATE SPECULAR COMPONENET
            let L = (light.getPos() - intersect).normalize();
            let V = (ray.dir * -1.0).normalize();
            let H = (L + V).normalize();
            let mut phongMax = 0.0_f32.max(normal.dot(&H));
            if phongMax == 0.0 {continue;}
            phongMax = phongMax.powf(self.phongExp);
            let mut specular = Vec3::newEmpty();
            specular[0] = self.specular[0] * light.getIntensity()[0];
            specular[1] = self.specular[1] * light.getIntensity()[1];
            specular[2] = self.specular[2] * light.getIntensity()[2];
            specular = specular * phongMax;

            //CALCULATE DIFFUSE COMPONENET
            let ndotl = normal.dot(&L);
            let max: f32 = 0.0_f32.max(ndotl);
            if max == 0.0 {continue;}
            lcolor[0] = light.getIntensity()[0] * color_to_shade[0];
            lcolor[1] = light.getIntensity()[1] * color_to_shade[1];
            lcolor[2] = light.getIntensity()[2] * color_to_shade[2];
            lcolor = lcolor * max;

            finalColor = finalColor + ((lcolor + specular) * light.getContribution(h_struct,intersect));

        }
       finalColor
    }
}

pub type BlinnPhong = s_BlinnPhong;