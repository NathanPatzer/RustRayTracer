use std::f32::consts::PI;

use rand::rngs::ThreadRng;

use crate::BVHNode::BVHNode;
use crate::Ray::Ray;
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

    fn getContribution(&self, _p: Vec3,_normal: Vec3,_rng: &mut ThreadRng,_root: &BVHNode) -> f32 {
        0.0
    }
    fn getSpecularContribution(&self,_intersection: Vec3,_normal: Vec3,_specular: f32,_r: Ray) -> f32 {
        0.0
    }

}

#[allow(dead_code)]
pub type SpotLight = l_SpotLight;