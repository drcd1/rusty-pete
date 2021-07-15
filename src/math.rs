use std::f32::consts::PI;

pub fn float2u8(a: f32) -> u8 {
    return (clamp(a,0.0,0.9999)*255.0) as u8;
}

pub fn clamp<T: PartialOrd>(x: T, a: T, b: T) -> T {
    if x<a {
        return a;
    }
    if x>b {
        return b;
    }
    return x;
}

#[derive(Copy,Clone)]
pub struct Vec3 {
    pub x : f32,
    pub y : f32,
    pub z : f32
}

impl Vec3{
    pub fn new() ->Vec3 {
        Vec3{x:0.,y:0.,z:0.}
    }
    pub fn xyz(x:f32,y:f32,z:f32) ->Vec3 {
        Vec3{x:x,y:y,z:z}
    }
    pub fn from(a: f32) ->Vec3 {
        Vec3{x:a,y:a,z:a}
    }
    
    pub fn dot(&self, rhs: &Vec3) -> f32{
        self.x*rhs.x + self.y*rhs.y + self.z*rhs.z
    }

    pub fn lensqr(&self) -> f32 {
        self.dot(self)
    }

    pub fn len(&self) -> f32 {
        (self.lensqr()).sqrt()
    }
    
    pub fn cross(self, rhs: &Vec3) -> Vec3{
        Vec3 {
            x: self.y*rhs.z - self.z*rhs.y,
            y: self.z*rhs.x - self.x*rhs.z,
            z: self.x*rhs.y - self.y*rhs.x
        }
    }
    

    pub fn normalized(&self) -> Vec3 {
        self/self.len()
    }
}

impl std::ops::Add for &Vec3{
    type Output = Vec3;
    fn add(self, rhs: &Vec3) -> Vec3{
        Vec3{x: self.x+rhs.x,
            y: self.y+rhs.y,
            z: self.z+rhs.z}
    }
}
impl std::ops::Add for Vec3{
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Vec3{
        Vec3{x: self.x+rhs.x,
            y: self.y+rhs.y,
            z: self.z+rhs.z}
    }
}

impl std::ops::Sub for &Vec3 {
    
    type Output = Vec3;
    fn sub(self, rhs: &Vec3) -> Vec3{
        Vec3{x: self.x-rhs.x,
            y: self.y-rhs.y,
            z: self.z-rhs.z}
    }
}
impl std::ops::Sub for Vec3 {
    
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Vec3{
        Vec3{x: self.x-rhs.x,
            y: self.y-rhs.y,
            z: self.z-rhs.z}
    }
}



impl std::ops::Mul for &Vec3 {
    
    type Output = Vec3;
    fn mul(self, rhs: &Vec3) -> Vec3{
        Vec3{x: self.x*rhs.x,
            y: self.y*rhs.y,
            z: self.z*rhs.z}
    }
}

impl std::ops::Mul<f32> for &Vec3 {
    
    type Output = Vec3;
    fn mul(self, rhs: f32) -> Vec3{
        Vec3{x: self.x*rhs,
            y: self.y*rhs,
            z: self.z*rhs}
    }
}
impl std::ops::Mul<f32> for Vec3 {
    
    type Output = Vec3;
    fn mul(self, rhs: f32) -> Vec3{
        Vec3{x: self.x*rhs,
            y: self.y*rhs,
            z: self.z*rhs}
    }
}



impl std::ops::Div<f32> for &Vec3 {
    
    type Output = Vec3;
    fn div(self, rhs: f32) -> Vec3{
        self*(1.0/rhs)
    }
}


pub struct Mat3{
    values: [f32;9]
}

impl Mat3{
    pub fn identity() -> Mat3{
        Mat3{
            values: [1.0,0.0,0.0,
                     0.0,1.0,0.0,
                     0.0,0.0,1.0]
        }
    }

    pub fn from_axis(axis1: &Vec3,axis2: &Vec3,axis3: &Vec3) -> Mat3{
        Mat3 {
            values: [
            axis1.x,axis2.x,axis3.x,
            axis1.y,axis2.y,axis3.y,
            axis1.z,axis2.z,axis3.z
            ]
        }
    }

    pub fn at(&mut self, a: usize, b: usize) -> &mut f32{
        return &mut self.values[a*3+b];
    }

    pub fn at_ro(&self, a: usize, b: usize) -> &f32{
        return &self.values[a*3+b];
    }

    pub fn inverse(&self) -> Mat3{
        let a = self.values[0];
        let b = self.values[1];
        let c = self.values[2];
        let d = self.values[3];   // a b c
        let e = self.values[4];   // d e f
        let f = self.values[5];   // g h i
        let g = self.values[6];
        let h = self.values[7];
        let i = self.values[8];

        let det = a*e*i+d*h*c+b*f*g - c*e*g-d*b*i-a*f*h;

        
        Mat3{
            values:[
                (e*i-f*h), -(b*i-c*h),  (b*f-c*e),
               -(d*i-f*g),  (a*i-c*g), -(a*f-c*d),
                (d*h-e*g), -(a*h-b*g),  (a*e-b*d)
            ]
        }/det
    }

    pub fn transpose(&self) -> Mat3 {
        Mat3{
            values: [
                self.values[0],
                self.values[3],
                self.values[6],

                
                self.values[1],
                self.values[4],
                self.values[7],

                
                self.values[2],
                self.values[5],
                self.values[8],
            ]
        }
    }

}

impl std::ops::Mul<&Vec3> for &Mat3 {
    
