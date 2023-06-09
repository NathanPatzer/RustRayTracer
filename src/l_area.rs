use crate::{Light::IsLight, Vec3, Shape::Hittable,Ray::Ray, BVHNode::BVHNode};
use rand::{Rng, thread_rng, rngs::ThreadRng};
#[allow(non_camel_case_types)]
#[derive(Clone,Debug,Copy)]
pub struct l_area
{
    samples: (u32,u32),
    center: Vec3,
    intensity: Vec3,
    axis: (Vec3,Vec3)
}

impl l_area
{
    pub fn new(x0: f32, x1: f32, y0: f32, y1: f32, z: f32,samples: (u32,u32), intensity: Vec3,axis: u8) -> l_area
    {
        let mut center = Vec3::newEmpty();
        #[allow(unused_assignments)]
        let mut axis1 = Vec3::newEmpty();
        #[allow(unused_assignments)]
        let mut axis2 = Vec3::newEmpty();
        if axis == 0 //yz rect
        {
            center[0] = z;
            center[1] = (x1 + x0) / 2.0;
            center[2] = (y1 + y0) / 2.0;
            axis1 = Vec3::new(0.0,1.0,0.0);
            axis2 = Vec3::new(0.0, 0.0, 1.0);
        }
        else if axis == 1 //xz rect
        {
            center[0] = (x1 + x0) / 2.0;
            center[1] = z;
            center[2] = (y1 + y0) / 2.0;
            axis1 = Vec3::new(1.0,0.0,0.0);
            axis2 = Vec3::new(0.0, 0.0, 1.0);
        }
        else if axis == 2 //xyrect
        {
            center[0] = (x1 + x0) / 2.0;
            center[1] = (y1 + y0) / 2.0;
            center[2] = z;
            axis1 = Vec3::new(1.0,0.0,0.0);
            axis2 = Vec3::new(0.0, 1.0, 0.0);
        }
        else {
            panic!("Invalid Axis");
        }
       l_area {samples: samples,center:center,intensity:intensity,axis: (axis1,axis2) }
    }

}

impl IsLight for l_area
{
    //https://github.com/dcower/raytracer/blob/master/raytracer/AreaLight.cpp getContribution Method
    fn getContribution(&self, intersectionPt: Vec3, normal: Vec3,rng: &mut ThreadRng,root: &BVHNode) -> f32 {
        let invSamplesX = 1.0 / self.samples.0 as f32;
        let invSamplesY = 1.0 / self.samples.1 as f32;
        let mut shading_contribution: f32 = 0.0;
        let mut diffuse_contribution: f32 = 0.0;
        let mut distance_contribution: f32 = 0.0;
        let totalSamples = self.samples.0 as f32 * self.samples.1 as f32;
        for x in 0..self.samples.0
        {
            for y in 0..self.samples.1
            {
                let lightP = self.center + 
                self.axis.0 * ((x as f32 + 0.5) * invSamplesX + (rng.gen::<f32>() - 0.5) * invSamplesX - 0.5 ) * self.axis.0.length() + 
                self.axis.1 * ((y as f32 + 0.5) * invSamplesY + (rng.gen::<f32>() - 0.5) * invSamplesY - 0.5 ) * self.axis.1.length();
                let ltop: Vec3 = lightP - intersectionPt;
                distance_contribution = distance_contribution + ltop.length();
                let L = ltop.normalize();
                let ndotl = normal.dot(&L);
                let max: f32 = 0.0_f32.max(ndotl);
                if max == 0.0 {continue;}
                
                diffuse_contribution = diffuse_contribution + max;

                let shadow_ray = Ray::new(ltop, intersectionPt);
                if !root.anyHit(&shadow_ray, 0.0001, 1.0)
                {
                    shading_contribution = shading_contribution + 1.0;
                }
            }
        }

        let total_distance: f32 = distance_contribution / totalSamples;
        let total_shading: f32 = shading_contribution / totalSamples;
        let total_diffuse: f32 = diffuse_contribution / totalSamples;
        let totalContribution: f32 = (total_shading * total_diffuse) / (total_distance * total_distance);
        return totalContribution;
    }

    fn getIntensity(&self) -> Vec3 {
        self.intensity
    }
    fn getPos(&self) -> Vec3 {
        self.center
    }

    //estimates phongMax
    fn getSpecularContribution(&self,intersection: Vec3,normal: Vec3,phongExp: f32,r: Ray) -> f32 {
        let samplesX = self.samples.0;
        let samplesY = self.samples.1;
        let invSamplesX = 1.0 / samplesX as f32;
        let invSamplesY = 1.0 / samplesY as f32;
        let mut specular_contribution = 0.0;
        let mut rng = thread_rng();
        for x in 0..samplesX
        {
            for y in 0..samplesY
            {
                let first = self.axis.0 * ((x as f32 + 0.5) * invSamplesX + (rng.gen::<f32>() - 0.5) * invSamplesX - 0.5 ) * self.axis.0.length();
                let second = self.axis.1 * ((y as f32 + 0.5) * invSamplesY + (rng.gen::<f32>() - 0.5) * invSamplesY - 0.5 ) * self.axis.1.length();
                let lightP = self.center + first + second;
                
                let L = (lightP - intersection).normalize();
                let V = (r.dir * -1.0).normalize();
                let H = (L + V).normalize();
                let mut phongMax = 0.0_f32.max(normal.dot(&H));
                phongMax = phongMax.powf(phongExp);
                specular_contribution = specular_contribution + phongMax;
            }
        }
        specular_contribution / (samplesX as f32 * samplesY as f32)
    }
}

pub type AreaLight = l_area;