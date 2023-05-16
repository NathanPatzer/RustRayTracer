use crate::Vec3;

pub struct HitStruct
{
    mint: f32,
    intersect: Vec3
}

impl HitStruct
{
    pub fn new() -> HitStruct
    {
        HitStruct{mint: 1.0, intersect: Vec3::new(0.0, 0.0, 0.0)}
    }

    pub fn setT(&mut self, newT: f32)
    {
        self.mint = newT;
    }

    pub fn getT(&self) -> f32
    {
        self.mint
    }
}

pub type HStruct = HitStruct;