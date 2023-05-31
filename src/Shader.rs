
use crate::Lambertian;
use crate::BlinnPhong;
use crate::HStruct;
use crate::Light::IsLight;
use crate::Shape::Hittable;
use crate::Vec3;
use crate::Light::Light;
use crate::Toon;
use crate::Ray::Ray;
use crate::Mirror;
#[derive(Clone)]
pub enum Shader
{
    Lambertian(Lambertian),
    BlinnPhong(BlinnPhong),
    Mirror(Mirror),
    Toon(Toon)
}

pub trait Shading
{
    fn apply(&self,h_struct: &mut HStruct, color_to_shade: &Vec3) -> Vec3;
}

impl Shading for Shader
{
    fn apply(&self,h_struct: &mut HStruct,color_to_shade: &Vec3) -> Vec3 
    {
        match self
        {
            Shader::Lambertian(L) => L.apply(h_struct,color_to_shade),
            Shader::BlinnPhong(B) => B.apply(h_struct,color_to_shade),
            Shader::Mirror(M) => M.apply(h_struct,color_to_shade),
            Shader::Toon(T) => T.apply(h_struct,color_to_shade)
        }    
    }
}

impl Shader
{
    pub fn shadowRay(light: Light,intersection: Vec3) -> Ray
    {
        Ray::new(light.getPos() - intersection, intersection)
    }

    pub fn anyHit(shadowRay: Ray, tMin: f32, tMax: f32, h: &mut HStruct) -> bool
    {
        h.scene.root.clone().unwrap().closestHit(&shadowRay, tMin, tMax, h)
    }
}