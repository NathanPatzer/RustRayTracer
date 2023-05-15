use crate::Triangle::Triangle;
use crate::Shape::Shape;
use crate::Sph;
pub struct SceneContainer
{
    pub shapes: Vec<Shape>
}



impl SceneContainer
{
    pub fn new()->SceneContainer
    {
        let v:Vec<Shape> = Vec::new();
        SceneContainer { shapes: v }
    }
    pub fn addTriangle(&mut self, tri: Triangle)
    {
        self.shapes.push(Shape::Triangle(tri));
    } 

    pub fn addSpheres(&mut self, s: Sph)
    {
        self.shapes.push(Shape::Sphere(s))
    }
}