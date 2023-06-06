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

    fn getContribution(&self,h: &mut crate::HStruct, intersection: Vec3) -> f32 {
        let shadow_ray = Ray::new(self.getPos() - intersection, intersection);
        let hit = h.scene.root.clone().unwrap().closestHit(&shadow_ray, 0.0001, 1.0, h);
        if hit {return 0.0}
        else {return 1.0}
    }
}

pub type PointLight = l_PointLight;