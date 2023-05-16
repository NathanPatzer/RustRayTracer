use crate::{Vec3, PointLight};

pub enum Light
{
    PointLight(PointLight)
}

pub trait isLight
{
    fn getPos(&self) -> Vec3;
    fn getIntensity(&self) -> Vec3;
}

impl isLight for Light
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