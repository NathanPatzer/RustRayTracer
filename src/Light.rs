use crate::{Vec3, PointLight, HStruct,AreaLight};
#[derive(Clone,Copy)]
pub enum Light
{
    PointLight(PointLight),
    AreaLight(AreaLight)
}

pub trait IsLight
{
    fn getPos(&self) -> Vec3;
    fn getIntensity(&self) -> Vec3;
    fn getContribution(&self,h: &mut HStruct, p: Vec3) -> f32;
}

impl IsLight for Light
{
    fn getIntensity(&self) -> Vec3
    {
        match self
        {
            Light::PointLight(p) => p.getIntensity(),
            Light::AreaLight(a) => a.getIntensity()
        }    
    }

    fn getPos(&self) -> Vec3 
    {
        match self
        {
            Light::PointLight(p) => p.getPos(),
            Light::AreaLight(a) => a.getPos()
        }
    }

    fn getContribution(&self,h: &mut HStruct, pt: Vec3) -> f32
    {
        match self
        {
            Light::PointLight(p) => p.getContribution(h,pt),
            Light::AreaLight(a) => a.getContribution(h, pt)
        }
    }
}