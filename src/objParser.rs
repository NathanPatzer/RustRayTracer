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

    pub fn parse_obj(&mut self,obj_file_path: String,shader_ref: &String,interpolate_on: bool)
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

        let mut vertex_normals: Vec<Vec3> = Vec::new();
        if interpolate_on
        {
            vertex_normals = OBJ::calculate_vertex_normals(&faces,verticies.len() as i32,&verticies);
        }
        
        for face in &faces
        {
            let mut triangle_to_push: Tri = Tri::new(verticies[face.0 as usize], verticies[face.1 as usize], verticies[face.2 as usize], shader_ref.clone(), shader_ref.clone());
            
            if interpolate_on
            {
                let tri_norms: (Vec3,Vec3,Vec3) = (vertex_normals[face.0 as usize],vertex_normals[face.1 as usize],vertex_normals[face.2 as usize]);
                triangle_to_push.setInterpolateON();
                triangle_to_push.setVNormals(tri_norms);
            }

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

        fn calculate_vertex_normals(f: &Vec<(i32,i32,i32)>,num_verts: i32, verts: &Vec<Vec3>) -> Vec<Vec3>
        {
            let mut face_vec: Vec<(i32,i32,i32)> = Vec::new();
            let mut normal_vec: Vec<Vec3> = Vec::new();
            let mut vertex_normals: Vec<Vec3> = Vec::new();
            //for every verticie, check if is contained in a face
            for i in 0..num_verts
            {
                for face in f
                {
                    if face.0 == i || face.1 == i || face.2 == i
                    {
                        face_vec.push((face.0,face.1,face.2));
                    }
                }
                for tri in &face_vec
                {
                    normal_vec.push(OBJ::calcualteFaceNormal(verts[tri.0 as usize], verts[tri.1 as usize], verts[tri.2 as usize]));
                }
                let mut norm_total: Vec3 = Vec3::newEmpty();
                for norm in &normal_vec
                {
                    norm_total = norm_total + norm;
                }
                norm_total = norm_total / face_vec.len() as f32;
                vertex_normals.push(norm_total.normalize());
                normal_vec.clear();
                face_vec.clear();
            }

            vertex_normals
        }

        fn calcualteFaceNormal(A: Vec3, B: Vec3, C: Vec3) -> Vec3
        {
            let a = B - A;
            let b = C - A;
            a.crossProduct(&b).normalize()
        }
}

pub type OBJ = OBJParser;