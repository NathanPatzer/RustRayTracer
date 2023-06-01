use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::{Vec3};
use crate::Tri;

pub struct OBJParser
{
    tri: Vec<Tri>
}

impl OBJParser
{
    pub fn new() ->OBJParser
    {
        let emptytri: Vec<Tri> = Vec::new(); 
        OBJParser { tri:  emptytri}
    }

    pub fn parse_obj(&mut self,obj_file_path: String,shader_ref: &String)
    {
        let mut verticies: Vec<Vec3> = Vec::new();
        let mut faces: Vec<(i32,i32,i32)> = Vec::new();
        let file_result = File::open(obj_file_path);
        let obj = match file_result {
            Ok(file)=> file,
            Err(error)=> panic!("ERROR OPENING OBJ FILE: {:?}", error)
        };

        let reader = BufReader::new(obj);
        for (_index,line_result) in reader.lines().enumerate()
        {
            let line = match line_result{
                Ok(l) => l,
                Err(error) => panic!("ERROR READING LINE: {:?}", error )
            };
            let line_Vec: Vec<String> = line.split(' ').map(|s| s.parse().unwrap()).collect();
            if line_Vec[0] == "v"
            {
                let vec3 = OBJ::getVec(&line_Vec);
                verticies.push(vec3);
            }
            if line_Vec[0] == "f"
            {
                let face: (i32,i32,i32) = OBJ::getFace(&line_Vec);  
                faces.push(face);
            }
        }

        for face in faces
        {
            let triangle_to_push: Tri = Tri::new(verticies[face.0 as usize], verticies[face.1 as usize], verticies[face.2 as usize], shader_ref.clone(), shader_ref.clone());
            self.tri.push(triangle_to_push);
        }

        
    }

        //helper function that converts line_vec into a vec3
        fn getVec(vert: &Vec<String>) -> Vec3
        {
            Vec3::new(vert[1].parse::<f32>().unwrap(), vert[2].parse::<f32>().unwrap(), vert[3].parse::<f32>().unwrap())
        }

        fn getFace(face: &Vec<String>) -> (i32,i32,i32)
        {
            (face[1].parse::<i32>().unwrap() - 1,face[2].parse::<i32>().unwrap() - 1,face[3].parse::<i32>().unwrap() - 1)
        }  

        pub fn getSceneShapes(&self) -> &Vec<Tri>
        {
            &self.tri
        }
}

pub type OBJ = OBJParser;