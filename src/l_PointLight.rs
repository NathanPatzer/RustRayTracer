use rand::rngs::ThreadRng;

use crate::BVHNode::BVHNode;
use crate::Ray::Ray;
use crate::Shape::Hittable;
use crate::Vec3;
use crate::Light::IsLight;
#[derive(Clone,Copy)]
#[allow(non_camel_case_types)]
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

    fn getContribution(&self, intersection: Vec3,normal: Vec3,_rng: &mut ThreadRng, root: &BVHNode) -> f32 {
        let shadow_ray = Ray::new(self.getPos() - intersection, intersection);
        let L_direction = (self.getPos() - intersection).normalize();
        let ndotl = normal.dot(&L_direction);
        let max: f32 = 0.0_f32.max(ndotl);
        if max == 0.0 {return 0.0;}
        if !root.anyHit(&shadow_ray, 0.0001, 1.0)
        {
            return max;
        }
        0.0
    }

    fn getSpecularContribution(&self,intersection: Vec3,normal: Vec3,phongExp: f32,r: Ray) -> f32 {
            
            let L = (self.getPos() - intersection).normalize();
            let V = (r.dir * -1.0).normalize();
            let H = (L + V).normalize();
            let mut phongMax = 0.0_f32.max(normal.dot(&H));
            phongMax = phongMax.powf(phongExp);
            phongMax
    }
}

pub type PointLight = l_PointLight;