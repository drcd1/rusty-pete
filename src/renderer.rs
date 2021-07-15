use image::RgbImage;
use rand::Rng;
use crate::scene::Scene;
use crate::math::Vec3;
use crate::primitive::Ray;
use crate::primitive::Primitive;

pub trait Renderer{
    fn render(&self, scene: &mut Scene, fname: &String);
}

pub struct DummyRenderer {
}

impl Renderer for DummyRenderer{
    fn render(&self, scene: &mut Scene, fname: &String){
        let image = scene.camera.image();
        let (x,y) = image.dimensions();
        for i in 0..x{
            for j in 0..y{
                let mut col = Vec3::new();
                col.x = i as f32/x as f32;
                col.y = j as f32/y as f32;
                col.z = 0.0;

                scene.camera.set_pixel(i,j,[col.x,col.y,col.z]);
            }
        }
        scene.camera.save_image(fname);
    }
}

pub struct PtRenderer {
    samples: u32
}

impl PtRenderer {
    pub fn new(s: u32) -> PtRenderer{
        if s<0 {
            panic!("Samples must be positive!");
        }
        PtRenderer{
            samples: s
        }
    }
    fn sky(&self, r: &Ray) -> Vec3{
        let a = r.d.normalized();
        let v = Vec3{x:1.0,y:0.0,z:1.0}*(a.z*0.5 + 0.5);
        return v;
    }

    fn integrate(&self, scene: &Scene, r: &mut Ray) -> Vec3{
        //println!("Ray: {} {} {}",r.d.x/r.d.y,-r.d.y/r.d.y,r.d.z/r.d.y);
        let it = scene.primitive.intersect(r);
        match it {
            None => {return self.sky(r);}
            Some(x) => {
                //return Vec3::from(((r.d.normalized()).dot(&x.n.normalized())).abs());
                //println!("Yikes: {}", x.n.normalized().dot(&r.d));
                Vec3::from(x.n.normalized().dot(&r.d).abs())
            }
        }
    }
}


impl Renderer for PtRenderer{
    fn render(&self, scene: &mut Scene, fname: &String){
        let image = scene.camera.image();
        let (x,y) = image.dimensions();
        let mut rng = rand::thread_rng();
        for i in 0..x{
            println!("Rendering line {}", i);
            for j in 0..y{

                let mut col = Vec3::new();
                for k in 0..self.samples {
                    for l in 0..self.samples{
                        //let r1 = rng.gen::<f32>();
                        //let r2 = rng.gen::<f32>();

                        
                        let r1 = 0.5;
                        let r2 = 0.5;
                        let mut u = (i as f32 + (k as f32 +r1)
                                              /self.samples as f32       )    
                                 /x as f32;
                        
                        let mut v = (j as f32 + (l as f32 +r2)
                                 /self.samples as f32       )    
                            /y as f32;

                        u = u*2.0-1.0;
                        v = -(v*2.0-1.0);
                        let sample = self.integrate(scene, &mut scene.camera.get_ray(Vec3{x:u,y:v,z:0.0}));
                        col = col + sample;
                    }
                }

                col = &col / (self.samples*self.samples) as f32;
                scene.camera.set_pixel(i,j,[col.x,col.y,col.z]);
            }
        }
        scene.camera.save_image(fname);
    }

}