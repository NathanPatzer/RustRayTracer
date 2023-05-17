use crate::Triangle::Triangle;
use crate::Ray::Ray;
use crate::HitStruct::HitStruct;
use crate::Sph;
#[derive(Clone)]
pub enum Shape
{
    Triangle(Triangle),
    Sphere(Sph)
}

pub trait Hittable
{
    fn closestHit(&self ,r: &Ray , tMin: f32 , tMax: f32,h: &mut HitStruct ) -> bool;
    fn getShaderName(&self) -> String;
}

impl Hittable for Shape
{
    fn closestHit(&self, r: &Ray, tMin: f32, tMax: f32, h: &mut HitStruct) -> bool 
    {
        match self 
        {
            Shape::Triangle(triangle) => triangle.closestHit(r, tMin, tMax, h),
            Shape::Sphere(sphere) => sphere.closestHit(r, tMin, tMax, h)
        }
    }

    fn getShaderName(&self) -> String 
    {
        match self
        {
            Shape::Triangle(t) => t.getShaderName(),
            Shape::Sphere(s) => s.getShaderName()
        }
    }

}