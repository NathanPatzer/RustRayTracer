use crate::Vec3D::Vec3D;
use crate::Ray::Ray;
use crate::Shape::Hittable;
use crate::HStruct;
use crate::BoundingBox;
#[derive(Clone,Debug)]
pub struct Triangle
{
    A: Vec3D,
    B: Vec3D,
    C: Vec3D,
    normal: Vec3D,
    v_normals: (Vec3D,Vec3D,Vec3D),
    shader_name: String,
    bounding_box: BoundingBox,
    centroid: Vec3D,
    texture_name: String,
    interpolate: bool
}

impl Triangle
{

    pub fn new(a: Vec3D, b: Vec3D, c: Vec3D, shader_name: String,t: String) -> Triangle
    {
        let bbox = self::Triangle::createBoundingBox(a, b, c);
        let centroid = (bbox.minPt + bbox.maxPt) / 2.0;
        let norm = Tri::calcualteNormal(a, b, c);
        Triangle{A: a, B: b, C: c, normal: norm,shader_name: shader_name,bounding_box: bbox,centroid: centroid, texture_name: t, v_normals: (norm,norm,norm),interpolate: false}
    }

    fn calcualteNormal(A: Vec3D, B: Vec3D, C: Vec3D) -> Vec3D
    {
        let a = B - A;
        let b = C - A;
        a.crossProduct(&b).normalize()
    }

    pub fn setInterpolateON(&mut self)
    {
        self.interpolate = true;
    }

    fn createBoundingBox(A: Vec3D, B: Vec3D, C: Vec3D) -> BoundingBox
    {
        let mut minPt = Vec3D::newEmpty();
        minPt[0] = self::Triangle::findMin(A[0], B[0], C[0]);
        minPt[1] = self::Triangle::findMin(A[1], B[1], C[1]);
        minPt[2] = self::Triangle::findMin(A[2], B[2], C[2]);

        let mut maxPt = Vec3D::newEmpty();
        maxPt[0] = self::Triangle::findMax(A[0], B[0], C[0]);
        maxPt[1] = self::Triangle::findMax(A[1], B[1], C[1]);
        maxPt[2] = self::Triangle::findMax(A[2], B[2], C[2]);

        BoundingBox::new(minPt, maxPt)

    }

    fn findMax(a: f32, b: f32, c: f32) -> f32
    {
       a.max(b.max(c))
    }

    fn findMin(a: f32, b: f32, c: f32) -> f32
    {
        a.min(b.min(c))
    }

    pub fn setVNormals(&mut self, norms: (Vec3D,Vec3D,Vec3D))
    {
        self.v_normals.0 = norms.0;
        self.v_normals.1 = norms.1;
        self.v_normals.2 = norms.2;
    }

    pub fn calcualate_barycentric_coords(A: Vec3D, B: Vec3D,C: Vec3D,intersection: Vec3D) -> (Vec3D,Vec3D,Vec3D)
    {
        let v1p = intersection - A;
        let v2p = intersection - B;
        let v3p = intersection - C;

        let area = (B - A).crossProduct(&(C - A)) * 0.5;
        let area1 = v2p.crossProduct(&v3p) * 0.5;
        let area2 = v3p.crossProduct(&v1p) * 0.5;
        let area3 = v1p.crossProduct(&v2p) * 0.5;

        let u = area1 / area;
        let v = area2 / area;
        let w = area3 / area;

        (u,v,w)
    }
}

impl Hittable for Triangle
{
    fn closestHit(&self,r: &Ray, tMin: f32, tMax: f32, h_struct: &mut HStruct) -> bool
    {
        let a = self.A[0] - self.B[0];
        let b = self.A[1] - self.B[1];
        let c = self.A[2] - self.B[2];
        let d = self.A[0] - self.C[0];
        let e = self.A[1] - self.C[1];
        let f = self.A[2] - self.C[2];
        let g = r.dir[0];
        let h = r.dir[1];
        let i = r.dir[2];
        let j = self.A[0] - r.origin[0];
        let k = self.A[1] - r.origin[1];
        let l = self.A[2] - r.origin[2];
        
        //compute M
	    let M = a*((e*i) - (h*f)) + b*((g*f) - (d*i)) + c*((d*h) - (e*g));

        //compute T
        let mut T = f*((a*k) - (j*b)) + e*((j*c) - (a*l)) + d*((b*l)-(k*c));
        T = T/M;
        T = T * -1.0;

        if T < tMin || T > tMax
        {
    		return false; 
        }

        //compute Gamma
        let mut G = i*((a*k)-(j*b)) + h*((j*c)-(a*l)) + g*((b*l)-(k*c));
        G = G/M;
        

        if G < 0.0 || G > 1.0
        {
                return false;
        }

        //compute Beta
        let mut B = j*((e*i) - (h*f)) + k*((g*f)-(d*i)) + l*((d*h)-(e*g));
        B = B/M;

        if B < 0.0 || B > 1.0-G
        {
   		 return false;
	    }

        let intersectionPt = r.origin + (r.dir * T);
        let mut normal = self.normal;
        //TESTING
        if self.interpolate
        {
            let barys = self::Tri::calcualate_barycentric_coords(self.A,self.B,self.C,intersectionPt);
            normal = ((barys.0 * self.v_normals.0) + (barys.1 * self.v_normals.1) + (barys.2 * self.v_normals.2)).normalize();
        }

        //DONE TESTING
        h_struct.setTextureName(self.texture_name.clone());
        h_struct.setT(T);
        h_struct.setNormal(normal);
        h_struct.setIntersect(intersectionPt);
        h_struct.setRay(Ray::new(r.dir, r.origin));
        h_struct.setShaderName(self.shader_name.clone());
        true
    }

    fn anyHit(&self,r: &Ray,tMin: f32, tMax: f32) -> bool {
        let a = self.A[0] - self.B[0];
        let b = self.A[1] - self.B[1];
        let c = self.A[2] - self.B[2];
        let d = self.A[0] - self.C[0];
        let e = self.A[1] - self.C[1];
        let f = self.A[2] - self.C[2];
        let g = r.dir[0];
        let h = r.dir[1];
        let i = r.dir[2];
        let j = self.A[0] - r.origin[0];
        let k = self.A[1] - r.origin[1];
        let l = self.A[2] - r.origin[2];
        
        //compute M
	    let M = a*((e*i) - (h*f)) + b*((g*f) - (d*i)) + c*((d*h) - (e*g));

        //compute T
        let mut T = f*((a*k) - (j*b)) + e*((j*c) - (a*l)) + d*((b*l)-(k*c));
        T = T/M;
        T = T * -1.0;

        if T < tMin || T > tMax
        {
    		return false; 
        }

        //compute Gamma
        let mut G = i*((a*k)-(j*b)) + h*((j*c)-(a*l)) + g*((b*l)-(k*c));
        G = G/M;
        
        if G < 0.0 || G > 1.0
        {
                return false;
        }

        //compute Beta
        let mut B = j*((e*i) - (h*f)) + k*((g*f)-(d*i)) + l*((d*h)-(e*g));
        B = B/M;

        if B < 0.0 || B > 1.0-G
        {
   		 return false;
	    }

        true
    }

    fn getShaderName(&self) -> String 
    {
        self.shader_name.clone()
    }
    
    fn getBoundingBox(&self) -> BoundingBox 
    {
        self.bounding_box
    }

    fn getCentroid(&self) -> crate::Vec3 
    {
        self.centroid
    }

}
pub type Tri = Triangle;