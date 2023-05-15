use crate::Triangle::Triangle;
use crate::Ray::Ray;
use crate::HitStruct::HitStruct;

pub enum Shape
{
    Triangle(Triangle)
}

pub trait Hittable
{
    fn closestHit(&self ,r: &Ray , tMin: f32 , tMax: f32,h: &mut HitStruct ) -> bool;
}

impl Hittable for Shape
{
    fn closestHit(&self, r: &Ray, tMin: f32, tMax: f32, h: &mut HitStruct) -> bool 
    {
        match self 
        {
            Shape::Triangle(triangle) => triangle.closestHit(r, tMin, tMax, h),
        }
    }
}