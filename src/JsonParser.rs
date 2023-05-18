use serde_json::Value;
use std::fs::File;
use std::io::Read;
use crate::BlinnPhong;
use crate::Tri;
use crate::Sph;
use crate::Vec3;
use crate::Shape::Shape;
use crate::SceneContainer::SceneContainer;
use crate::Shader::Shader;
use crate::Lambertian;
use crate::PointLight;
use crate::Light::Light;
use crate::Coord;
use crate::Camera::Camera;
use crate::PerspectiveCamera;
use crate::Box;
use crate::s_mirror::Mirror;
pub struct JsonParser
{
    path: String,
    width: i32,
    height: i32
}

impl JsonParser
{
    pub fn new(p: String, w: i32, h: i32) -> JsonParser
    {
        JsonParser{path: p, width: w, height: h}
    }

    pub fn readFile(path: String) -> String
    {
        match File::open(path)
        {
            Ok(mut f) =>
            {
                let mut contents = String::new();
                f.read_to_string(&mut contents).unwrap();
                return contents;
            },
            Err(e) => {
                // Handle the error here (print an error message, etc.)
                panic!("Failed to open JSON file: {}", e);
            }
        };
    } 

    pub fn Parse(self,scene: &mut SceneContainer)
    {
        println!("PARSING JSON...");
        let jString = self::JsonParser::readFile(self.path);
        let json: serde_json::Value = serde_json::from_str(&jString).expect("JSON not well formatted");
        let jShapes: Option<&Vec<Value>> = json["scene"]["shape"].as_array();
        let jShaders = json["scene"]["shader"].as_array();
        let jLights = json["scene"]["light"].as_array();
        let jCameras = json["scene"]["camera"].as_array();
        let camera_vec = jCameras.unwrap();
        let camera_len: usize = camera_vec.len();
        let light_vec = jLights.unwrap();
        let light_len: usize = light_vec.len();
        let shader_vec = jShaders.unwrap();
        let shader_len: usize = shader_vec.len();
        let shape_vec = jShapes.unwrap();
        let shape_len: usize = shape_vec.len();

        //ADDS SHAPES TO SHAPEVECTOR     
        for i in 0..shape_len
        {
            
            let shape_type = shape_vec[i].get("_type").unwrap().as_str().unwrap();
            if shape_type == "triangle"
            {
                let v0 = self::JsonParser::getVec(shape_vec[i].get("v0").unwrap().as_str().unwrap());
                let v1 = self::JsonParser::getVec(shape_vec[i].get("v1").unwrap().as_str().unwrap());
                let v2 = self::JsonParser::getVec(shape_vec[i].get("v2").unwrap().as_str().unwrap());
                let shader_name = shape_vec[i].get("shader").unwrap().get("_ref").unwrap().as_str().unwrap().to_string();
                let tri = Tri::new(v0, v1, v2, shader_name);
                scene.addShape(Shape::Triangle(tri));
            }
            else if shape_type == "sphere" 
            {
                let center = self::JsonParser::getVec(shape_vec[i].get("center").unwrap().as_str().unwrap());
                let radius = shape_vec[i].get("radius").unwrap().as_f64().unwrap();
                let shader_name = shape_vec[i].get("shader").unwrap().get("_ref").unwrap().as_str().unwrap().to_string();
                let s = Sph::new(center, radius as f32,shader_name);
                scene.addShape(Shape::Sphere(s));
            }
            else if shape_type == "box"
            {
                let minPt = self::JsonParser::getVec(shape_vec[i].get("minPt").unwrap().as_str().unwrap());
                let maxPt = self::JsonParser::getVec(shape_vec[i].get("maxPt").unwrap().as_str().unwrap());
                let shader_name = shape_vec[i].get("shader").unwrap().get("_ref").unwrap().as_str().unwrap().to_string();
                let box_vec = Box::new(minPt, maxPt,shader_name);
                //PUSH ALL 12 TRIANGLES INTO SHAPEVEC
                for triangle in box_vec
                {
                    scene.addShape(Shape::Triangle(triangle));
                }            
            }
        }
        println!("Added {} Shapes", shape_len);
        //ADD SHADERS TO SHADERVECTOR
        for i in 0..shader_len
        {
            
            let shader_type = shader_vec[i].get("_type").unwrap().as_str().unwrap();
            if shader_type == "Lambertian"
            {
                let diffuse = self::JsonParser::getVec(shader_vec[i].get("diffuse").unwrap().as_str().unwrap());
                let shader = Lambertian::new(diffuse);
                let name = shader_vec[i].get("_name").unwrap().as_str().unwrap();
                scene.addShader(Shader::Lambertian(shader), name.to_string());
            }
            else if shader_type == "BlinnPhong"
            {
                let diffuse = self::JsonParser::getVec(shader_vec[i].get("diffuse").unwrap().as_str().unwrap());
                let name = shader_vec[i].get("_name").unwrap().as_str().unwrap();
                let specular = self::JsonParser::getVec(shader_vec[i].get("specular").unwrap().as_str().unwrap());
                let phong_exp = shader_vec[i].get("phongExp").unwrap().as_f64().unwrap();
                let shader = BlinnPhong::new(diffuse, specular, phong_exp as f32);
                scene.addShader(Shader::BlinnPhong(shader), name.to_string());
            }
            else if shader_type == "Mirror"
            {
                let roughness = shader_vec[i].get("roughness")
                    .and_then(|value| value.as_f64())
                    .unwrap_or(0.0);
                let name = shader_vec[i].get("_name").unwrap().as_str().unwrap();
                let shader = Mirror::new(roughness as f32);
                scene.addShader(Shader::Mirror(shader), name.to_string());
            }
        }
        println!("Added {} Shaders", shader_len);
        //ADD LIGHTS TO LIGHTVECTOR
        for i in 0..light_len
        {
            
            let light_type = light_vec[i].get("_type").unwrap().as_str().unwrap();
            if light_type == "point"
            {
                let pos = self::JsonParser::getVec(light_vec[i].get("position").unwrap().as_str().unwrap());
                let intensity = self::JsonParser::getVec(light_vec[i].get("intensity").unwrap().as_str().unwrap());     
                let l: PointLight = PointLight::new(pos, intensity);
                scene.addLight(Light::PointLight(l));              
            }
        }
        println!("Added {} Lights", light_len);
        //ADD CAMERA TO CAMERAVECTOR
        for i in 0..camera_len
        {
            let cam_type = camera_vec[i].get("_type").unwrap().as_str().unwrap();
            if cam_type == "perspective"
            {
                let pos = self::JsonParser::getVec(camera_vec[i].get("position").unwrap().as_str().unwrap());
                let view_dir = self::JsonParser::getVec(camera_vec[i].get("viewDir").unwrap().as_str().unwrap());
                let focal_length = camera_vec[i].get("focalLength").unwrap().as_f64().unwrap();
                let plane_w = camera_vec[i].get("imagePlaneWidth").unwrap().as_f64().unwrap();
                let coord_sys = Coord::new(view_dir, Vec3::new(0.0, 1.0, 0.0));
                let p_cam = PerspectiveCamera::new(pos, plane_w as f32, focal_length as f32, self.width, self.height, coord_sys);
                scene.addCamera(Camera::PerpectiveCamera(p_cam));
            }
        }
        println!("Added {} Cameras",camera_len);
        println!("RENDERING...");      
    }

    //helper function that converts "a b c" into a vec3(a,b,c)
    fn getVec(v: &str) -> Vec3
    {
        let parts: Vec<f32> = v.split(' ').map(|s| s.parse().unwrap()).collect();
        Vec3::new(parts[0], parts[1], parts[2])
    }  
}