use crate::{Vec3, HStruct};
use crate::Shader::Shading;
use crate::Light::IsLight;
use crate::Shader::Shader;

#[allow(non_camel_case_types)]
#[derive(Clone)]
pub struct s_Lambertian
{

}

impl s_Lambertian
{
    pub fn new() -> s_Lambertian
    {
        s_Lambertian{}
    }
}

impl Shading for s_Lambertian
{
    fn apply(&self,h_struct: &mut HStruct, color_to_shade: &Vec3) -> Vec3 
    {
        let mut finalColor = Vec3::newEmpty();
        let lights = h_struct.getLights();
        let intersect = h_struct.getIntersect();
        let normal = h_struct.getNormal();
        for light in lights
        {
            let L = (light.getPos() - intersect).normalize();
            let ndotl = normal.dot(&L);
            let max: f32 = 0.0_f32.max(ndotl);
            let mut lcolor = Vec3::newEmpty();

            lcolor[0] = light.getIntensity()[0] * color_to_shade[0];
            lcolor[1] = light.getIntensity()[1] * color_to_shade[1];
            lcolor[2] = light.getIntensity()[2] * color_to_shade[2];

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