use crate::{Vec3, HStruct};
use crate::Shader::Shading;
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
    fn apply(&self,h_struct: &HStruct) -> Vec3 
    {
        Vec3::new(0.0, 0.0, 0.0)    
    }
}


pub type Lambertian = s_Lambertian;