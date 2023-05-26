use crate::{HStruct, Vec3, Shading, Shape::Shape};

use std::collections::HashMap;
use crate::Shader::Shader;
use crate::Light::Light;
use crate::Camera::Camera;
use crate::Ray::Ray;
use crate::Shape::Hittable;
use crate::BVHNode::BVHNode;
#[derive(Clone)]
pub struct SceneContainer
{
    pub allShapes: Vec<Shape>,
    pub allShaders: HashMap<String,Shader>,
    pub allLights: Vec<Light>,
    pub allCameras: Vec<Camera>,
    pub background_color: Vec3,
    pub root: Option<BVHNode>
}



impl SceneContainer
{
    pub fn new()->SceneContainer
    {
        let v: Vec<Shape> = Vec::new();
        let s: HashMap<String,Shader> = HashMap::new();
        let l: Vec<Light> = Vec::new();
        let c: Vec<Camera> = Vec::new();
        
        SceneContainer { allShapes: v, allShaders: s, allLights: l, allCameras: c, background_color: Vec3::newEmpty(),root: None }
    }

    pub fn addShape(&mut self, shape: Shape)
    {
        match shape
        {
            Shape::Sphere(s) => self.allShapes.push(Shape::Sphere(s)),
            Shape::Triangle(t) => self.allShapes.push(Shape::Triangle(t)),
            Shape::BVHNode(b) => self.allShapes.push(Shape::BVHNode(b))
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
                    Some(_old_shader) => println!("Replaced old shader")
                }
            }
            Shader::Mirror(M) =>
            {
                match self.allShaders.insert(name, Shader::Mirror(M))  
                {
                    None => (),
                    Some(_old_shader) => println!("Replaced old shader")
                }
            }
            Shader::Toon(T)=>
            {
                match self.allShaders.insert(name, Shader::Toon(T))
                {
                    None => (),
                    Some(_old_shader) => println!("Replaced old shader")
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

    pub fn rayColor(&self,r: Ray,minT: f32, maxT: f32,_depth: i32, h: &mut HStruct) -> Vec3
    {
        let mut shader: Option<&Shader> =  None;
        let mut didHit: bool = false;
        h.setRoot(self.root.clone());
        assert!(h.scene.root.is_some(), "ROOT IS NONE IN SCENE");
        
        if self.root.clone().unwrap().closestHit(&r, minT, maxT, h)
        {
            didHit = true;
            shader = Some(self.allShaders.get(&h.getShaderName()).expect("INVALID SHADER"));
        }
        if didHit
        {
            shader.unwrap().apply(h)
        }
        else 
        {
            self.background_color
        }
    }


}