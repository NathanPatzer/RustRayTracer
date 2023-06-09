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
use crate::l_area::AreaLight;
use crate::lookAtCam::lookAtCam;
use crate::s_Toon::Toon;
use crate::s_mirror::Mirror;
use crate::Texture::Texture;
use crate::OBJ;
use crate::Glaze;
pub struct JsonParser
{
    path: String,
    width: i32,
    height: i32,
    interpolate: bool
}

impl JsonParser
{
    pub fn new(p: String, w: i32, h: i32,interpolate: bool) -> JsonParser
    {
        JsonParser{path: p, width: w, height: h,interpolate: interpolate}
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
        let start = std::time::Instant::now();
        println!("PARSING JSON...");
        let jString = self::JsonParser::readFile(self.path);
        let json: serde_json::Value = serde_json::from_str(&jString).expect("JSON not well formatted");
        let jShapes: Option<&Vec<Value>> = json["scene"]["shape"].as_array();
        let jShaders = json["scene"]["shader"].as_array();
        let jObj = json["scene"]["obj"].as_array();
        let jLights = json["scene"]["light"].as_array();
        let jCameras = json["scene"]["camera"].as_array();
        let jTextures = json["scene"]["texture"].as_array();
        let obj_vec: &Vec<Value> = jObj.unwrap();
        let texture_vec = jTextures.unwrap();
        let camera_vec = jCameras.unwrap();
        let light_vec = jLights.unwrap();
        let shader_vec = jShaders.unwrap();
        let shape_vec = jShapes.unwrap();

        //add background texture to scene if it is present
        if json["scene"].get("background-texture").is_some()
        {
            let background_name = "background".to_string();
            let background_path = JsonParser::getStr(json["scene"].get("background-texture"));
            let texture = Texture::new("TEXTURES/".to_string() + &background_path);
            scene.addTexture(texture, background_name);  
        }
        
        JsonParser::parse_textures(texture_vec, scene);
        JsonParser::parse_cameras(camera_vec, scene,self.width,self.height);
        JsonParser::parse_shapes(shape_vec, scene);
        JsonParser::parse_lights(light_vec, scene);
        JsonParser::parse_shaders(shader_vec, scene);
        JsonParser::parse_obj(obj_vec,scene,self.interpolate);

        println!("Added {} Shapes", scene.allShapes.len());
        println!("Added {} Shaders", scene.allShaders.len());
        println!("Added {} Lights", scene.allLights.len());
        println!("Added {} Textures", scene.allTextures.len());
        let end = std::time::Instant::now();
        println!("Time to parse: {:.2}s", (end - start).as_secs_f32());
    }

    fn parse_textures(texture_vec: &Vec<Value>,scene: &mut SceneContainer)
    {
        for i in 0..texture_vec.len()
        {
            //let texture_name = texture_vec[i].get("_name").unwrap().as_str().unwrap();
            let texture_name = JsonParser::getStr(texture_vec[i].get("_name"));
            let stexture = JsonParser::getStr(texture_vec[i].get("texture"));
            let texture = Texture::new("TEXTURES/".to_string() + &stexture);
            scene.addTexture(texture, texture_name);  
        }
    }

    fn parse_cameras(camera_vec: &Vec<Value>,scene: &mut SceneContainer,w: i32, h: i32)
    {
        for i in 0..camera_vec.len()
        {
            let cam_type = camera_vec[i].get("_type").unwrap().as_str().unwrap();
            if cam_type == "perspective"
            {
                if camera_vec[i].get("lookat").is_none()
                {                
                    let pos = JsonParser::getVec(camera_vec[i].get("position").unwrap().as_str().unwrap());
                    let view_dir = JsonParser::getVec(camera_vec[i].get("viewDir").unwrap().as_str().unwrap());
                    let focal_length = camera_vec[i].get("focalLength").unwrap().as_f64().unwrap();
                    let plane_w = camera_vec[i].get("imagePlaneWidth").unwrap().as_f64().unwrap();
                    let coord_sys = Coord::new(view_dir, Vec3::new(0.0, 1.0, 0.0));
                    let p_cam = PerspectiveCamera::new(pos, plane_w as f32, focal_length as f32, w, h, coord_sys);
                    scene.addCamera(Camera::PerpectiveCamera(p_cam));
                }
                else 
                {
                    let pos = JsonParser::getVec(camera_vec[i].get("position").unwrap().as_str().unwrap());
                    let lookat = JsonParser::getVec(camera_vec[i].get("lookat").unwrap().as_str().unwrap());
                    let vfov = JsonParser::getInt(camera_vec[i].get("vfov"));
                    let aspect: f32 = w as f32 / h as f32;
                    let coord_sys = Coord::new_look_at(pos, lookat, Vec3::new(0.0, 1.0, 0.0));
                    let p_cam = lookAtCam::new(pos, vfov as f64,aspect,coord_sys);
                    scene.addCamera(Camera::lookAtCam(p_cam));
                }
            }
        }
    }

