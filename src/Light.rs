use rand::rngs::ThreadRng;

use crate::{Vec3, PointLight,AreaLight,Ray::Ray, BVHNode::BVHNode};
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
    fn getContribution(&self, p: Vec3,normal: Vec3,rng: &mut ThreadRng,root: &BVHNode) -> f32;
    fn getSpecularContribution(&self,intersection: Vec3,normal: Vec3,specular: f32,r: Ray) -> f32;

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

    fn getContribution(&self,pt: Vec3,normal: Vec3,rng: &mut ThreadRng,root: &BVHNode) -> f32
    {
        match self
        {
            Light::PointLight(p) => p.getContribution(pt,normal,rng,root),
            Light::AreaLight(a) => a.getContribution(pt,normal,rng,root)
        }
    }

    fn getSpecularContribution(&self,intersection: Vec3,normal: Vec3,specular: f32,r: Ray) -> f32 {
        match self
        {
            Light::PointLight(p) => p.getSpecularContribution(intersection,normal,specular,r),
            Light::AreaLight(a) => a.getSpecularContribution(intersection,normal,specular,r)
        }
    }

}