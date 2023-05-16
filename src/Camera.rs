use crate::PerspectiveCamera;
use crate::Ray::Ray;
pub enum Camera
{
    PerpectiveCamera(PerspectiveCamera)
}

pub trait canGenRay
{
    fn genRay(&self, i: i32, j: i32, offI: f32, offJ: f32) -> Ray;
}

impl canGenRay for Camera
{
    fn genRay(&self, i: i32, j: i32, offI: f32, offJ: f32) -> Ray
    {
        match self
        {
            Camera::PerpectiveCamera(p) => p.genRay(i, j, offI, offJ)
        }
    }
}