    fn parse_shapes(shape_vec: &Vec<Value>,scene: &mut SceneContainer)
    {
        for i in 0..shape_vec.len()
        {
            
            let shape_type = shape_vec[i].get("_type").unwrap().as_str().unwrap();
            if shape_type == "triangle"
            {
                let v0 = JsonParser::getVec(shape_vec[i].get("v0").unwrap().as_str().unwrap());
                let v1 = JsonParser::getVec(shape_vec[i].get("v1").unwrap().as_str().unwrap());
                let v2 = JsonParser::getVec(shape_vec[i].get("v2").unwrap().as_str().unwrap());
                let shader_name = shape_vec[i].get("shader").unwrap().get("_ref").unwrap().as_str().unwrap().to_string();
                let tri = Tri::new(v0, v1, v2, shader_name.clone(),shader_name);
                scene.addShape(Shape::Triangle(tri));
                

            }
            else if shape_type == "sphere" 
            {
                let center = self::JsonParser::getVec(shape_vec[i].get("center").unwrap().as_str().unwrap());
                let radius = shape_vec[i].get("radius").unwrap().as_f64().unwrap();
                

                if shape_vec[i].get("texture").is_some()
                {
                    let texture_name = shape_vec[i].get("texture").unwrap().get("_ref").unwrap().as_str().unwrap().to_string();
                    let shader_name = shape_vec[i].get("shader").unwrap().get("_ref").unwrap().as_str().unwrap().to_string();
                    let s = Sph::new(center, radius as f32,shader_name.clone(),texture_name);
                    scene.addShape(Shape::Sphere(s));
                }
                else {
                    let shader_name = shape_vec[i].get("shader").unwrap().get("_ref").unwrap().as_str().unwrap().to_string();
                    let s = Sph::new(center, radius as f32,shader_name.clone(),shader_name);
                    scene.addShape(Shape::Sphere(s));
                }
               
            }
            else if shape_type == "box"
            {
                let minPt = JsonParser::getVec(shape_vec[i].get("minPt").unwrap().as_str().unwrap());
                let maxPt = JsonParser::getVec(shape_vec[i].get("maxPt").unwrap().as_str().unwrap());
                let shader_name = shape_vec[i].get("shader").unwrap().get("_ref").unwrap().as_str().unwrap().to_string();
                let box_vec = Box::new(minPt, maxPt,shader_name);
                //PUSH ALL 12 TRIANGLES INTO SHAPEVEC
                for triangle in box_vec
                {
                    scene.addShape(Shape::Triangle(triangle));
                }
                           
            }
        }
    }

    fn parse_lights(light_vec: &Vec<Value>,scene: &mut SceneContainer)
    {
        for i in 0..light_vec.len()
        {
            let light_type = light_vec[i].get("_type").unwrap().as_str().unwrap();
            if light_type == "point"
            {
                let pos = JsonParser::getVec(light_vec[i].get("position").unwrap().as_str().unwrap());
                let intensity = JsonParser::getVec(light_vec[i].get("intensity").unwrap().as_str().unwrap());     
                let l: PointLight = PointLight::new(pos, intensity);
                scene.addLight(Light::PointLight(l));              
            }
            else if light_type == "area"
            {
                let x0 = JsonParser::getFloat(light_vec[i].get("x0"));
                let x1 = JsonParser::getFloat(light_vec[i].get("x1"));
                let y0 = JsonParser::getFloat(light_vec[i].get("y0"));
                let y1 = JsonParser::getFloat(light_vec[i].get("y1"));
                let z = JsonParser::getFloat(light_vec[i].get("z"));
                let s: u32 =JsonParser::getFloat(light_vec[i].get("s")) as u32;
                let axis: u8 =JsonParser::getFloat(light_vec[i].get("axis")) as u8;
                let intensity = JsonParser::getVec(light_vec[i].get("intensity").unwrap().as_str().unwrap());
                let l: AreaLight = AreaLight::new(x0, x1, y0, y1, z, (s,s),intensity,axis);
                scene.addLight(Light::AreaLight(l));
            }
        }
    }

