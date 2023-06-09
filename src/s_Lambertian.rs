use std::collections::HashMap;
use std::f32::INFINITY;


use crate::Ray::Ray;
use crate::Texture::Texture;
use crate::s_mirror::Mirror;
use crate::{Vec3, HStruct};
use crate::Shader::{Shading, Shader};
use crate::Light::{IsLight, Light};
use rand::rngs::ThreadRng;
use rand::{Rng, thread_rng};


#[allow(non_camel_case_types)]
#[derive(Clone,Copy)]
pub struct s_Lambertian
{
    bleed: bool
}

impl s_Lambertian
{
    pub fn new(b: bool) -> s_Lambertian
    {
        s_Lambertian{bleed: b}
    }

    fn random_vector(rng: &mut ThreadRng) -> Vec3
    {
        
        Vec3::new(rng.gen_range(-1.0, 1.0), rng.gen_range(-1.0, 1.0), rng.gen_range(-1.0, 1.0))
    }

    fn random_in_unit_sphere(rng: &mut ThreadRng) -> Vec3
    {
        loop
        {
            let p = Lambertian::random_vector(rng);
            if p.length_squared() >= 1.0 {continue};
            return p;
        }
    }

    pub fn getAttenuation(intersection: Vec3, normal: Vec3,samples: i32,h_struct: &mut HStruct,depth: i32, lights: &Vec<Light>,shaders: &HashMap<String,Shader>,textures: &HashMap<String,Texture>,rng: &mut ThreadRng) -> Vec3
    {
        
        let mut indirectColor = Vec3::newEmpty();
        for _i in 0..samples
        {
            let mut target = intersection + normal + Lambertian::random_in_unit_sphere(rng);
            if target.nearZero()
            {
                target = normal;
            }
            let ray = Ray::new(target - intersection, intersection);
            indirectColor = indirectColor + Mirror::mirror_color(ray, 1.0e-5, INFINITY, depth,h_struct,lights,shaders,textures);
        }

        indirectColor / samples as f32
    }
}

impl Shading for s_Lambertian
{
    #[allow(implied_bounds_entailment)]
    fn apply(&self,h_struct: &mut HStruct, color_to_shade: &Vec3,lights: &Vec<Light>,shaders: &HashMap<String,Shader>,textures: &HashMap<String,Texture>) -> Vec3 
    {
        let mut finalColor = Vec3::newEmpty();
        let mut rng = thread_rng();
        let intersect = h_struct.getIntersect();
        let normal = h_struct.getNormal();
        let ambient = Vec3::new(0.1, 0.1, 0.1) * color_to_shade;
        let mut indirectColor = Vec3::newEmpty();
        if self.bleed
        {
            if h_struct.getDepth() <= 1
            {
                h_struct.setDepth(1);
            }
            h_struct.setDepth(h_struct.getDepth() - 1); //subtract depth by 1
            let depth = h_struct.getDepth();
            indirectColor = Lambertian::getAttenuation(intersect, normal, 25, h_struct,depth,lights,shaders,textures,&mut rng);
        }
        
        for light in lights
        {
            let mut lcolor = Vec3::newEmpty();
            lcolor[0] = light.getIntensity()[0] * color_to_shade[0];
            lcolor[1] = light.getIntensity()[1] * color_to_shade[1];
            lcolor[2] = light.getIntensity()[2] * color_to_shade[2];

            finalColor = finalColor + ((lcolor * light.getContribution(intersect,normal,&mut rng,h_struct.getRoot())));

        }
        finalColor + ambient + (indirectColor * Vec3::new(0.5, 0.5, 0.5))
    }
}


pub type Lambertian = s_Lambertian;