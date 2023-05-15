use serde_json::json;
use serde_json::{Result, Value};
use std::fs::File;
use std::io::Read;
use crate::Tri;
use crate::Sph;
use crate::Vec3;
use crate::SceneContainer::SceneContainer;
pub struct JsonParser
{
    path: String
}

impl JsonParser
{
    pub fn new(p: String) -> JsonParser
    {
        JsonParser{path: p}
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
                panic!("leFailed to open JSON fi: {}", e);
            }
        };
    } 

    pub fn Parse(self,scene: &mut SceneContainer)
    {
        let jString = self::JsonParser::readFile(self.path);
        let json: serde_json::Value = serde_json::from_str(&jString).expect("JSON not well formatted");
        //let v0 = json["Scene"].get("Shape").and_then(Value::as_str).unwrap_or("Unknown");
        let v0 = json["scene"]["shape"][0].get("v0").unwrap().as_str().unwrap();
        let v1 = json["scene"]["shape"][0].get("v1").unwrap().as_str().unwrap();
        let v2 = json["scene"]["shape"][0].get("v2").unwrap().as_str().unwrap();
        //println!("{}", v0.unwrap());
        scene.addTriangle(Tri::new(self::JsonParser::getVec(v0) ,self::JsonParser::getVec(v1),self::JsonParser::getVec(v2)));
    }

    //helper function that converts "a b c" into a vec3(a,b,c)
    fn getVec(v: &str) -> Vec3
    {
        let parts: Vec<f32> = v.split(' ').map(|s| s.parse().unwrap()).collect();
        Vec3::new(parts[0], parts[1], parts[2])
    }  
}