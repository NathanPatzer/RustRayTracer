use crate::Vec3;
use crate::Shape::Hittable;
use crate::HStruct;
use crate::Ray::Ray;
pub struct Sphere
{
    center: Vec3,
    radius: f32
}

impl Sphere
{
    pub fn new(c: Vec3, r: f32) -> Sphere
    {
        Sphere{center: c, radius: r}
    }
}

impl Hittable for Sphere
{
    fn closestHit(&self,r: &Ray, tMin: f32, tMax: f32, h_struct: &mut HStruct) -> bool
    {
        let oc = &r.origin - &self.center;
        let a = r.dir.dot(&r.dir);
        let b = 2.0 * &oc.dot(&r.dir);
        let c = oc.dot(&oc) - (self.radius * self.radius);
        let disc = (b*b) - (4.0*a*c);

        if disc < 0.0
        {
            return false;
        }
        else 
        {
            let T = (-b - disc.sqrt() ) / (2.0*a);

            if T < tMin || T > tMax 
            {
                return false;
            }
            else 
            {
                h_struct.setT(T);
                return true;
            }
        }
    }
}

pub type Sph = Sphere;