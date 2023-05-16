use crate::{Vec3, PointLight};
#[derive(Clone,Copy)]
pub enum Light
{
    PointLight(PointLight)
}

pub trait IsLight
{
    fn getPos(&self) -> Vec3;
    fn getIntensity(&self) -> Vec3;
}

impl IsLight for Light
{
    fn getIntensity(&self) -> Vec3
    {
        match self
        {
            Light::PointLight(p) => p.getIntensity()
        }    
    }

    fn getPos(&self) -> Vec3 
    {
        match self
        {
            Light::PointLight(p) => p.getPos()
        }
    }
}