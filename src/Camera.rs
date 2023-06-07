use crate::PerspectiveCamera;
use crate::Ray::Ray;
use crate::lookAtCam::lookAtCam;
#[derive(Clone,Copy)]
#[allow(non_camel_case_types)]
pub enum Camera
{
    PerpectiveCamera(PerspectiveCamera),
    lookAtCam(lookAtCam)
}

pub trait CanGenRay
{
    fn genRay(&self, u: f32,v: f32) -> Ray;
}

impl CanGenRay for Camera
{
    fn genRay(&self, u: f32,v: f32) -> Ray
    {
        match self
        {
            Camera::PerpectiveCamera(p) => p.genRay(u,v),
            Camera::lookAtCam(l) => l.genRay(u, v)
        }
    }
}