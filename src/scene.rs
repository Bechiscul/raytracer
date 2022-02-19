use nalgebra::{vector, Matrix4, Vector3};

use crate::raytracer::Ray;

#[derive(Default)]
pub struct Scene {
    camera: Camera,
    objects: Vec<Box<dyn Object>>,
    lights: Vec<Light>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            camera: Camera {
                origin: vector![0.0, 0.0, 0.0],
                fov: (60.0f32).to_radians(),
            },
            ..Default::default()
        }
    }

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
    pub origin: Vector3<f32>,
    pub fov: f32,
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
    fn intersect(&self, ray: &Ray) -> Option<Hit>;
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Hit {
    pub t: f32,
    pub normal: Vector3<f32>,
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Sphere {
    pub center: Vector3<f32>,
    pub radius: f32,
}

impl Object for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        // Koordinaterne for cirklens centrum indsættes i ligningen for kuglen.
        // https://www.webmatematik.dk/lektioner/matematik-a/vektorer-i-3d/skaering-mellem-linje-og-kugle
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * oc.dot(&ray.direction);
        let c = oc.dot(&oc) - self.radius.powi(2);
        let discriminant = b.powi(2) - 4.0 * a * c;

        if discriminant <= 0.0 {
            return None;
        }

        let t = (-b - discriminant.sqrt()) / (2.0 * a);
        let normal = (ray.at(t) - self.center).normalize();
        Some(Hit { t, normal })
    }
}
