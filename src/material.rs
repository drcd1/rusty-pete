use crate::math::Vec3;
use crate::primitive::Intersection;
use crate::math::sample_hemisphere_cos;
use crate::math::orthogonal;
use crate::math::Mat3;

use std::f32::consts::PI;

pub trait Material{
    fn eval(&self, it: &Intersection, wi: &Vec3) -> Vec3;
    fn sample(&self, it: &Intersection,r1:f32,r2:f32) -> (Vec3, f32);
    fn pdf(&self, it: &Intersection, sample: &Vec3)->f32;

    fn emit(&self, it: &Intersection) -> Vec3;
}

pub struct DummyMaterial{
    
}

impl Material for DummyMaterial {    
    fn eval(&self, it: &Intersection, wi: &Vec3) -> Vec3 { Vec3::new()}
    fn sample(&self, it: &Intersection,r1:f32,r2:f32) -> (Vec3, f32) {(Vec3::new(),1.0)}
    fn pdf(&self, it: &Intersection, sample: &Vec3)->f32 {1.0}
    fn emit(&self, it: &Intersection) -> Vec3 {Vec3::new()}
}

pub struct DiffuseMaterial {
    pub albedo: Vec3
}
impl DiffuseMaterial {
    pub fn new(albedo: Vec3)->DiffuseMaterial{
        DiffuseMaterial{albedo:albedo}
    }
}

impl Material for DiffuseMaterial {    
    fn eval(&self, it: &Intersection, wi: &Vec3) -> Vec3 {
        self.albedo*(it.n.dot(wi).abs()/PI)
    }
    fn sample(&self, it: &Intersection, r1: f32, r2:f32) -> (Vec3, f32) {
        let mut sample = sample_hemisphere_cos(r1, r2);
        let p = sample.z/PI;

        let mut n = it.n;
        if n.dot(&it.wo) < 0.0 {
            n = it.n*(-1.0);
        }
        let (x,y,z) = orthogonal(it.n);
        let coords = Mat3::from_axis(&y,&z,&x);
        sample = &coords*&sample;

        (sample, p)
    
    }
    fn pdf(&self, it: &Intersection, sample: &Vec3)->f32 {return (sample.dot(&it.n)).abs()/PI}
    fn emit(&self, it: &Intersection) -> Vec3 {Vec3::new()}

}

pub struct EmissionMaterial {
    pub light: Vec3,
    pub intensity: f32
}

impl Material for EmissionMaterial {    
    fn eval(&self, it: &Intersection, wi: &Vec3) -> Vec3 {
        Vec3::new()
    }
    fn sample(&self, it: &Intersection,r1:f32,r2:f32) -> (Vec3, f32) {(Vec3::new(),-1.0)}
    fn pdf(&self, it: &Intersection, sample: &Vec3)->f32 {-1.0}
    fn emit(&self, it: &Intersection) -> Vec3 {self.light*self.intensity}
    
}