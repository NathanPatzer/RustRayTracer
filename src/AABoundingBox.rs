use crate::{Vec3};
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
        else {
            return true;
        }
        
    }

    pub fn union(&self, rBox: AABoundingBox) -> AABoundingBox
    {
        let mut uMin: Vec3 = Vec3::newEmpty();
        let mut uMax: Vec3 = Vec3::newEmpty();

        //MIN
        uMin[0] = self.minPt[0].min(rBox.minPt[0]);
        uMin[1] = self.minPt[1].min(rBox.minPt[1]);
        uMin[2] = self.minPt[2].min(rBox.minPt[2]);

        //MAX
        uMax[0] = self.maxPt[0].max(rBox.maxPt[0]);
        uMax[1] = self.maxPt[1].max(rBox.maxPt[1]);
        uMax[2] = self.maxPt[2].max(rBox.maxPt[2]);
        let eps = 0.00001;

        for i in 0..3
        {
            if uMax[i] + eps > uMin[i] && uMax[i] - eps < uMin[i]
            {
                uMax[i] += eps;
                uMin[i] -= eps;
            }
        }

        AABoundingBox { minPt: uMin, maxPt: uMax }

    }
}

pub type BoundingBox = AABoundingBox;