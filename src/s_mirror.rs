

use crate::{Vec3, Shading, Ray::Ray, INFINITY, HStruct, Shape::Hittable, Light::Light, Shader::Shader};
use rand::Rng;
#[allow(non_camel_case_types)]
#[derive(Clone,Copy)]
pub struct s_mirror
{
    _roughness: f32
}

impl s_mirror
{
    pub fn new(r: f32) -> s_mirror
    {
        s_mirror { _roughness: r }
    }
}

impl s_mirror
{
    pub fn mirror_color(r: Ray,min_t: f32, max_t: f32,depth: i32, h: &mut HStruct, lights: &Vec<Light>) -> Vec3
    {
        if depth == 0
        {
            return h.getBackGroundColor();
        }
        let mut shader: Option<Shader> = None;
        if h.getRoot().closestHit(&r, min_t, max_t, h)
        {
            let mut color = Vec3::newEmpty();
            if let Some(textures) = &h.textures
            {
                
                let locked_textures = textures.lock().unwrap();
                if let Some(texture) = locked_textures.get(h.getTextureName())
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
            }

            if let Some(shaders) = &h.shaders
            {
                let locked_shaders = shaders.lock().unwrap();
                if let Some(s) = locked_shaders.get(&h.getShaderName())
                {
                    shader = Some(*s);
                }
            }
            return shader.unwrap().apply(h, &color, lights);
        }
            h.getBackGroundColor()
    }
}

impl Shading for s_mirror
{
    fn apply(&self,h_struct: &mut HStruct,_color_to_shade: &Vec3,lights: &Vec<Light>) -> Vec3
    {
        let v = (h_struct.getRay().dir * -1.0).normalize();
        let n = h_struct.getNormal().normalize();
        let r = (v * -1.0) + (n * v.dot(&n) * 2.0);
        let mut rng = rand::thread_rng();
        let lower_range = -1.0 * self._roughness;
        let upper_range = self._roughness;
        let mut rough_r = r;
        
        if self._roughness > 0.0
        {
            let random_perturbation = Vec3::new(
            rng.gen_range(lower_range..upper_range), 
            rng.gen_range(lower_range..upper_range), 
            rng.gen_range(lower_range..upper_range));
            rough_r = rough_r + random_perturbation;
        }

        let mirror_ray = Ray::new(rough_r, h_struct.getIntersect());
        let depth = h_struct.getDepth();
        self::s_mirror::mirror_color(mirror_ray, 1.0e-5, INFINITY, depth - 1, h_struct,lights)
    }
}

pub type Mirror = s_mirror;