use crate::Triangle::Triangle;
use crate::Shape::Shape;
use crate::Sph;
use std::collections::HashMap;
use crate::Shader::Shader;
pub struct SceneContainer
{
    pub allShapes: Vec<Shape>,
    pub allShaders: HashMap<String,Shader>
}



impl SceneContainer
{
    pub fn new()->SceneContainer
    {
        let v: Vec<Shape> = Vec::new();
        let s: HashMap<String,Shader> = HashMap::new();
        SceneContainer { allShapes: v, allShaders: s }
    }
    pub fn addTriangle(&mut self, tri: Triangle)
    {
        self.allShapes.push(Shape::Triangle(tri));
    } 

    pub fn addSpheres(&mut self, s: Sph)
    {
        self.allShapes.push(Shape::Sphere(s))
    }
}