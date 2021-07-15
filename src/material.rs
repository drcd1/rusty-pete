use crate::math::Vec3;

pub trait Material{
    fn eval(&self, normal: &Vec3, wo:Vec3, wi:Vec3, uv: &Vec3) -> Vec3;
}

pub struct DummyMaterial{
    
}

impl Material for DummyMaterial {    
    fn eval(&self, normal: &Vec3, wo:Vec3, wi:Vec3, uv: &Vec3) -> Vec3 {
        return Vec3::new();
    }
}