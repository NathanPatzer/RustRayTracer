use crate::Vec3D::Vec3D;

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
        //W
        //m_W = -1.0 * gaze;
        //m_W = m_W/m_W.length();
        let mut w= gaze * -1.0;
        w = w.clone() / w.clone().length();

        //U
        //m_U = up;
        //m_U = m_U.crossProduct(m_W);
        //m_U = m_U/m_U.length();
        let mut u = up.crossProduct(w.clone());
        u = u.clone() / u.clone().length();

        //V
        //m_V = m_W.crossProduct(m_U);
        //m_V = m_V / m_V.length();
        let mut v = w.clone().crossProduct(u.clone());
        v = v.clone() / v.clone().length();
        
        CoordSys { U: (u), V: (v), W: (w) }


    }
}
pub type Coord = CoordSys;