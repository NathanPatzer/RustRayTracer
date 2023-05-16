use crate::Vec3;
use crate::Light::isLight;
pub struct l_PointLight
{
    pos: Vec3,
    intensity: Vec3
}

impl isLight for l_PointLight
{
    fn getIntensity(&self) -> Vec3 
    {
        self.intensity    
    }

    fn getPos(&self) -> Vec3
    {
        self.pos    
    }
}

pub type PointLight = l_PointLight;