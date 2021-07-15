mod math;
mod scene;
mod material;
mod primitive;
mod light;
mod renderer;
mod camera;
mod constants;


use scene::Scene;
use camera::Camera2D;
use camera::CameraPerspective;
use primitive::SimpleGroup;
use renderer::DummyRenderer;
use renderer::PtRenderer;
use renderer::Renderer;
use math::Vec3;
use primitive::Sphere;
use primitive::Group;
use material::DiffuseMaterial;
use material::EmissionMaterial;
use material::Material;

fn main(){

    

    let floor_mat = &(DiffuseMaterial::new(Vec3::xyz(0.8,0.8,0.8))) as & dyn Material;
    let green_mat =&(DiffuseMaterial{albedo: Vec3::xyz(0.05,0.8,0.05)}) as & dyn Material;
    let red_mat = &(DiffuseMaterial{albedo: Vec3::xyz(0.8,0.05,0.05)}) as & dyn Material;
    let emit = &(EmissionMaterial{light: Vec3::xyz(1.0,1.0,1.0), intensity: 6.0}) as & dyn Material;
    
    let sg =&mut SimpleGroup::new();



    sg.add(Box::new(Sphere{
                        o: Vec3{x:0.3,y:0.0,z:0.1},
                        r: 0.2,
                        mat: floor_mat
                    })
            );

    sg.add(Box::new(Sphere{
                o: Vec3{x:-0.2,y:-0.5,z:-0.8},
                r: 0.2,                
                mat: floor_mat
            })
    );


    sg.add(Box::new(Sphere{
                o: Vec3{x:1001.0,y:0.0,z:0.0},
                r: 1000.,
                
                mat: green_mat
            })
    );

    sg.add(Box::new(Sphere{
            o: Vec3{x:-1001.,y:0.0,z:0.0},
            r: 1000.0,
            mat: red_mat
        })
    );
    sg.add(Box::new(Sphere{
            o: Vec3{x:0.0,y:0.0,z:1001.0},
            r: 1000.0,
                
            mat: floor_mat
        })
    );
    sg.add(Box::new(Sphere{
            o: Vec3{x:0.0,y:0.0,z:-1001.0},
            r: 1000.0,
                
            mat: floor_mat
        })
    );

    sg.add(Box::new(Sphere{
            o: Vec3{x:0.0,y:-1001.0,z:0.0},
            r: 1000.0,
                
            mat: floor_mat
        })
    );

    sg.add(Box::new(Sphere{
        o: Vec3{x:0.6,y:0.0,z:7.05},
        r: 6.06,
        mat: emit
    })
);
    

    let mut c = CameraPerspective::new(
        512,512,
        1.0,
        Vec3{x:0.0,y:1.0,z:0.0},
        Vec3{x:0.0,y:-1.0,z:0.0},
        Vec3{x:0.0,y:0.0,z:1.0},
    );


    let mut s = Scene{
        primitive: sg,
        camera: &mut c,
        lights: Vec::new()
    };

    let renderer = PtRenderer::new(50);
    renderer.render(&mut s, &String::from("lol3.png"));
    /*

    let mut s = Scene{
        primitive: Box::new(sg),
        camera: Box::new(Camera2D::new(256,256, &Vec3{x: 1.0,y: 1.0,z: 0.0}, &Vec3{x: 1.0,y: 1.,z: 0.})),
        lights: Vec::new()

    };

    let renderer = DummyRenderer{};
    renderer.render(&mut s, &String::from("lol.png"));

    println!("Hello world!");
    */
}