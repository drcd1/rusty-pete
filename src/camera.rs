use crate::math::Vec3;
use crate::primitive::Ray;
use crate::math::float2u8;
use crate::math::Mat3;

use image::{RgbImage, Rgb};

pub trait Camera{    
    fn get_ray(&self, u: Vec3) -> Ray;

    //todo: make private
    fn image(&mut self) -> &mut RgbImage;

    
    //make not mut?
    
    fn save_image(&mut self, filename: &String){
        
        match self.image().save_with_format(filename, image::ImageFormat::Png){
            Ok(_) => {}
            Err(_) => {panic!("Couldn't save!")}
        }
    }
    fn set_pixel(&mut self, i:u32,j:u32,rgb: [f32;3]){
        let a = Rgb{0:[float2u8(rgb[0]),float2u8(rgb[1]),float2u8(rgb[2])]};
        self.image().put_pixel(i,j,a);
    }
}


pub struct Camera2D{
    image: RgbImage
}


impl Camera2D {
    pub fn new(resX: u32, resY: u32, scale: &Vec3, origin: &Vec3) -> Camera2D{
        Camera2D{
            image: RgbImage::new(resX, resY)
        }
    }
}

impl Camera for Camera2D {

    fn get_ray(&self, u: Vec3) -> Ray{
        Ray::new(Vec3::new(), u)
    }
    

    //todo: make private
    fn image(&mut self) -> &mut RgbImage{
        return &mut self.image;
    }
    

}


pub struct CameraPerspective {
    aspect_ratio: f32,
    tan_fovy: f32,
    coords: Mat3,
    origin: Vec3,
    image: RgbImage
}

impl CameraPerspective {
    

    pub fn new(res_x: u32, res_y: u32, 
        tan_fovy: f32, origin: Vec3, 
        forward: Vec3, up: Vec3) 
                -> CameraPerspective{
        let z = (forward*(-1.0)).normalized();
        let x = (up.cross(&z)).normalized();
        let y = (z.cross(&x)).normalized();
        CameraPerspective{
                            aspect_ratio: res_x as f32 / res_y as f32,
                            tan_fovy: tan_fovy,
                            coords: Mat3::from_axis(&x,&y,&z),
                            origin: origin,
                            image: RgbImage::new(res_x, res_y)
                            }
    }
}

impl Camera for CameraPerspective {

    fn get_ray(&self, u: Vec3) -> Ray{
        let mut dir = Vec3{ x:u.x*self.aspect_ratio*self.tan_fovy, 
                            y:u.y*self.tan_fovy, 
                            z:-1.0

                            };
        dir = &self.coords*&dir;
        dir = dir.normalized();


        Ray::new(self.origin, dir)
    }


    //todo: make private
    fn image(&mut self) -> &mut RgbImage{
        return &mut self.image;
    }
    

}