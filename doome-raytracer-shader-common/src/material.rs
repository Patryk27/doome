use crate::*;

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct Material {
    // color: Vec4,
    reflectivity: Vec4,
}

impl Material {
    // #[cfg(not(target_arch = "spirv"))]
    // pub fn with_color(mut self, color: u32) -> Self {
    //     self.color = rgb_to_srgb(color).extend(1.0);
    //     self
    // }

    #[cfg(not(target_arch = "spirv"))]
    pub fn with_reflectivity(
        mut self,
        reflectivity: f32,
        reflection_color: u32,
    ) -> Self {
        self.reflectivity = rgb_to_srgb(reflection_color).extend(reflectivity);
        self
    }

    pub fn shade(
        &self,
        geometry: &Geometry,
        lights: &Lights,
        materials: &Materials,
        texture: &Texture,
        hit: Hit,
    ) -> Vec3 {
        let mut color = self.radiance(geometry, lights, texture, hit);
        let reflectivity = self.reflectivity.w;

        if reflectivity > 0.0 {
            let reflection_color = self.reflectivity.xyz();

            let reflection_dir = {
                let camera_dir = -hit.ray.direction();

                hit.normal * hit.normal.dot(camera_dir) * 2.0 - camera_dir
            };

            let ray_color = Ray::new(hit.point, reflection_dir)
                .shade_basic(geometry, lights, materials, texture);

            color += ray_color * reflection_color * reflectivity;
        }

        color
    }

    /// See: [`Ray::shade_basic()`].
    pub fn shade_basic(
        &self,
        geometry: &Geometry,
        lights: &Lights,
        texture: &Texture,
        hit: Hit,
    ) -> Vec3 {
        self.radiance(geometry, lights, texture, hit)
    }

    fn radiance(
        &self,
        geometry: &Geometry,
        lights: &Lights,
        texture: &Texture,
        hit: Hit,
    ) -> Vec3 {
        let color = texture.sample(hit.uv).truncate();
        let mut radiance = vec3(0.0, 0.0, 0.0);

        let mut light_idx = 0;

        while light_idx < lights.len() {
            let light = lights.get(light_idx);
            let ray = Ray::new(hit.point, light.pos() - hit.point);
            let distance = light.pos().distance(hit.point);

            if !ray.hits_anything_up_to(geometry, distance) {
                let direction = (light.pos() - hit.point).normalize();
                let diffuse_factor = direction.dot(hit.normal);

                if diffuse_factor > 0.0 {
                    radiance += light.color()
                        * light.intensity()
                        * color
                        * diffuse_factor;
                }
            }

            light_idx += 1;
        }

        radiance
    }
}

#[cfg(not(target_arch = "spirv"))]
impl Default for Material {
    fn default() -> Self {
        Material::zeroed()
    }
}

#[derive(Copy, Clone)]
pub struct MaterialId(u32);

impl MaterialId {
    pub(crate) fn new(id: u32) -> Self {
        Self(id)
    }

    pub(crate) fn get(self) -> u32 {
        self.0
    }
}
