use crate::Vec3D::Vec3D;
use crate::Ray::Ray;
pub struct Triangle
{
    A: Vec3D,
    B: Vec3D,
    C: Vec3D
}


impl Triangle
{

    pub fn new(a: Vec3D, b: Vec3D, c: Vec3D) -> Triangle
    {
        Triangle{A: a, B: b, C: c}
    }

    pub fn closestHit(&self,r: Ray, tMin: f32, tMax: f32) -> bool
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

        true
    }
}

