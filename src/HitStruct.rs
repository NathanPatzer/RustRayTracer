use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::Texture::Texture;
use crate::Vec3;
use crate::Ray::Ray;
use crate::Shader::Shader;
use crate::BVHNode::BVHNode;

#[derive(Clone)]
pub struct HitStruct
{
    min_t: f32,
    intersect: Vec3,
    normal: Vec3,
    ray: Ray,
    root: Option<BVHNode>,
    pub shaders: Option<Arc<Mutex<HashMap<String,Shader>>>>,
    pub textures: Option<Arc<Mutex<HashMap<String,Texture>>>>,
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
            min_t: 1.0, //cloned
            intersect: Vec3::newEmpty(), //cloned
            normal: Vec3::newEmpty(), //cloned
            ray: Ray::new(Vec3::newEmpty(), Vec3::newEmpty()), //cloned
            root: None,
            shaders: None,
            textures: None,
            depth: 1, //cloned
            background_color: Vec3::newEmpty(), //referenced
            shader_name: "".to_string(), //cloned
            texture_name: "".to_string(), //cloned
            t_coords: (0.0,0.0) //cloned
        }
    }

    pub fn setTextures(&mut self, textures: HashMap<String,Texture>)
    {
        self.textures = Some(Arc::new(Mutex::new(textures)));
    }

    pub fn setShaders(&mut self, shaders: HashMap<String,Shader>)
    {
        self.shaders = Some(Arc::new(Mutex::new(shaders)));
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
 
    

    pub fn setDepth(&mut self, d: i32)
    {
        self.depth = d;
    }

    pub fn getDepth(&self) -> i32
    {
        self.depth
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

    pub fn setRoot(&mut self,root: Option<BVHNode>)
    {
        self.root = root;
    }

    pub fn getRoot(&self) -> BVHNode
    {
        self.root.clone().unwrap()
    }

}

pub type HStruct = HitStruct;