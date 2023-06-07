use crate::{Vec3, Tri};
#[allow(non_camel_case_types)]
pub struct sBox
{
    
}

impl sBox
{
    pub fn new(minPt: Vec3, maxPt: Vec3, shader: String) -> Vec<Tri>
    {
        //let shader = shader_name;
        
        let minx = minPt[0];
        let miny = minPt[1];
        let minz = minPt[2];

        let maxx = maxPt[0];
        let maxy = maxPt[1];
        let maxz = maxPt[2];

        //VERTICIES
        let v1 = Vec3::new(minx,maxy,minz);
        let v2 = Vec3::new(maxx,maxy,minz);
        let v3 = Vec3::new(maxx,miny,minz);
        let v4 = Vec3::new(maxx,miny,maxz);
        let v5 = Vec3::new(minx,maxy,maxz);
        let v6 = Vec3::new(minx,miny,maxz);

        //TRIANGLES
        let tri1 = Tri::new(minPt,v2,v3, shader.to_string(),shader.to_string());
        let tri2 = Tri::new(minPt,v1,v2, shader.to_string(),shader.to_string());
        let tri3 = Tri::new(v3,maxPt,v4, shader.to_string(),shader.to_string());
        let tri4 = Tri::new(v3,v2,maxPt, shader.to_string(),shader.to_string());
        let tri5 = Tri::new(v4,v5,v6, shader.to_string(),shader.to_string());
        let tri6 = Tri::new(v4,maxPt,v5, shader.to_string(),shader.to_string());
        let tri7 = Tri::new(v1,minPt,v6, shader.to_string(),shader.to_string());
        let tri8 = Tri::new(v6,v5,v1, shader.to_string(),shader.to_string());
        let tri9 = Tri::new(v1,maxPt,v2, shader.to_string(),shader.to_string());
        let tri10 = Tri::new(v1,v5,maxPt, shader.to_string(),shader.to_string());
        let tri11 = Tri::new(minPt,v3,v4, shader.to_string(),shader.to_string());
        let tri12 = Tri::new(minPt,v4,v6, shader.to_string(),shader.to_string());

        //VECTOR
        let mut box_vec: Vec<Tri> = Vec::new();
        box_vec.push(tri1);
        box_vec.push(tri2);
        box_vec.push(tri3);
        box_vec.push(tri4);
        box_vec.push(tri5);
        box_vec.push(tri6);
        box_vec.push(tri7);
        box_vec.push(tri8);
        box_vec.push(tri9);
        box_vec.push(tri10);
        box_vec.push(tri11);
        box_vec.push(tri12);
        
        box_vec
        
    }
}

pub type Box = sBox;