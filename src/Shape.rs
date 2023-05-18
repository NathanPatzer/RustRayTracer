use crate::AABoundingBox::BoundingBox;
use crate::Triangle::Triangle;
use crate::Ray::Ray;
use crate::HitStruct::HitStruct;
use crate::{Sph, Vec3};
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
    fn getBoundingBox(&self) -> BoundingBox;
    fn getCentroid(&self) -> Vec3;
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

    fn getBoundingBox(&self) -> BoundingBox
    {
        match self
        {
            Shape::Triangle(t) => t.getBoundingBox(),
            Shape::Sphere(s) => s.getBoundingBox()
        }
    }

    fn getCentroid(&self) -> Vec3 
    {
        match self
        {
            Shape::Triangle(t) => t.getCentroid(),
            Shape::Sphere(s) => s.getCentroid()
        }    
    }

}