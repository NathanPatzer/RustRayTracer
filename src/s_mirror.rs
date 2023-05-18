use crate::{Vec3, Shading, Ray::Ray, INFINITY, HStruct, Shader::Shader, Shape::Hittable};
#[allow(non_camel_case_types)]
#[derive(Clone)]
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
    fn mirror_color(r: Ray,min_t: f32, mut max_t: f32,depth: i32, h: &mut HStruct) -> Vec3
    {
        if depth == 0
        {
            return h.getBackGroundColor();
        }
        let mut shader: Option<&Shader> =  None;
        let mut did_hit: bool = false;
        let shaders = h.getShaders();
        let shapes = h.getShapes();
        
        for s in shapes.iter()
        {   
            if s.closestHit(&r, min_t, max_t, h)
            {   
                did_hit = true;
                shader = Some(shaders.get(&s.getShaderName()).expect("INVALID SHADER"));
                max_t = h.getT();
            }
        }
        if did_hit
        {
            shader.unwrap().apply(h)
        }
        else 
        {
            return h.getBackGroundColor();
        }
    }
}

impl Shading for s_mirror
{
    fn apply(&self,h_struct: &mut HStruct) -> Vec3
    {

        let v = (h_struct.getRay().dir * -1.0).normalize();
        let n = h_struct.getNormal().normalize();
        let r = (v * -1.0) + (n * v.dot(&n) * 2.0);
        let mirror_ray = Ray::new(r, h_struct.getIntersect());
        let depth = h_struct.getDepth();
        self::s_mirror::mirror_color(mirror_ray, 1.0e-5, INFINITY, depth - 1, h_struct)
    }
}

pub type Mirror = s_mirror;