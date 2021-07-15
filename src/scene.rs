use crate::primitive::Primitive;
use crate::light::Light;
use crate::camera::Camera;
use crate::primitive::Group;

pub struct Scene {
    pub primitive: Box<dyn Primitive>,
    pub lights: Vec<Box<dyn Light>>,
    pub camera: Box<dyn Camera>
}