use crate::Vec3D::Vec3D;
#[derive(Clone,Copy)]
pub struct CoordSys
{
    pub U: Vec3D,
    pub V: Vec3D,
    pub W: Vec3D
}

impl CoordSys
{
    pub fn new(gaze: Vec3D, up: Vec3D) -> CoordSys
    {

        let mut w= gaze * -1.0;
        w = &w / w.length();

        let mut u = up.crossProduct(&w);
        u = &u / u.length();

        let mut v = w.crossProduct(&u);
        v = v / v.length();
        
        CoordSys { U: (u), V: (v), W: (w) }
    }

    pub fn new_look_at(lookFrom: Vec3D, lookAt: Vec3D, up: Vec3D) -> CoordSys
    {
        let mut w= lookFrom - lookAt;
        w = &w / w.length();

        let mut u = up.crossProduct(&w);
        u = &u / u.length();

        let mut v = w.crossProduct(&u);
        v = v / v.length();
        
        CoordSys { U: (u), V: (v), W: (w) }
    }
}
pub type Coord = CoordSys;