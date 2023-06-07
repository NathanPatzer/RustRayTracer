use std::f32::INFINITY;

use crate::Ray::Ray;
use crate::s_mirror::Mirror;
use crate::{Vec3, HStruct};
use crate::Shader::Shading;
use crate::Light::IsLight;
use fastrand::Rng;
#[allow(non_camel_case_types)]
#[derive(Clone)]
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

    fn random_double(min: f32,max: f32) -> f32
    {
        let rng = Rng::new();
        min + (max - min) * rng.f32()
    }

    fn random_vector() -> Vec3
    {
        Vec3::new(Lambertian::random_double(-1.0,1.0), Lambertian::random_double(-1.0,1.0), Lambertian::random_double(-1.0,1.0))
    }

    fn random_in_unit_sphere() -> Vec3
    {
        loop
        {
            let p = Lambertian::random_vector();
            if p.length_squared() >= 1.0 {continue};
            return p;
        }
    }

    fn getAttenuation(intersection: Vec3, normal: Vec3,samples: i32,h_struct: &mut HStruct,depth: i32) -> Vec3
    {
        let mut indirectColor = Vec3::newEmpty();
        for _i in 0..samples
        {
            let target = intersection + normal + Lambertian::random_in_unit_sphere();
            let ray = Ray::new(target - intersection, intersection);
            indirectColor = indirectColor + Mirror::mirror_color(ray, 1.0e-5, INFINITY, depth,h_struct);
        }

        indirectColor / samples as f32
    }
}

impl Shading for s_Lambertian
{
    fn apply(&self,h_struct: &mut HStruct, color_to_shade: &Vec3) -> Vec3 
    {
        let mut finalColor = Vec3::newEmpty();
        let lights = h_struct.getLights();
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
            indirectColor = Lambertian::getAttenuation(intersect, normal, 75, h_struct,depth);
        }
       
        for light in lights
        {
            let mut lcolor = Vec3::newEmpty();
            lcolor[0] = light.getIntensity()[0] * color_to_shade[0];
            lcolor[1] = light.getIntensity()[1] * color_to_shade[1];
            lcolor[2] = light.getIntensity()[2] * color_to_shade[2];

            finalColor = finalColor + ((lcolor * light.getContribution(h_struct,intersect,normal)));

        }
        finalColor + ambient + (indirectColor * Vec3::new(0.5, 0.5, 0.5))
    }
}


pub type Lambertian = s_Lambertian;