    fn parse_shaders(shader_vec: &Vec<Value>,scene: &mut SceneContainer)
    {
        for i in 0..shader_vec.len()
        {
            let shader_type = shader_vec[i].get("_type").unwrap().as_str().unwrap();
            if shader_type == "Lambertian"
            {
                let mut bleed: Option<i32> = None;
                if shader_vec[i].get("bleed").is_some()
                {
                    bleed = Some(JsonParser::getInt(shader_vec[i].get("bleed")) as i32);
                    
                }
                let shader = Lambertian::new(bleed);
                let name = JsonParser::getStr(shader_vec[i].get("_name"));
                if shader_vec[i].get("diffuse").is_none()
                {
                    scene.addShader(Shader::Lambertian(shader), name);
                }
                else {
                    let diffuse = JsonParser::getVec(shader_vec[i].get("diffuse").unwrap().as_str().unwrap());
                    let texture = Texture::create_diffuse_texture(diffuse);
                    scene.addTexture(texture, name.to_string());
                    scene.addShader(Shader::Lambertian(shader), name);
                }
            }
            else if shader_type == "BlinnPhong"
            {
                let name = JsonParser::getStr(shader_vec[i].get("_name"));
                let specular = JsonParser::getVec(shader_vec[i].get("specular").unwrap().as_str().unwrap());
                let diffuse = JsonParser::getVec(shader_vec[i].get("diffuse").unwrap().as_str().unwrap());
                let phong_exp = shader_vec[i].get("phongExp").unwrap().as_f64().unwrap();
                let shader = BlinnPhong::new(specular, phong_exp as f32);
                let texure = Texture::create_diffuse_texture(diffuse);
                scene.addTexture(texure, name.to_string());
                scene.addShader(Shader::BlinnPhong(shader), name);
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
            else if shader_type == "Toon"
            {
                let thresh = shader_vec[i].get("thresh").unwrap().as_i64().unwrap();
                let shader = Toon::new(thresh as i32);
                let name = JsonParser::getStr(shader_vec[i].get("_name"));
                let color = JsonParser::getVec(shader_vec[i].get("color").unwrap().as_str().unwrap());
                let Texture = Texture::create_diffuse_texture(color);
                scene.addTexture(Texture, name.to_string());
                scene.addShader(Shader::Toon(shader), name);
            }
            else if shader_type == "Glaze"
            {
                let roughness = shader_vec[i].get("roughness")
                    .and_then(|value| value.as_f64())
                    .unwrap_or(0.0);
                let name = JsonParser::getStr(shader_vec[i].get("_name"));
                let specular = JsonParser::getVec(shader_vec[i].get("specular").unwrap().as_str().unwrap());
                let phong_exp = shader_vec[i].get("phongExp").unwrap().as_f64().unwrap();
                let mut bleed: Option<i32> = None;
                if shader_vec[i].get("bleed").is_some()
                {
                    bleed = Some(JsonParser::getInt(shader_vec[i].get("bleed")) as i32);

                }
                let shader = Glaze::new(roughness as f32, bleed,specular,phong_exp as f32);
                if shader_vec[i].get("diffuse").is_none()
                {
                    scene.addShader(Shader::Glaze(shader), name);
                }
                else {
                    let diffuse = JsonParser::getVec(shader_vec[i].get("diffuse").unwrap().as_str().unwrap());
                    let texture = Texture::create_diffuse_texture(diffuse);
                    scene.addTexture(texture, name.to_string());
                    scene.addShader(Shader::Glaze(shader), name);
                }
            }
        }
    }

    fn parse_obj(obj_vec: &Vec<Value>, scene: &mut SceneContainer,interpolate: bool)
    {
        for i in 0..obj_vec.len()
        {
            let obj_file_name = JsonParser::getStr(obj_vec[i].get("obj"));
            let shader_ref = JsonParser::getStr(obj_vec[i].get("shader_ref"));
            let mut roll: Option<f64> = Some(0.0);
            let mut pitch: Option<f64> = Some(0.0);
            let mut yaw: Option<f64> = Some(0.0);
            if obj_vec[i].get("roll").is_some()
            {
                roll = Some(JsonParser::getF64(obj_vec[i].get("roll")));
            }
            if obj_vec[i].get("pitch").is_some()
            {
                pitch = Some(JsonParser::getF64(obj_vec[i].get("pitch")));
            }
            if obj_vec[i].get("yaw").is_some()
            {
                yaw = Some(JsonParser::getF64(obj_vec[i].get("yaw")));
            }
            let mut obj_parser: OBJ = OBJ::new(roll,pitch,yaw);
            if obj_vec[i].get("shift").is_some()
            {
                let shift = JsonParser::getVec(obj_vec[i].get("shift").unwrap().as_str().unwrap());
                obj_parser.setShift(shift);
            }
            else {
                obj_parser.setShift(Vec3::newEmpty());
            }
            if obj_vec[i].get("scale").is_some()
            {
                let scale = obj_vec[i].get("scale").unwrap().as_f64().unwrap() as f32;
                obj_parser.setScale(scale);
            }
            else {
                obj_parser.setScale(1.0);
            }
            obj_parser.parse_obj("OBJ/".to_string() + obj_file_name.as_ref(), &shader_ref,interpolate,scene);
        }
    }

    //helper function that converts "a b c" into a vec3(a,b,c)
    fn getVec(v: &str) -> Vec3
    {
        let parts: Vec<f32> = v.split(' ').map(|s| s.parse().unwrap()).collect();
        Vec3::new(parts[0], parts[1], parts[2])
    }  

    fn getStr(val: Option<&Value>) -> String
    {
        val.unwrap().as_str().unwrap().to_string()
    }

    fn getFloat(val: Option<&Value>) -> f32
    {
        val.unwrap().as_f64().unwrap() as f32
    }

    fn getF64(val: Option<&Value>) -> f64
    {
        val.unwrap().as_f64().unwrap()
    }

    fn getInt(val: Option<&Value>) -> i64
    {
        val.unwrap().as_i64().unwrap()
    }
}