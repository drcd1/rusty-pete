use image::RgbImage;
use rand::Rng;
use crate::scene::Scene;
use crate::math::Vec3;
use crate::primitive::Ray;
use crate::primitive::Primitive;
use crate::math::linear2srgb;
use rand::thread_rng;

pub trait Renderer{
    fn render(&self, scene: &mut Scene, fname: &String);
}

pub struct DummyRenderer {
}

impl Renderer for DummyRenderer{
    fn render(&self, scene: &mut Scene, fname: &String){
        let image : &RgbImage= scene.camera.image();
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

    fn russian_roulette(col: &Vec3)-> f32{
        //LOL this is not LAB ahahahah
        return (col.x*0.2 + col.y*0.5 +col.z*0.3)*0.5 + 0.4;
    }

    fn get_random(&self) -> f32 {
        return thread_rng().gen::<f32>();
    }

    fn integrate(&self, scene: &Scene, r: &mut Ray) -> Vec3{
        //println!("Ray: {} {} {}",r.d.x/r.d.y,-r.d.y/r.d.y,r.d.z/r.d.y);
        
        let mut color = Vec3::new();
        let mut mul = Vec3::from(1.0);
        
        for steps in 0..32 {
            let it = scene.primitive.intersect(r);
            match it {
                None => {color =  color + &mul*&self.sky(r); break;}
                Some(x) => {
                    //return Vec3::from(((r.d.normalized()).dot(&x.n.normalized())).abs());
                    color =  color + &mul* &x.mat.emit(&x);
                    let r1 = self.get_random();
                    let r2 = self.get_random();
                    let (sample,mut p) = x.mat.sample(&x,r1,r2);

                    //negative probability encodes zero reflectance
                    if p<=0.0 {
                        break;
                    }
                    
                    let eval = x.mat.eval(&x,&sample);
                    if steps > 2{
                        let rr = PtRenderer::russian_roulette(&eval);
                        if self.get_random() > rr {
                            break;
                        }

                        p = p*rr;
                    }
                    mul = &(&mul*&eval)/p;
                    
                    *r = Ray::new(x.p, sample);
                }
            }

        }
        return color;
        
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

                
                scene.camera.set_pixel(i,j,[linear2srgb(col.x),linear2srgb(col.y),linear2srgb(col.z)]);
            }
        }
        scene.camera.save_image(fname);
    }

}