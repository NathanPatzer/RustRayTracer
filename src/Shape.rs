use crate::AABoundingBox::BoundingBox;
use crate::Triangle::Triangle;
use crate::Ray::Ray;
use crate::HitStruct::HitStruct;
use crate::{Sph, Vec3};
use crate::BVHNode::BVHNode;

#[derive(Clone,Debug)]
pub enum Shape
{
    Triangle(Triangle),
    Sphere(Sph),
    BVHNode(BVHNode),
    
}

pub trait Hittable
{
    fn closestHit(&self ,r: &Ray , tMin: f32 , tMax: f32,h: &mut HitStruct ) -> bool;
    fn getShaderName(&self) -> String;
    fn getBoundingBox(&self) -> BoundingBox;
    fn getCentroid(&self) -> Vec3;
    fn anyHit(&self,r: &Ray,tMin: f32, tMax: f32) -> bool;
}

impl Hittable for Shape
{
    fn closestHit(&self, r: &Ray, tMin: f32, tMax: f32, h: &mut HitStruct) -> bool 
    {
        match self 
        {
            Shape::Triangle(triangle) => triangle.closestHit(r, tMin, tMax, h),
            Shape::Sphere(sphere) => sphere.closestHit(r, tMin, tMax, h),
            Shape::BVHNode(BVH)=> BVH.closestHit(r, tMin, tMax, h), 
        }
    }

    fn getShaderName(&self) -> String 
    {
        match self
        {
            Shape::Triangle(t) => t.getShaderName(),
            Shape::Sphere(s) => s.getShaderName(),
            Shape::BVHNode(b)=> b.getShaderName(),
            
        }
    }

    fn getBoundingBox(&self) -> BoundingBox
    {
        match self
        {
            Shape::Triangle(t) => t.getBoundingBox(),
            Shape::Sphere(s) => s.getBoundingBox(),
            Shape::BVHNode(b)=> b.getBoundingBox(),
            
        }
    }

    fn getCentroid(&self) -> Vec3 
    {
        match self
        {
            Shape::Triangle(t) => t.getCentroid(),
            Shape::Sphere(s) => s.getCentroid(),
            Shape::BVHNode(b)=> b.getCentroid(),
           
        }    
    }
    
    fn anyHit(&self,r: &Ray,tMin: f32, tMax: f32) -> bool {
        match self 
        {
            Shape::Triangle(triangle) => triangle.anyHit(r, tMin, tMax),
            Shape::Sphere(sphere) => sphere.anyHit(r, tMin, tMax),
            Shape::BVHNode(BVH)=> BVH.anyHit(r, tMin, tMax), 
        }
    }

}