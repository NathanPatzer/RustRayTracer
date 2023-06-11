use rand::{thread_rng, Rng};
use crate::s_mirror::Mirror;
use crate::{Vec3, Shading, Ray::Ray, INFINITY, HStruct, Light::{Light, IsLight}, Shader::Shader, Texture::Texture, Lambertian};
#[allow(non_camel_case_types)]
#[derive(Clone,Copy)]
pub struct s_Glaze
{
    roughness: f32,
    bleed: Option<i32>,
    specular: Vec3,
    phongExp: f32
}

impl s_Glaze
{
    pub fn new(roughness: f32, bleed: Option<i32>,specular: Vec3,phongExp: f32) -> s_Glaze
    {
        s_Glaze{roughness: roughness, bleed,specular: specular, phongExp: phongExp}
    }

    pub fn getMirrorColor(&self,h_struct: &mut HStruct,lights: &Vec<Light>,shaders: &std::collections::HashMap<String,Shader>,textures: &std::collections::HashMap<String,Texture>) -> Vec3
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
        Mirror::mirror_color(mirror_ray, 1.0e-5, INFINITY, depth - 1, h_struct,lights,shaders,textures)
    }

    pub fn getDiffuseColor(&self,h_struct: &mut HStruct, color_to_shade: &Vec3,lights: &Vec<Light>,shaders: &std::collections::HashMap<String,Shader>,textures: &std::collections::HashMap<String,Texture>) -> Vec3
    {
        let ray = h_struct.getRay();
        let mut finalColor = Vec3::newEmpty();
        let mut rng = thread_rng();
        let intersect = h_struct.getIntersect();
        let normal = h_struct.getNormal();
        let ambient = Vec3::new(0.1, 0.1, 0.1) * color_to_shade;
        let mut indirectColor = Vec3::newEmpty();
        if self.bleed.is_some()
        {
            if h_struct.getDepth() <= 1
            {
                h_struct.setDepth(1);
            }
            h_struct.setDepth(h_struct.getDepth() - 1); //subtract depth by 1
            let depth = h_struct.getDepth();
            indirectColor = Lambertian::getAttenuation(intersect, normal, self.bleed.unwrap(), h_struct,depth,lights,shaders,textures,&mut rng);
        }
        
        for light in lights
        {
            let mut lcolor = Vec3::newEmpty();
            lcolor[0] = light.getIntensity()[0] * color_to_shade[0];
            lcolor[1] = light.getIntensity()[1] * color_to_shade[1];
            lcolor[2] = light.getIntensity()[2] * color_to_shade[2];

            //CALCULATE SPECULAR COMPONENET
            let mut specular = Vec3::newEmpty();
            specular[0] = self.specular[0] * light.getIntensity()[0];
            specular[1] = self.specular[1] * light.getIntensity()[1];
            specular[2] = self.specular[2] * light.getIntensity()[2];
            specular = specular * light.getSpecularContribution(intersect, normal, self.phongExp, ray);

            finalColor = finalColor + ((lcolor + specular) * light.getContribution(intersect,normal,&mut rng,h_struct.getRoot()));
        }
        finalColor + ambient + (indirectColor * Vec3::new(0.5, 0.5, 0.5))
    }
}

impl Shading for s_Glaze
{
    fn apply(&self,h_struct: &mut HStruct, color_to_shade: &Vec3,lights: &Vec<Light>,shaders: &std::collections::HashMap<String,Shader>,textures: &std::collections::HashMap<String,Texture>) -> Vec3 {
        let mut new_h = h_struct.clone();
        let diffuse_color = Glaze::getDiffuseColor(&self, &mut new_h, color_to_shade, lights, shaders, textures);
        let mirror_color = Glaze::getMirrorColor(&self, h_struct,  lights, shaders, textures);

        (diffuse_color * 0.8) + (mirror_color * 0.2)
    }
}

pub type Glaze = s_Glaze;