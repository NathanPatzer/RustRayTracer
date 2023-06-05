use std::f32::consts::PI;

use crate::Vec3;
use crate::Light::IsLight;
#[derive(Clone,Copy)]
#[allow(non_camel_case_types)]
pub struct l_SpotLight
{
    pos: Vec3,
    intensity: Vec3,
    _direction: Vec3,
    _angle: f32
}

impl l_SpotLight
{
    pub fn _new(pos: Vec3, intensity: Vec3, dir: Vec3, angle: f32) -> l_SpotLight
    {
        assert!(angle > 0.0 && angle < PI,"SPOTLIGHT ANGLE OUT OF BOUNDS");
        l_SpotLight { pos: pos, intensity: intensity, _direction: dir, _angle: angle }
    }
}

impl IsLight for l_SpotLight
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

#[allow(dead_code)]
pub type SpotLight = l_SpotLight;