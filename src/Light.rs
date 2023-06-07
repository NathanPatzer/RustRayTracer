use crate::{Vec3, PointLight, HStruct,AreaLight,Ray::Ray};
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
    fn getContribution(&self,h: &mut HStruct, p: Vec3,normal: Vec3) -> f32;
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

    fn getContribution(&self,h: &mut HStruct, pt: Vec3,normal: Vec3) -> f32
    {
        match self
        {
            Light::PointLight(p) => p.getContribution(h,pt,normal),
            Light::AreaLight(a) => a.getContribution(h, pt,normal)
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