use std::collections::HashMap;

use crate::Texture::Texture;
use crate::Vec3;
use crate::Ray::Ray;
use crate::Shape::Shape;
use crate::Light::Light;
use crate::SceneContainer::SceneContainer;
use crate::Shader::Shader;
use crate::BVHNode;

#[derive(Clone)]
pub struct HitStruct
{
    min_t: f32,
    intersect: Vec3,
    normal: Vec3,
    ray: Ray,
    pub scene: SceneContainer,
    depth: i32,
    background_color: Vec3,
    shader_name: String,
    texture_name: String,
    t_coords: (f32,f32)
}

impl HitStruct
{
    pub fn new() -> HitStruct
    {
        HitStruct{
            min_t: 1.0,
            intersect: Vec3::newEmpty(),
            normal: Vec3::newEmpty(), 
            ray: Ray::new(Vec3::newEmpty(), Vec3::newEmpty()),
            scene: SceneContainer::new(),
            depth: 1,
            background_color: Vec3::newEmpty(),
            shader_name: "".to_string(),
            texture_name: "".to_string(),
            t_coords: (0.0,0.0)
        }
    }

    pub fn setTextureName(&mut self, n: String)
    {
        self.texture_name = n;
    }

    pub fn getTextureName(&self) -> &String
    {
        &self.texture_name
    }

    pub fn setCoords(&mut self, coords: (f32,f32))
    {
        self.t_coords = coords; 
    }

    pub fn getCoords(&self) -> (f32,f32)
    {
        self.t_coords
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
        //self.shapes = shapes;
        self.scene.allShapes = shapes;
    }
    
    pub fn getShapes(&self) -> Vec<Shape>
    {
        //self.shapes[..].to_vec()
        self.scene.allShapes[..].to_vec()
    }

    pub fn setLights(&mut self, lights: Vec<Light>)
    {
        //self.lights = lights;
        self.scene.allLights = lights
    }

    pub fn getLights(&self) -> Vec<Light>
    {
        //self.lights[..].to_vec()
        self.scene.allLights[..].to_vec()
    }

    pub fn setDepth(&mut self, d: i32)
    {
        self.depth = d;
    }

    pub fn getDepth(&self) -> i32
    {
        self.depth
    }

    pub fn setShaders(&mut self, shaders: HashMap<String,Shader>)
    {
        self.scene.allShaders = shaders;
    }

    pub fn getShaders(&self) -> HashMap<String,Shader>
    {
        self.scene.allShaders.clone()
    }

    pub fn setBackGroundColor(&mut self, bg: Vec3)
    {
        self.background_color = bg;
    }

    pub fn getBackGroundColor(&self) -> Vec3
    {
        self.background_color
    }

    pub fn setShaderName(&mut self, shader: String)
    {
        self.shader_name = shader;
    }

    pub fn getShaderName(&self) -> String
    {
        self.shader_name.to_string()
    }

    pub fn setRoot(&mut self,root: Option<BVHNode::BVHNode>)
    {
        self.scene.root = root;
    }

    pub fn getRoot(&self) -> BVHNode::BVHNode
    {
        self.scene.root.clone().unwrap()
    }

    pub fn setTextures(&mut self, textures: HashMap<String,Texture>)
    {
        self.scene.allTextures = textures;
    }

    pub fn getTexture(&self, t: String) -> &Texture
    {
        self.scene.allTextures.get(&t).unwrap()
    }

}

pub type HStruct = HitStruct;