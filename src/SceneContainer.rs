use crate::Shape::Shape;
use std::collections::HashMap;
use crate::Shader::Shader;
use crate::Light::Light;
use crate::Camera::Camera;
pub struct SceneContainer
{
    pub allShapes: Vec<Shape>,
    pub allShaders: HashMap<String,Shader>,
    pub allLights: Vec<Light>,
    pub allCameras: Vec<Camera>
}



impl SceneContainer
{
    pub fn new()->SceneContainer
    {
        let v: Vec<Shape> = Vec::new();
        let s: HashMap<String,Shader> = HashMap::new();
        let l: Vec<Light> = Vec::new();
        let c: Vec<Camera> = Vec::new();
        SceneContainer { allShapes: v, allShaders: s, allLights: l, allCameras: c }
    }

    pub fn addShape(&mut self, shape: Shape)
    {
        match shape
        {
            Shape::Sphere(s) => self.allShapes.push(Shape::Sphere(s)),
            Shape::Triangle(t) => self.allShapes.push(Shape::Triangle(t))
        }
    }

    pub fn addShader(&mut self, shader: Shader, name: String)
    {
        match shader
        {
            Shader::Lambertian(l) => 
            {
                match self.allShaders.insert(name, Shader::Lambertian(l))
                {
                    None => (),
                    Some(_old_shader) => println!("Replaced old shader"),
                }
            }
            Shader::BlinnPhong(B)=>
            {
                match self.allShaders.insert(name, Shader::BlinnPhong(B))
                {
                    None=> (),
                    Some(_old_shader) => println!("Replaced old shade")
                }
            }
        }
    }

    pub fn addLight(&mut self, light: Light)
    {
        match light
        {
            Light::PointLight(l) => self.allLights.push(Light::PointLight(l)),
        }
    }

    pub fn addCamera(&mut self, cam: Camera)
    {
        match cam
        {
            Camera::PerpectiveCamera(p) => self.allCameras.push(Camera::PerpectiveCamera(p))
        }
    }

    pub fn getCamera(&self) -> Camera
    {
        self.allCameras[0]
    }
}