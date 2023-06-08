use crate::{Vec3, HStruct};
use crate::Shader::Shading;
use crate::Light::{IsLight, Light};


#[allow(non_camel_case_types)]
#[derive(Clone,Copy)]
pub struct s_Toon
{
    thresh: i32
}

impl s_Toon
{
    pub fn new(t: i32) ->s_Toon
    {
        s_Toon {thresh: t }
    }
}

impl Shading for s_Toon
{
    fn apply(&self,h_struct: &mut HStruct,color_to_shade: &Vec3, lights: &Vec<Light>) -> Vec3 
    {
        let mut final_color = Vec3::newEmpty();

        for light in lights.iter() {
            let l = (light.getPos() - h_struct.getIntersect()).normalize();
            let ndotl = h_struct.getNormal().dot(&l);
            let max = 0.0_f32.max(ndotl);
            let mut lcolor = Vec3::newEmpty();
            lcolor[0] = light.getIntensity()[0] * color_to_shade[0];
            lcolor[1] = light.getIntensity()[1] * color_to_shade[1];
            lcolor[2] = light.getIntensity()[2] * color_to_shade[2];

        // SHADOWS
            
            
                // Toon Shading
                 // Number of shading levels
                let threshold = 1.0 / self.thresh as f32;
                let shade = (max / threshold).floor() * threshold;
                final_color = final_color + (lcolor * shade);

    }
    final_color
    }
}

pub type Toon = s_Toon;