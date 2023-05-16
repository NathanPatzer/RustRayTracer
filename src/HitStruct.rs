use crate::Vec3;
use crate::Ray::Ray;
use crate::Shape::Shape;
use crate::Light::Light;
#[derive(Clone)]
pub struct HitStruct
{
    min_t: f32,
    intersect: Vec3,
    normal: Vec3,
    ray: Ray,
    shapes: Vec<Shape>,
    lights: Vec<Light>
}

impl HitStruct
{
    pub fn new() -> HitStruct
    {
        let shapes: Vec<Shape> = Vec::new();
        let lights: Vec<Light> = Vec::new();
        //default values
        HitStruct{
            min_t: 1.0,
            intersect: Vec3::newEmpty(),
            normal: Vec3::newEmpty(), 
            ray: Ray::new(Vec3::newEmpty(), Vec3::newEmpty()),
            shapes: shapes,
            lights: lights
        }
    }

    pub fn setT(&mut self, newT: f32)
    {
        self.min_t = newT;
    }

    pub fn getT(&self) -> f32
    {
        self.min_t
    }

    pub fn setIntersect(&mut self, v: Vec3)
    {
        self.intersect = v;
    }

    pub fn getIntersect(&self) -> Vec3
    {
        self.intersect
    }

    pub fn setNormal(&mut self, v: Vec3)
    {
        self.normal = v;
    }

    pub fn getNormal(&self) -> Vec3
    {
        self.normal
    }

    pub fn setRay(&mut self, r: Ray)
    {
        self.ray = r;
    }

    pub fn getRay(&self) -> Ray
    {
        self.ray
    }

    pub fn setShapes(&mut self, shapes: Vec<Shape>)
    {
        self.shapes = shapes;
    }
    
    pub fn getShapes(&self) -> Vec<Shape>
    {
        self.shapes[..].to_vec()
    }

    pub fn setLights(&mut self, lights: Vec<Light>)
    {
        self.lights = lights;
    }

    pub fn getLights(&self) -> Vec<Light>
    {
        self.lights[..].to_vec()
    }

}

pub type HStruct = HitStruct;