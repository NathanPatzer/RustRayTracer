use crate::{Vec3, HStruct};
use crate::Shader::Shading;
use crate::Light::IsLight;
use crate::Shader::Shader;
#[allow(non_camel_case_types)]
#[derive(Clone)]
pub struct s_BlinnPhong
{   
    diffuse: Vec3,
    specular: Vec3,
    phongExp: f32
}

impl s_BlinnPhong
{
    pub fn new(diffuse: Vec3,specular: Vec3, phongExp: f32) -> s_BlinnPhong
    {
        s_BlinnPhong {diffuse: diffuse,  specular: specular, phongExp: phongExp }
    }
}

impl Shading for s_BlinnPhong
{
    fn apply(&self,h_struct: &mut HStruct) -> Vec3 
    {
        let mut finalColor = Vec3::newEmpty();
        let lights = h_struct.getLights();
        for light in lights
        {
            let mut lcolor = Vec3::newEmpty();

            //CALCULATE SPECULAR COMPONENET
            let L = (light.getPos() - h_struct.getIntersect()).normalize();
            let V = (h_struct.getRay().dir * -1.0).normalize();
            let H = (L + V).normalize();
            let mut phongMax = 0.0_f32.max(h_struct.getNormal().dot(&H));
            phongMax = phongMax.powf(self.phongExp);
            let mut specular = Vec3::newEmpty();
            specular[0] = self.specular[0] * light.getIntensity()[0];
            specular[1] = self.specular[1] * light.getIntensity()[1];
            specular[2] = self.specular[2] * light.getIntensity()[2];
            specular = specular * phongMax;

            //CALCULATE DIFFUSE COMPONENET
            let ndotl = h_struct.getNormal().dot(&L);
            let max: f32 = 0.0_f32.max(ndotl);
            lcolor[0] = light.getIntensity()[0] * self.diffuse[0];
            lcolor[1] = light.getIntensity()[1] * self.diffuse[1];
            lcolor[2] = light.getIntensity()[2] * self.diffuse[2];
            lcolor = lcolor * max;

            //SHADOWS
            let shadowRay = Shader::shadowRay(light, h_struct.getIntersect());
            if Shader::anyHit(shadowRay, 0.0001, 1.0, h_struct) == false
            {
                finalColor = finalColor + (lcolor + specular);
            }

        }

       finalColor
    }
}

pub type BlinnPhong = s_BlinnPhong;