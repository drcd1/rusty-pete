use crate::primitive::Primitive;
use crate::light::Light;
use crate::camera::Camera;
use crate::primitive::Group;

pub struct Scene<'a> {
    pub primitive: &'a dyn Primitive,
    pub lights: Vec<&'a dyn Light>,
    pub camera: &'a mut dyn Camera
}