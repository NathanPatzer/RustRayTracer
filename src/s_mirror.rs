

use std::collections::HashMap;

use crate::{Vec3, Shading, Ray::Ray, INFINITY, HStruct, Shape::Hittable, Light::Light, Shader::Shader, Texture::Texture};
use rand::{Rng, thread_rng};

#[allow(non_camel_case_types)]
#[derive(Clone,Copy)]
pub struct s_mirror
{
    roughness: f32
}

impl s_mirror
{
    pub fn new(r: f32) -> s_mirror
    {
        s_mirror { roughness: r }
    }
}

impl s_mirror
{
    pub fn mirror_color(r: Ray,min_t: f32, max_t: f32,depth: i32, h: &mut HStruct, lights: &Vec<Light>,shaders: &HashMap<String,Shader>,textures: &HashMap<String,Texture>) -> Vec3
    {
        if depth == 0
        {
            return h.getBackGroundColor();
        }
        let mut new_h = HStruct::new();
        
        if h.getRoot().closestHit(&r, min_t, max_t, &mut new_h)
        {
            let mut color: Vec3 = Vec3::newEmpty();

            if let Some(texture) = textures.get(new_h.getTextureName())
            {
                if texture.isTexture
                {
                    let coords = new_h.getCoords();
                    color = texture.get_texture_color(coords.0, coords.1, texture);
                }
                else {
                    color = texture.get_diffuse_color();
                }
            }
            if let Some(shader) = shaders.get(new_h.getShaderName())
            {
                h.setCoords(new_h.getCoords());
                h.setIntersect(new_h.getIntersect());
                h.setNormal(new_h.getNormal());
                h.setT(new_h.getT());
                h.setRay(new_h.getRay());
                return shader.apply(h, &color, lights, shaders,textures)
            }
        }
            h.getBackGroundColor()
    }
}

impl Shading for s_mirror
{
    fn apply(&self,h_struct: &mut HStruct,_color_to_shade: &Vec3,lights: &Vec<Light>,shaders: &HashMap<String,Shader>,textures: &HashMap<String,Texture>) -> Vec3
    {
        let v = (h_struct.getRay().dir * -1.0).normalize();
        let u = h_struct.getNormal().normalize();
        let r = (v * -1.0) + (u * v.dot(&u) * 2.0);
        let mut rng = thread_rng();
        
        let ray_dir = (r + 
        (u * rng.gen_range(-1.0,1.0) * 0.5 * self.roughness) +
        (v * rng.gen_range(-1.0, 1.0) * 0.5 * self.roughness)).normalize();
        
        let mirror_ray = Ray::new(ray_dir, h_struct.getIntersect());
        let depth = h_struct.getDepth();
        self::s_mirror::mirror_color(mirror_ray, 1.0e-5, INFINITY, depth - 1, h_struct,lights,shaders,textures)
    }
}

pub type Mirror = s_mirror;