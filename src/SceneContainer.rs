use crate::Triangle::Triangle;
use crate::Shape::Shape;

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
}