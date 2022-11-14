use crate::*;

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct Material {
    // x,y,z is color, w is 1.0 indicates texture is present, 0.0 indicates texture is not present
    color: Vec4,
    // x,y,z is reflectivity color, w is reflectivity
    reflectivity: Vec4,
}

#[cfg(not(target_arch = "spirv"))]
impl Material {
    pub fn with_color(mut self, color: u32) -> Self {
        self.color = rgb_to_srgb(color).extend(self.color.w);
        self
    }

    pub fn with_color_rgb(mut self, r: u8, g: u8, b: u8) -> Self {
        self.color =
            rgb_to_srgb(u32::from_be_bytes([0, r, g, b])).extend(self.color.w);
        self
    }

    pub fn with_color_rgb_norm(mut self, r: f32, g: f32, b: f32) -> Self {
        let (r, g, b) = (r * 255.0, g * 255.0, b * 255.0);
        self.color =
            rgb_to_srgb(u32::from_be_bytes([0, r as u8, g as u8, b as u8]))
                .extend(self.color.w);
        self
    }

    pub fn with_texture(mut self, with_texture: bool) -> Self {
        self.color.w = if with_texture { 1.0 } else { 0.0 };
        self
    }

    pub fn with_reflectivity(
        mut self,
        reflectivity: f32,
        reflection_color: u32,
    ) -> Self {
        self.reflectivity = rgb_to_srgb(reflection_color).extend(reflectivity);
        self
    }
}

impl Material {
    pub fn shade(
        &self,
        geometry: &Geometry,
        geometry_index: &GeometryIndex,
        lights: &Lights,
        materials: &Materials,
        texture: &Texture,
        hit: Hit,
    ) -> Vec3 {
        let mut color =
            self.radiance(geometry, geometry_index, lights, texture, hit);

        let reflectivity = self.reflectivity.w;

        if reflectivity > 0.0 {
            let reflection_color = self.reflectivity.xyz();

            let reflection_dir = {
                let camera_dir = -hit.ray.direction();

                hit.normal * hit.normal.dot(camera_dir) * 2.0 - camera_dir
            };

            let ray_color = Ray::new(hit.point, reflection_dir).shade_basic(
                geometry,
                geometry_index,
                lights,
                materials,
                texture,
            );

            color += ray_color * reflection_color * reflectivity;
        }

        color
    }

    /// See: [`Ray::shade_basic()`].
    pub fn shade_basic(
        &self,
        geometry: &Geometry,
        geometry_index: &GeometryIndex,
        lights: &Lights,
        texture: &Texture,
        hit: Hit,
    ) -> Vec3 {
        self.radiance(geometry, geometry_index, lights, texture, hit)
    }

    fn radiance(
        &self,
        geometry: &Geometry,
        geometry_index: &GeometryIndex,
        lights: &Lights,
        texture: &Texture,
        hit: Hit,
    ) -> Vec3 {
        let color = if self.color.w == 1.0 {
            (texture.sample(hit.uv) * self.color).truncate()
        } else {
            self.color.truncate()
        };

        let mut radiance = vec3(0.0, 0.0, 0.0);
        let mut light_idx = 0;

        while light_idx < lights.len() {
            let light = lights.get(light_idx);
            let ray = Ray::new(hit.point, light.pos() - hit.point);
            let distance = light.pos().distance(hit.point);

            if !ray.hits_anything_up_to(geometry, geometry_index, distance) {
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
        Material {
            color: vec4(1.0, 1.0, 1.0, 0.0),
            reflectivity: vec4(0.0, 0.0, 0.0, 0.0),
        }
    }
}

#[derive(Copy, Clone)]
#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
pub struct MaterialId(u32);

impl MaterialId {
    pub(crate) fn new(id: u32) -> Self {
        Self(id)
    }

    pub(crate) fn get(self) -> u32 {
        self.0
    }
}
