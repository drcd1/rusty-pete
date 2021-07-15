
use crate::math::Vec3;
use crate::material::Material;
use crate::material::DummyMaterial;

use std::option::Option;
use crate::constants::EPS;
use crate::math::solve_quadratic;


pub struct Ray{
    pub o: Vec3,
    pub d: Vec3,
    pub max_t: f32
}

impl Ray {
    pub fn new(o:Vec3, d:Vec3) -> Ray {
        Ray {o:o, d:d, max_t: 10000.0}
    }
}

pub struct Intersection<'a>{
    pub p: Vec3,
    pub n: Vec3,
    pub uv: Vec3,
    pub mat: &'a dyn Material
}

pub trait Primitive {
    fn intersect(&self, r: &mut Ray) -> Option<Intersection>;
    fn intersectAny(&self, r: &mut Ray) -> bool;
}

pub trait Group: Primitive {
    fn add(&mut self, p: Box<dyn Primitive>);
    fn buildIndex(&mut self){}
}


pub struct SimpleGroup{
    primitives: Vec<Box<dyn Primitive>>
} 

impl SimpleGroup{
    pub fn new() -> SimpleGroup {
        return SimpleGroup{primitives: Vec::new()}
    }
}

impl Group for SimpleGroup {
    fn add(&mut self, p: Box<dyn Primitive>){
        self.primitives.push(p);
    }
}

pub struct Sphere {
    pub o: Vec3,
    pub r: f32
}


impl Primitive for Sphere {
    fn intersect(&self, r:&mut Ray) -> Option<Intersection> {
        if !self.intersectAny(r) {
            return None;
        }

        let x = (r.d*r.max_t)+ r.o;
        return Some(Intersection{
            n: (x-self.o).normalized(),
            p: x,
            uv: Vec3::new(),
            mat: &DummyMaterial{}
        })
    }
    
    fn intersectAny(&self, r:&mut Ray) -> bool {
        //  r*r =  (x-o)^2
        //  r*r = ((r.o + d*t- s.o))^2
        //  r*r = ((r.o + d*t)^2  - 2*r.o*s.o - 2*s.o*d*t + s.o^2)      
        //  0 = (r.o^2 + 2*r.o*d*t +d^2*t^2 - 2*r.o*s.o - 2*s.o*d*t + s.o^2 ) 
        // 0 = (r.o^2 + s.o^2 -2*r.o*s.o +  (2*r.o*d - 2*s.o*d)*t + d^2 * t^2

        //  r*r =  (x)^2
        //  r*r = ((r.o + d*t))^2    
        //  0 = (r.o^2 + 2*r.o*d*t +d^2*t^2) -r*r
        let aux = r.o-self.o;

        let a = r.d.lensqr();
        let b = 2.0*(aux.dot(&r.d));
        let c = aux.lensqr() - self.r*self.r;

        let sol = solve_quadratic(a,b,c);

        let d;
        let e;

        

        match sol{
            None => {return false;}
            Some(x) => {d =x[0]; e = x[1];}
        }

        if d<EPS || d>r.max_t {
            if e<EPS || e> r.max_t {
                return false;
            } else {
                r.max_t = e;
                return true;
            }
        } else {
            r.max_t = d;
            return true;
        }


    }
}


impl Primitive for SimpleGroup{
    fn intersect(&self, r:&mut Ray) -> Option<Intersection> {
        let mut it = None;
        for p in &self.primitives{
            let tmp = p.intersect(r);
            if !tmp.is_none(){
                it = tmp;
            }
           // println!("Testing intersection: {}",!it.is_none());
            
        }
        return it;

    }

    
    fn intersectAny(&self, r: &mut Ray) -> bool{
        for p in &self.primitives{
            if p.intersectAny(r){
                return true;
            }
        }
        return false;
    }
}