
use std::collections::HashMap;

use crate::Lambertian;
use crate::BlinnPhong;
use crate::HStruct;
use crate::Texture::Texture;
use crate::Vec3;
use crate::Light::Light;
use crate::Toon;
use crate::Glaze;
use crate::Mirror;
#[derive(Clone,Copy)]
pub enum Shader
{
    Lambertian(Lambertian),
    BlinnPhong(BlinnPhong),
    Mirror(Mirror),
    Toon(Toon),
    Glaze(Glaze)
}

pub trait Shading
{
    fn apply(&self,h_struct: &mut HStruct, color_to_shade: &Vec3,lights: &Vec<Light>,shaders: &HashMap<String,Shader>,textures: &HashMap<String,Texture>) -> Vec3;
}

impl Shading for Shader
{
    fn apply(&self,h_struct: &mut HStruct,color_to_shade: &Vec3,lights: &Vec<Light>,shaders: &HashMap<String,Shader>,textures: &HashMap<String,Texture>) -> Vec3 
    {
        match self
        {
            Shader::Lambertian(L) => L.apply(h_struct,color_to_shade,lights,shaders,textures),
            Shader::BlinnPhong(B) => B.apply(h_struct,color_to_shade,lights,shaders,textures),
            Shader::Mirror(M) => M.apply(h_struct,color_to_shade,lights,shaders,textures),
            Shader::Toon(T) => T.apply(h_struct,color_to_shade,lights,shaders,textures),
            Shader::Glaze(G) => G.apply(h_struct,color_to_shade,lights,shaders,textures)
        }    
    }
}
