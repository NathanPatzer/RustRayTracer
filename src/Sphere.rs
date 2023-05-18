use crate::Vec3;
use crate::Shape::Hittable;
use crate::HStruct;
use crate::Ray::Ray;
use crate::BoundingBox;
#[derive(Clone)]
pub struct Sphere
{
    center: Vec3,
    radius: f32,
    shader_name: String,
    bounding_box: BoundingBox,
    centroid: Vec3

}

impl Sphere
{
    pub fn new(c: Vec3, r: f32,shader_name: String) -> Sphere
    {   
        let bbox = self::Sphere::createBoundingBox(c,r);
        Sphere{center: c, radius: r, shader_name: shader_name,bounding_box: bbox,centroid: c}
    }

    pub fn calcualteNormal(r: &Ray, center: Vec3, T: f32) -> Vec3
    {
        let intersect = r.origin + (r.dir * T);
        let norm = intersect - center;
        norm.normalize()
    }

    pub fn createBoundingBox(center: Vec3, radius: f32) -> BoundingBox
    {
        let minPt = center - radius;
        let maxPt = center + radius; 
        BoundingBox::new(minPt, maxPt)
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
                h_struct.setIntersect(r.origin + (r.dir * T));
                h_struct.setNormal(self::Sphere::calcualteNormal(r, self.center, T));
                h_struct.setRay(Ray::new(r.dir, r.origin));
                return true;
            }
        }
    }

    fn getShaderName(&self) -> String 
    {
        self.shader_name.clone()
    }

    fn getBoundingBox(&self) -> BoundingBox 
    {
        self.bounding_box
    }

    fn getCentroid(&self) -> Vec3 
    {
        self.centroid
    }
}

pub type Sph = Sphere;