use nalgebra::Matrix4;

#[derive(Default)]
pub struct Scene {
    camera: Camera,
    objects: Vec<Box<dyn Shape>>,
    lights: Vec<Light>,
}

impl Scene {
    pub fn set_camera(&mut self, camera: Camera) {
        self.camera = camera;
    }

    pub fn camera(&self) -> &Camera {
        &self.camera
    }

    pub fn add_object(&mut self, object: impl Object) {
        self.objects.push(Box::new(object))
    }

    pub fn objects(&self) -> impl Iterator<&dyn Object> {
        self.objects.iter()
    }

    pub fn lights(&self) -> &Vec<Light> {
        &self.lights
    }

    pub fn lights_mut(&mut self) -> &mut Vec<Light> {
        &mut self.lights
    }

    pub fn add_light(&mut self, light: Light) {
        self.lights.push(light);
    }
}

pub struct Camera {
    projection: Matrix<f32>,
}

pub struct Light {
    position: Vector3<f32>,
}

pub trait Object {
    fn intersects(&self, ray: &Ray) -> bool;
}
