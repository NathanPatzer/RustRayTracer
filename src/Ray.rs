use crate::Vec3D::Vec3D;

pub struct Ray
{
    pub dir: Vec3D,
    pub origin: Vec3D,
}

impl Ray
{
    pub fn new(dir: Vec3D, origin: Vec3D) -> Ray
    {
        Ray { dir: (dir), origin: (origin) }
    }
}
