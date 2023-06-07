use std::{f64::consts::PI};

use crate::Vec3D::Vec3D;
use crate::CoordSys::CoordSys;
use crate::Ray::Ray;
use crate::Camera::CanGenRay;
use libm::{self, tan};
#[derive(Clone,Copy)]
#[allow(non_camel_case_types)]
pub struct lookAtCam
{
    pos: Vec3D,
    coord: CoordSys,
    Horizontal: Vec3D,
    Vertical: Vec3D
}

impl lookAtCam
{
    pub fn new(origin: Vec3D, vfov: f64, aspect_ratio: f32, coordsys: CoordSys) -> lookAtCam
    {
        let theta: f64 = lookAtCam::degrees_to_radians(vfov);
        let H = tan(theta / 2.0);
        let viewH = (2.0 * H) as f32;
        let viewW = aspect_ratio * viewH;
        let vertical = coordsys.V * viewH;
        let horizontal = coordsys.U * viewW;

        lookAtCam { pos: origin,coord: coordsys, Horizontal: horizontal, Vertical: vertical }
    }

    fn degrees_to_radians(degrees: f64) -> f64
    {
        (degrees * PI) / 180.0
    }
}

impl CanGenRay for lookAtCam
{
    fn genRay(&self,s: f32, t: f32) -> Ray {
        let lower_left_corner = self.pos - (self.Horizontal/2.0) - (self.Vertical/2.0) - self.coord.W;
        let dir = lower_left_corner + (self.Horizontal*s) + (self.Vertical*t) - self.pos;
        Ray::new(dir, self.pos)
    }
}