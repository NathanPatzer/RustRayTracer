
#[derive(Clone,Copy)]
pub struct Vec3D{
    rgb: [f32; 3]
}

impl Vec3D{
    pub fn new(a: f32, b: f32, c: f32) -> Vec3D
    {
        Vec3D{rgb: [a,b,c]}      
    }

    pub fn newEmpty() -> Vec3D
    {
        Vec3D { rgb: [0.0,0.0,0.0] }
    }

    pub fn length(&self) -> f32
    {
        ((self.rgb[0] * self.rgb[0]) + (self.rgb[1] * self.rgb[1]) + (self.rgb[2] * self.rgb[2])).sqrt()
    }

    pub fn crossProduct(self,rhs: &Vec3D) -> Vec3D
    {
        let mut cross = Vec3D::newEmpty();
        cross[0] = (self.rgb[1] * rhs[2]) - (self.rgb[2] * rhs[1]);
        cross[1] = (self.rgb[2] * rhs[0]) - (self.rgb[0] * rhs[2]);
        cross[2] = (self.rgb[0] * rhs[1]) - (self.rgb[1] * rhs[0]);
        cross //-> return
    }

    pub fn dot(&self, rhs: &Vec3D) -> f32
    {
        (self[0] * rhs[0]) + (self[1] * rhs[1]) + (self[2] * rhs[2])
    }

    pub fn normalize(&self) -> Vec3D
    {
        let mut norm = Vec3D::newEmpty();
        let mut temp = Vec3D::newEmpty();
        temp[0] = self.rgb[0];
        temp[1] = self.rgb[1];
        temp[2] = self.rgb[2];
        
        norm[0] = temp[0] / temp.length();
        norm[1] = temp[1] / temp.length();
        norm[2] = temp[2] / temp.length();
        norm
    }

    pub fn print(&self)
    {
        println!("{} {} {}", self.rgb[0], self.rgb[1], self.rgb[2]);
    }
}

impl std::ops::Index<usize> for Vec3D
{
    type Output = f32;
    fn index(&self, idx: usize) -> &Self::Output {
        assert!(idx < 3, "Index out of range");
        &self.rgb[idx]
    }
}

impl std::ops::IndexMut<usize> for Vec3D {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        assert!(idx < 3, "Index out of range");
        &mut self.rgb[idx]
    }
}

impl std::ops::Mul<f32> for Vec3D
{
    type Output = Vec3D;
    fn mul(self, rhs: f32) -> Vec3D {
        let r = self.rgb[0] * rhs;
        let g = self.rgb[1] * rhs;
        let b = self.rgb[2] * rhs;
        Vec3D { rgb: [r,g,b] }
    }
}

impl std::ops::Div<f32> for Vec3D
{
    type Output = Vec3D;
    fn div(self, rhs: f32) -> Vec3D 
    {
        let r = self.rgb[0] / rhs;
        let g = self.rgb[1] / rhs;
        let b = self.rgb[2] / rhs;
        Vec3D { rgb: [r,g,b] }
    }
}

impl std::ops::Div<f32> for &Vec3D
{
    type Output = Vec3D;
    fn div(self, rhs: f32) -> Vec3D 
    {
        let r = self.rgb[0] / rhs;
        let g = self.rgb[1] / rhs;
        let b = self.rgb[2] / rhs;
        Vec3D { rgb: [r,g,b] }
    }
}

impl std::ops::Add<Vec3D> for Vec3D
{
    type Output = Vec3D;
    fn add(self, rhs: Vec3D) -> Self::Output 
    {
        let mut r: Vec3D = Vec3D::newEmpty(); 
        r[0] = self.rgb[0] + rhs[0];
        r[1] = self.rgb[1] + rhs[1];   
        r[2] = self.rgb[2] + rhs[2];
        r      
    }
}

impl std::ops::Sub<&Vec3D> for &Vec3D
{
    type Output = Vec3D;
    fn sub(self, rhs: &Vec3D) -> Self::Output 
    {
        let mut r: Vec3D = Vec3D::newEmpty();
        r[0] = self.rgb[0] - rhs[0];
        r[1] = self.rgb[1] - rhs[1];
        r[2] = self.rgb[2] - rhs[2];
        r
    }
}

impl std::ops::Sub<Vec3D> for Vec3D
{
    type Output = Vec3D;
    fn sub(self, rhs: Vec3D) -> Self::Output 
    {
        let mut r: Vec3D = Vec3D::newEmpty();
        r[0] = self.rgb[0] - rhs[0];
        r[1] = self.rgb[1] - rhs[1];
        r[2] = self.rgb[2] - rhs[2];
        r
    }
}

impl std::ops::Mul<f32> for &Vec3D
{
    type Output = Vec3D;
    fn mul(self, rhs: f32) -> Self::Output 
    {
        let r = self.rgb[0] * rhs;
        let g = self.rgb[1] * rhs;
        let b = self.rgb[2] * rhs;
        Vec3D { rgb: [r,g,b] }
    }
}

impl std::ops::Add<f32> for Vec3D
{
    type Output = Vec3D;
    fn add(self, rhs: f32) -> Self::Output
    {
        let mut r = Vec3::newEmpty();
        r[0] = self.rgb[0] + rhs;
        r[1] = self.rgb[1] + rhs;
        r[2] = self.rgb[2] + rhs;
        r
    }
}

impl std::ops::Add<&Vec3D> for Vec3D
{
    type Output = Vec3D;
    fn add(self, rhs: &Vec3D) -> Self::Output 
    {
        let mut r = Vec3::newEmpty();
        r[0] = self.rgb[0] + rhs[0];
        r[1] = self.rgb[1] + rhs[1];
        r[2] = self.rgb[2] + rhs[2];
        r
    }
}

impl std::ops::Sub<f32> for Vec3D
{
    type Output = Vec3D;
    fn sub(self, rhs: f32) -> Self::Output 
    {
        let r = self.rgb[0] - rhs;
        let g = self.rgb[1] - rhs;
        let b = self.rgb[2] - rhs;
        Vec3D { rgb: [r,g,b] }
    }
}

impl PartialEq for Vec3D
{
    fn eq(&self, other: &Self) -> bool {
        if self[0] == other[0] && self[1] == other[1] && self[2] == other[2]
        {
            return true;
        }
        else {
            return false;
        }
    }
}
pub type Vec3 = Vec3D; 