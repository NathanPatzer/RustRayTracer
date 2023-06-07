use crate::{Vec3, HStruct};
use crate::Shader::Shading;
use crate::Light::IsLight;

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
        let ambient = Vec3::new(0.1, 0.1, 0.1) * color_to_shade;
        for light in lights
        {
            let mut lcolor = Vec3::newEmpty();
            lcolor[0] = light.getIntensity()[0] * color_to_shade[0];
            lcolor[1] = light.getIntensity()[1] * color_to_shade[1];
            lcolor[2] = light.getIntensity()[2] * color_to_shade[2];

            finalColor = finalColor + ((lcolor * light.getContribution(h_struct,intersect,normal)));

        }
        finalColor + ambient
    }
}


pub type Lambertian = s_Lambertian;