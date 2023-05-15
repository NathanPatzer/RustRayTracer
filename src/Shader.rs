use crate::Lambertian;
use crate::HStruct;
use crate::Vec3;
pub enum Shader
{
    Lambertian(Lambertian)
}

pub trait Shading
{
    fn apply(&self,h_struct: &HStruct) -> Vec3;
}

impl Shading for Shader
{
    fn apply(&self,h_struct: &HStruct) -> Vec3 
    {
        match self
        {
            Shader::Lambertian(L) => L.apply(h_struct),
        }    
    }
}