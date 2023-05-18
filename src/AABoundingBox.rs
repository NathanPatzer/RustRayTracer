use crate::{Vec3, EPS};
use crate::Ray::Ray;
#[derive(Clone,Copy)]
pub struct AABoundingBox
{
    pub minPt: Vec3,
    pub maxPt: Vec3
}

impl AABoundingBox
{
    pub fn new(min_pt: Vec3, max_pt: Vec3) -> BoundingBox
    {
        AABoundingBox { minPt: min_pt, maxPt: max_pt }
    }

    pub fn intersect(&self,ray: &Ray) -> bool
    {
        let invX: f32 = 1.0 / ray.dir[0];
        let invY: f32 = 1.0 / ray.dir[1];
        let invZ: f32 = 1.0 / ray.dir[2];

        let mut txMin: f32 = invX * (self.maxPt[0] - ray.origin[0]);
        let mut txMax: f32 = invX * (self.minPt[0] - ray.origin[0]);
        let mut tyMin: f32 = invY * (self.maxPt[1] - ray.origin[1]);
        let mut tyMax: f32 = invY * (self.minPt[1] - ray.origin[1]);
        let mut tzMin: f32 = invZ * (self.maxPt[2] - ray.origin[2]);
        let mut tzMax: f32 = invZ * (self.minPt[2] - ray.origin[2]);

        if invX >= 0.0
        {
            txMin = invX * (self.minPt[0] - ray.origin[0]);
            txMax = invX * (self.maxPt[0] - ray.origin[0]);
        }

        if invY >= 0.0
        {
            tyMin = invY * (self.minPt[1] - ray.origin[1]);
            tyMax = invY * (self.maxPt[1] - ray.origin[1]);
        }

        if invZ >= 0.0
        {
            tzMin = invZ * (self.minPt[2] - ray.origin[2]);
            tzMax = invZ * (self.maxPt[2] - ray.origin[2]);
        }

        if  (txMin >  tyMax) || (tyMin > txMax) || (txMin > tzMax) || (tzMin > txMax) || (tyMin > tzMax) || (tzMin > tyMax) 
        {
            return false;
        }

        true
    }

    pub fn union(&self, rBox: AABoundingBox) -> AABoundingBox
    {
        let mut uMin: Vec3 = Vec3::newEmpty();
        let mut uMax: Vec3 = Vec3::newEmpty();

        //MINIMUM
        for i in 0..2
        {
            if self.minPt[i] < rBox.minPt[i]
            {
                uMin[i] = self.minPt[i]
            }
            else 
            {
                uMin[i] = rBox.minPt[i];   
            }
        }

        //MAXIMUM
        for i in 0..2
        {
            if self.maxPt[i] < rBox.maxPt[i]
            {
                uMax[i] = self.maxPt[i]
            }
            else 
            {
                uMax[i] = rBox.maxPt[i];   
            }
        }

        for i in 0..2
        {
            if uMax[i] + EPS > uMin[i] && uMax[i] - EPS < uMin[i]
            {
                uMax[i] += EPS;
                uMin[i] += EPS;
            }
        }

        AABoundingBox { minPt: uMin, maxPt: uMax }

    }
}

pub type BoundingBox = AABoundingBox;