    type Output = Vec3;
    fn mul(self, rhs: &Vec3) -> Vec3{
        Vec3 {
            x: self.values[0]*rhs.x+self.values[1]*rhs.y+self.values[2]*rhs.z,
            y: self.values[3]*rhs.x+self.values[4]*rhs.y+self.values[5]*rhs.z,
            z: self.values[6]*rhs.x+self.values[7]*rhs.y+self.values[8]*rhs.z
        }
    }
}

impl std::ops::Mul for &Mat3 {
    
    type Output = Mat3;
    fn mul(self, rhs: &Mat3) -> Mat3{
        Mat3 {
            values: [
                self.values[0]*rhs.values[0]+self.values[1]*rhs.values[3]+self.values[2]*rhs.values[6],
                self.values[0]*rhs.values[1]+self.values[1]*rhs.values[4]+self.values[2]*rhs.values[7],
                self.values[0]*rhs.values[2]+self.values[1]*rhs.values[5]+self.values[2]*rhs.values[8],
                
                self.values[3]*rhs.values[0]+self.values[4]*rhs.values[3]+self.values[5]*rhs.values[6],
                self.values[3]*rhs.values[1]+self.values[4]*rhs.values[4]+self.values[5]*rhs.values[7],
                self.values[3]*rhs.values[2]+self.values[4]*rhs.values[5]+self.values[5]*rhs.values[8],
                
                self.values[6]*rhs.values[0]+self.values[7]*rhs.values[3]+self.values[8]*rhs.values[6],
                self.values[6]*rhs.values[1]+self.values[7]*rhs.values[4]+self.values[8]*rhs.values[7],
                self.values[6]*rhs.values[2]+self.values[7]*rhs.values[5]+self.values[8]*rhs.values[8]
            ]
        }
    }
}


impl std::ops::Add for &Mat3 {
    
    type Output = Mat3;
    fn add(self, rhs: &Mat3) -> Mat3{
        Mat3 {
            values: [
                self.values[0] + rhs.values[0],
                self.values[1] + rhs.values[1],
                self.values[2] + rhs.values[2],
                self.values[3] + rhs.values[3],
                self.values[4] + rhs.values[4],
                self.values[5] + rhs.values[5],
                self.values[6] + rhs.values[6],
                self.values[7] + rhs.values[7],
                self.values[8] + rhs.values[8]
            ]
        }
    }
}


impl std::ops::Mul<f32> for &Mat3 { 
    type Output = Mat3;
    fn mul(self, rhs: f32) -> Mat3{
        Mat3 {
            values: [
                self.values[0]*rhs,
                self.values[1]*rhs,
                self.values[2]*rhs,
                self.values[3]*rhs,
                self.values[4]*rhs,
                self.values[5]*rhs,
                self.values[6]*rhs,
                self.values[7]*rhs,
                self.values[8]*rhs
            ]
        }
    }
}

impl std::ops::Div<f32> for &Mat3 { 
    type Output = Mat3;
    fn div(self, rhs: f32) -> Mat3{
        self*(1.0/rhs)
    }
}


impl std::ops::Mul<f32> for Mat3 { 
    type Output = Mat3;
    fn mul(self, rhs: f32) -> Mat3{
        Mat3 {
            values: [
                self.values[0]*rhs,
                self.values[1]*rhs,
                self.values[2]*rhs,
                self.values[3]*rhs,
                self.values[4]*rhs,
                self.values[5]*rhs,
                self.values[6]*rhs,
                self.values[7]*rhs,
                self.values[8]*rhs
            ]
        }
    }
}

impl std::ops::Div<f32> for Mat3 { 
    type Output = Mat3;
    fn div(self, rhs: f32) -> Mat3{
        self*(1.0/rhs)
    }
}

pub fn solve_quadratic(a : f32,b: f32,c: f32) -> Option<[f32;2]>{
    let mut det = b*b - 4.0*a*c;
    if det<0.0{
        return None; 
    }
    det = det.sqrt();
    let div = 1.0/(2.0*a);
    return Some([(-b-det)*div, (-b+det)*div]);

}

pub fn sample_hemisphere_cos(r1: f32, r2: f32) -> Vec3 {
    let theta = r1*2.0*PI;
    let sin_phi = r2.sqrt();
    let cos_phi = (1.0-sin_phi*sin_phi).sqrt();

    return Vec3::xyz(theta.cos()*sin_phi,theta.sin()*sin_phi,cos_phi);

}

//gets an orthogonal system -- function inpired PBRT
pub fn orthogonal(v1: Vec3) -> (Vec3,Vec3,Vec3) {
    let abs_x = v1.x.abs();
    let abs_y = v1.y.abs();
    let abs_z = v1.z.abs();

    let aux;
    if abs_x<abs_y {
        if abs_x < abs_z {
            aux = Vec3::xyz(1.0,0.0,0.0);
        } else {
            aux = Vec3::xyz(0.0,0.0,1.0);
        }
    } else {
        if(abs_y<abs_z) {
            aux = Vec3::xyz(0.0,1.0,0.0);
        } else {
            aux = Vec3::xyz(0.0,0.0,1.0);
        }
    }

    let aux2 = v1.cross(&aux);
    return (v1.normalized(), aux2.cross(&v1).normalized(), aux2.normalized());
}

pub fn linear2srgb(a:f32) -> f32 {
    if(a>0.0031308){
        (1.0+0.055)*a.powf(1.0/2.4)-0.055
    } else {
        12.92*a
    }
}