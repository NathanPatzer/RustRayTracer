
#[derive(Clone,Copy,Debug)]
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
        Vec3D::new( (self.rgb[1] * rhs[2]) - (self.rgb[2] * rhs[1]),
                    (self.rgb[2] * rhs[0]) - (self.rgb[0] * rhs[2]),
                    (self.rgb[0] * rhs[1]) - (self.rgb[1] * rhs[0]))
    }

    pub fn dot(&self, rhs: &Vec3D) -> f32
    {
        (self[0] * rhs[0]) + (self[1] * rhs[1]) + (self[2] * rhs[2])
    }

    pub fn normalize(&self) -> Vec3D
    {
        Vec3D::new(
        self.rgb[0] / self.length(),
        self.rgb[1] / self.length(),
        self.rgb[2] / self.length()    
        )
    }

    pub fn print(&self)
    {
        println!("{} {} {}", self.rgb[0], self.rgb[1], self.rgb[2]);
    }

    pub fn equals(&self,rhs: Vec3) -> bool
    {
        if self.rgb[0] == rhs[0] && self.rgb[1] == rhs[1] && self.rgb[2] == rhs[2]
        {
            return true;
        }
        false
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
        Vec3D { rgb: [
            self.rgb[0] * rhs,
            self.rgb[1] * rhs,
            self.rgb[2] * rhs] }
    }
}

impl std::ops::Mul<Vec3D> for Vec3D
{
    type Output = Vec3D;
    fn mul(self, rhs: Vec3D) -> Vec3D {
        Vec3D { rgb: [
            self.rgb[0] * rhs[0],
            self.rgb[1] * rhs[1],
            self.rgb[2] * rhs[2]] }
    }
}

impl std::ops::Mul<&Vec3D> for Vec3D
{
    type Output = Vec3D;
    fn mul(self, rhs: &Vec3D) -> Vec3D {
        Vec3D { rgb: [
            self.rgb[0] * rhs[0],
            self.rgb[1] * rhs[1],
            self.rgb[2] * rhs[2]] }
    }
}

impl std::ops::Div<f32> for Vec3D
{
    type Output = Vec3D;
    fn div(self, rhs: f32) -> Vec3D 
    {
        Vec3D { rgb: [self.rgb[0] / rhs,
                      self.rgb[1] / rhs,
                      self.rgb[2] / rhs] }
    }
}

impl std::ops::Div<f32> for &Vec3D
{
    type Output = Vec3D;
    fn div(self, rhs: f32) -> Vec3D 
    {
        Vec3D { rgb: [
            self.rgb[0] / rhs,
            self.rgb[1] / rhs,
            self.rgb[2] / rhs] }
    }
}

impl std::ops::Div<Vec3D> for Vec3D
{
    type Output = Vec3D;
    fn div(self, rhs: Vec3D) -> Vec3D 
    {
        Vec3D { rgb: [
            self.rgb[0] / rhs[0],
            self.rgb[1] / rhs[1],
            self.rgb[2] / rhs[2]] }
    }
}

impl std::ops::Add<Vec3D> for Vec3D
{
    type Output = Vec3D;
    fn add(self, rhs: Vec3D) -> Self::Output 
    {
        Vec3D::new(
            self.rgb[0] + rhs[0],
            self.rgb[1] + rhs[1],
            self.rgb[2] + rhs[2]
        )
    }
}

impl std::ops::Sub<&Vec3D> for &Vec3D
{
    type Output = Vec3D;
    fn sub(self, rhs: &Vec3D) -> Self::Output 
    {
        Vec3D::new(
            self.rgb[0] - rhs[0],
            self.rgb[1] - rhs[1],
            self.rgb[2] - rhs[2]
        )
    }
}

impl std::ops::Sub<Vec3D> for Vec3D
{
    type Output = Vec3D;
    fn sub(self, rhs: Vec3D) -> Self::Output 
    {
        Vec3D::new(
            self.rgb[0] - rhs[0],
            self.rgb[1] - rhs[1],
            self.rgb[2] - rhs[2]
        )
    }
}

impl std::ops::Mul<f32> for &Vec3D
{
    type Output = Vec3D;
    fn mul(self, rhs: f32) -> Self::Output 
    {
        Vec3D { rgb: [
            self.rgb[0] * rhs,
            self.rgb[1] * rhs,
            self.rgb[2] * rhs] }
    }
}

impl std::ops::Add<f32> for Vec3D
{
    type Output = Vec3D;
    fn add(self, rhs: f32) -> Self::Output
    {
        Vec3::new(
            self.rgb[0] + rhs,
            self.rgb[1] + rhs,
            self.rgb[2] + rhs
        )
    }
}

impl std::ops::Add<&Vec3D> for Vec3D
{
    type Output = Vec3D;
    fn add(self, rhs: &Vec3D) -> Self::Output 
    {
        Vec3::new(
        self.rgb[0] + rhs[0],
        self.rgb[1] + rhs[1],
        self.rgb[2] + rhs[2]
        )
    }
}

impl std::ops::Sub<f32> for Vec3D
{
    type Output = Vec3D;
    fn sub(self, rhs: f32) -> Self::Output 
    {
        Vec3D { rgb: [
            self.rgb[0] - rhs,
            self.rgb[1] - rhs,
            self.rgb[2] - rhs] }
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