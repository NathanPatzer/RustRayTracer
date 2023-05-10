use crate::Vec3D::Vec3D;
use crate::CoordSys::CoordSys;
use crate::Ray::Ray;
pub struct PerspectiveCamera
{
    pos: Vec3D,
    planeW: f32,
    planeH: f32,
    focalLength: f32,
    dimX: i32,
    dimY: i32,
    coord: CoordSys

}

impl PerspectiveCamera
{
    pub fn new(pos: Vec3D, planeW: f32, focalLength: f32, dimX: i32, dimY: i32, coord: CoordSys) -> PerspectiveCamera
    {
        let planeH = planeW * (dimY / dimX) as f32;
        PerspectiveCamera { pos: (pos), planeW: (planeW), planeH: (planeH), focalLength: (focalLength), dimX: (dimX), dimY: (dimY), coord: (coord) }
    }

    pub fn genRay(&self, i: i32, j: i32, offI: f32, offJ: f32) -> Ray
    {
        let left: f32 = -self.planeW / 2.0;
        let right: f32 = self.planeW / 2.0;
        let bottom: f32 = -self.planeH / 2.0;
        let top: f32 = self.planeW / 2.0;
        
        let u: f32 = left + (right-left) * ((i as f32 + offI) / self.dimX as f32) as f32;
        let v: f32 = bottom + (top - bottom) * ((j as f32 + offJ) / self.dimY as f32) as f32;

        let dir: Vec3D = (self.coord.W.clone() * -1.0 * self.focalLength) + (self.coord.U.clone() * u) + (self.coord.V.clone() * v);
        Ray::new(dir, self.pos.clone())
    }
}