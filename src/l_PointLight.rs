use crate::Vec3;
use crate::Light::IsLight;
#[derive(Clone,Copy)]
pub struct l_PointLight
{
    pos: Vec3,
    intensity: Vec3
}

impl l_PointLight
{
    pub fn new(pos: Vec3, intensity: Vec3) -> l_PointLight
    {
        l_PointLight { pos: pos, intensity: intensity }
    }
}

impl IsLight for l_PointLight
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