use nalgebra::{Matrix4, Vector3};

use crate::raytracer::Ray;

#[derive(Default)]
pub struct Scene {
    camera: Camera,
    objects: Vec<Box<dyn Object>>,
    lights: Vec<Light>,
}

impl Scene {
    pub fn set_camera(&mut self, camera: Camera) {
        self.camera = camera;
    }

    pub fn camera(&self) -> &Camera {
        &self.camera
    }

    pub fn add_object(&mut self, object: impl Object + 'static) {
        self.objects.push(Box::new(object))
    }

    pub fn objects(&self) -> impl Iterator<Item = &dyn Object> {
        self.objects.iter().map(|o| o.as_ref())
    }

    pub fn lights(&self) -> impl Iterator<Item = &Light> {
        self.lights.iter()
    }

    pub fn lights_mut(&mut self) -> &mut Vec<Light> {
        &mut self.lights
    }

    pub fn add_light(&mut self, light: Light) {
        self.lights.push(light);
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Camera {
    position: Vector3<f32>,
    projection: Matrix4<f32>,
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Light {
    position: Vector3<f32>,
}

pub trait Object {
    /// Checks whether this object and `ray` are intersecting.
    ///
    /// The first field in the tuple is whether or not the object and ray intersects.
    /// If the objects doesn't intersect the second field contains the distance between them.
    /// In the case of intersection the second value in the returned tuple is 0.0.
    ///
    /// # Arguments
    ///
    /// `ray` - The ray to check intersection with.
    ///
    /// # Examples
    ///
    /// ```rs
    /// let sphere = Sphere::new(vector![], 50.0);
    /// let ray = Ray::default();
    /// println!("Intersects: {}", sphere.intersects(&ray).0);
    /// ```
    fn intersects(&self, ray: &Ray) -> bool;
}

struct Sphere {
    center: Vector3<f32>,
    radius: f32,
}

impl Object for Sphere {
    fn intersects(&self, ray: &Ray) -> bool {
        // Koordinaterne for cirklens centrum indsættes i ligningen for kuglen.
        // https://www.webmatematik.dk/lektioner/matematik-a/vektorer-i-3d/skaering-mellem-linje-og-kugle
        let mut co = ray.origin - self.center;
        co[2] -= self.radius.powi(2);

        true
    }
}
