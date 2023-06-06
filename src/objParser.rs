use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::{Arc, Mutex};
use crate::SceneContainer::SceneContainer;
use crate::Shape::Shape;
use crate::{Vec3};
use crate::Tri;
use rayon::prelude::*;
pub struct OBJParser
{
    shift: Option<Vec3>,
    scale: Option<f32>
}

impl OBJParser
{
    pub fn new() ->OBJParser
    {
        OBJParser {shift: None,scale: None}
    }

    pub fn setShift(&mut self,shift: Vec3)
    {
        self.shift = Some(shift);
    }

    pub fn setScale(&mut self, s: f32)
    {
        self.scale = Some(s);
    }

    pub fn parse_obj(&mut self,obj_file_path: String,shader_ref: &String,interpolate_on: bool,scene: &mut SceneContainer) -> i32
    {
        let mut totalShapes = 0;
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
             let vec3 = OBJ::getVec(&line_Vec,self.shift,self.scale);   
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
            vertex_normals = OBJ::par_calculate_vertex_normals(&faces, &verticies);
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
            totalShapes+=1;
            scene.addShape(Shape::Triangle(triangle_to_push));
        }

        totalShapes
    }

        //helper function that converts line_vec into a vec3
        fn getVec(vert: &Vec<String>, s: Option<Vec3>, sc: Option<f32>) -> Vec3
        {
            let mut shift = Vec3::newEmpty();
            let mut scale = 1.0;
            if s.is_some()
            {
                shift = s.unwrap();
            }
            if sc.is_some()
            {
                scale = sc.unwrap();
            }
            let a = (vert[1].parse::<f32>().unwrap() + shift[0]) * scale;
            let b = (vert[2].parse::<f32>().unwrap() + shift[1]) * scale;
            let c = (vert[3].parse::<f32>().unwrap() + shift[2]) * scale;
            Vec3::new(a, b, c)
        }

        fn getFace(face: &Vec<String>) -> (i32,i32,i32)
        {
            (face[1].parse::<i32>().unwrap() - 1,face[2].parse::<i32>().unwrap() - 1,face[3].parse::<i32>().unwrap() - 1)
        }  

        fn calcualteFaceNormal(A: Vec3, B: Vec3, C: Vec3) -> Vec3
        {
            let a = B - A;
            let b = C - A;
            a.crossProduct(&b).normalize()
        }

        fn par_calculate_vertex_normals(f: &Vec<(i32,i32,i32)>, verts: &Vec<Vec3>) -> Vec<Vec3>
        {
            let vertex_normals: Arc<Mutex<Vec<Vec3>>> = Arc::new(Mutex::new(Vec::with_capacity(verts.len())));
            vertex_normals.lock().unwrap().resize_with(verts.len(), Vec3::newEmpty);
            //for every verticie, check if is contained in a face
            verts.par_iter().enumerate().for_each(|(i,_face)|
            {
                let mut face_vec: Vec<(i32,i32,i32)> = Vec::new();
                let mut normal_vec: Vec<Vec3> = Vec::new();
                for face in f
                {
                    if face.0 == i as i32 || face.1 == i as i32 || face.2 == i as i32
                    {
                        face_vec.push((face.0,face.1,face.2));
                    }
                }
                for tri in &face_vec
                {
                    let normal = OBJ::calcualteFaceNormal(verts[tri.0 as usize], verts[tri.1 as usize], verts[tri.2 as usize]);
                    normal_vec.push(normal);
                }
                
                let mut norm_total: Vec3 = Vec3::newEmpty();
                //TODO: only account for unique normals to account for normals that are on the same plane
                for norm in &normal_vec
                {
                    norm_total = norm_total + norm;
                }
                norm_total = norm_total / normal_vec.len() as f32;
                let mut vertex_normals_lock = vertex_normals.lock().expect("Failed to acquire lock on vertex_normals");
                vertex_normals_lock[i] = norm_total.normalize();
            });
            Arc::try_unwrap(vertex_normals).unwrap().into_inner().unwrap()
        }
}

pub type OBJ = OBJParser;