use crate::Texture::Texture;
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
    pub allTextures: HashMap<String,Texture>,
    pub background_color: Vec3,
    pub root: BVHNode
}

impl SceneContainer
{
    pub fn new()->SceneContainer
    {
        let v: Vec<Shape> = Vec::new();
        let s: HashMap<String,Shader> = HashMap::new();
        let l: Vec<Light> = Vec::new();
        let c: Vec<Camera> = Vec::new();
        let t: HashMap<String,Texture> = HashMap::new();
        let b: BVHNode = BVHNode::create_empty();
        SceneContainer { allShapes: v, allShaders: s, allLights: l, allCameras: c,allTextures: t, background_color: Vec3::newEmpty(),root: b }
    }

    pub fn addShape(&mut self, shape: Shape)
    {
        match shape
        {
            Shape::Sphere(s) => self.allShapes.push(Shape::Sphere(s)),
            Shape::Triangle(t) => self.allShapes.push(Shape::Triangle(t)),
            Shape::BVHNode(b) => self.allShapes.push(Shape::BVHNode(b)),
           
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
            Shader::Glaze(G)=>
            {
                match self.allShaders.insert(name, Shader::Glaze(G))
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
            Light::AreaLight(a) => self.allLights.push(Light::AreaLight(a))
        }
    }

    pub fn addCamera(&mut self, cam: Camera)
    {
        match cam
        {
            Camera::PerpectiveCamera(p) => self.allCameras.push(Camera::PerpectiveCamera(p)),
            Camera::lookAtCam(l) => self.allCameras.push(Camera::lookAtCam(l))
        }
    }

    pub fn getCamera(&self) -> Camera
    {
        self.allCameras[0]
    }

    pub fn rayColor(&self,r: Ray,minT: f32, maxT: f32, h: &mut HStruct,coords: (f32,f32)) -> Vec3
    {
        if self.root.closestHit(&r, minT, maxT, h)
        {
            let mut color: Vec3 = Vec3::newEmpty();
            if let Some(texture) = self.allTextures.get(h.getTextureName())
            {
                if texture.isTexture
                {
                    let coords = h.getCoords();
                    color = texture.get_texture_color(coords.0, coords.1, texture);
                }
                else {
                   
                    color = texture.get_diffuse_color();
                }
            }

            if let Some(shader) = self.allShaders.get(h.getShaderName()) {
                return shader.apply(h,&color,&self.allLights,&self.allShaders,&self.allTextures);
            }
        }

        if let Some(background) = self.allTextures.get("background")
        {

            return background.get_texture_color(coords.0, coords.1, background);
        }
        else {
            return self.background_color;
        }
        
        
    }

    pub fn addTexture(&mut self, t: Texture,tname: String)
    {
        self.allTextures.insert(tname, t);
    }
}