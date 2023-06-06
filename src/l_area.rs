use crate::{Light::IsLight, Vec3, Shape::Hittable,Ray::Ray};
use rand::Rng;
#[allow(non_camel_case_types)]
#[derive(Clone,Debug,Copy)]
pub struct l_area
{
    samples: (u32,u32),
    center: Vec3,
    intensity: Vec3
}

impl l_area
{
    pub fn new(x0: f32, x1: f32, y0: f32, y1: f32, z: f32,samples: (u32,u32), intensity: Vec3) -> l_area
    {
        let mut center = Vec3::newEmpty();
        center[0] = (x1 + x0) / 2.0;
        center[1] = (y1 + y0) / 2.0;
        center[2] = z;
       l_area {samples: samples,center:center,intensity:intensity }
    }

    fn randomFloat() -> f32
    {
        let mut rng = rand::thread_rng();
        let random_number: f32 = rng.gen_range(-0.5..0.5);
        random_number
    }
}

impl IsLight for l_area
{
    //https://github.com/dcower/raytracer/blob/master/raytracer/AreaLight.cpp getContribution Method
    fn getContribution(&self,h: &mut crate::HStruct, intersectionPt: crate::Vec3) -> f32 {
        let samplesX = self.samples.0;
        let samplesY = self.samples.1;
        let invSamplesX = 1.0 / samplesX as f32;
        let invSamplesY = 1.0 / samplesY as f32;
        let mut contribution: f32 = 0.0;
        let axis1 = Vec3::new(1.0, 0.0, 0.0);
        let axis2 = Vec3::new(0.0, 1.0, 0.0);

        for x in 0..samplesX
        {
            for y in 0..samplesY
            {
                let first = axis1 * ((x as f32 + 0.5) * invSamplesX + AreaLight::randomFloat() * invSamplesX - 0.5 ) * axis1.length();
                let second = axis2 * ((y as f32 + 0.5) * invSamplesY + AreaLight::randomFloat() * invSamplesY - 0.5 ) * axis2.length();
                let lightP = self.center + first + second;
                
                let shadow_ray = Ray::new(lightP - intersectionPt, intersectionPt);
                if !h.scene.root.clone().unwrap().closestHit(&shadow_ray, 0.0001, 1.0, h)
                {
                    contribution = contribution + 1.0;
                }
            }
        }
        
        return contribution / (samplesX as f32 * samplesY as f32)
    }

    fn getIntensity(&self) -> Vec3 {
        self.intensity
    }
    fn getPos(&self) -> Vec3 {
        self.center
    }
}

pub type AreaLight = l_area;