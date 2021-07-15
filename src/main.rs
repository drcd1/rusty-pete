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

fn main(){

    let mut sg = SimpleGroup::new();
    sg.add(Box::new(Sphere{
                        o: Vec3{x:0.3,y:0.0,z:0.1},
                        r: 0.2,
                    })
            );

    sg.add(Box::new(Sphere{
                o: Vec3{x:-0.2,y:0.1,z:-0.2},
                r: 0.2,
            })
    );


    sg.add(Box::new(Sphere{
                o: Vec3{x:1001.0,y:0.0,z:0.0},
                r: 1000.,
            })
    );

    sg.add(Box::new(Sphere{
            o: Vec3{x:-1001.,y:0.0,z:0.0},
            r: 1000.0,
        })
    );
    sg.add(Box::new(Sphere{
            o: Vec3{x:0.0,y:0.0,z:1001.0},
            r: 1000.0,
        })
    );
    sg.add(Box::new(Sphere{
            o: Vec3{x:0.0,y:0.0,z:-1001.0},
            r: 1000.0,
        })
    );

    sg.add(Box::new(Sphere{
            o: Vec3{x:0.0,y:-1001.0,z:0.0},
            r: 1000.0,
        })
    );

    sg.add(Box::new(Sphere{
        o: Vec3{x:0.0,y:0.0,z:2.0},
        r: 1.1,
    })
);
    

    let c = CameraPerspective::new(
        256,256,
        1.0,
        Vec3{x:0.0,y:1.0,z:0.0},
        Vec3{x:0.0,y:-1.0,z:0.0},
        Vec3{x:0.0,y:0.0,z:1.0},
    );

    
    let mut s = Scene{
        primitive: Box::new(sg),
        camera: Box::new(c),
        lights: Vec::new()
    };

    let renderer = PtRenderer::new(1);
    renderer.render(&mut s, &String::from("lol2.png"));
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