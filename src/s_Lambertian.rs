use crate::{Vec3, HStruct};
use crate::Shader::Shading;
use crate::Light::IsLight;
use crate::Shader::Shader;
#[allow(non_camel_case_types)]
#[derive(Clone)]
pub struct s_Lambertian
{
    diffuse: Vec3
}

impl s_Lambertian
{
    pub fn new(diffuse: Vec3) -> s_Lambertian
    {
        s_Lambertian{diffuse: diffuse}
    }
}

impl Shading for s_Lambertian
{
    fn apply(&self,h_struct: &mut HStruct) -> Vec3 
    {
        let mut finalColor = Vec3::newEmpty();
        let lights = h_struct.getLights();
        for light in lights
        {
            let L = (light.getPos() - h_struct.getIntersect()).normalize();
            
            let ndotl = h_struct.getNormal().dot(&L);
            let max: f32 = 0.0_f32.max(ndotl);
            let mut lcolor = Vec3::newEmpty();
            lcolor[0] = light.getIntensity()[0] * self.diffuse[0];
            lcolor[1] = light.getIntensity()[1] * self.diffuse[1];
            lcolor[2] = light.getIntensity()[2] * self.diffuse[2];
            
            //SHADOWS
            let shadowRay = Shader::shadowRay(light, h_struct.getIntersect());
            assert!(h_struct.scene.root.is_some(),"ROOT IS NONE");
            if Shader::anyHit(shadowRay, 0.000001, 1.0, h_struct) == false
            {
                finalColor = finalColor + (lcolor * max);
            }
        }
        finalColor
    }
}


pub type Lambertian = s_Lambertian;