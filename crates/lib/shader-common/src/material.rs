use crate::*;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Pod, Zeroable)]
#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
pub struct Material {
    // x,y,z is color, w is 1.0 indicates texture is present, 0.0 indicates texture is not present
    color: Vec4,
    // x,y,z is reflectivity color, w is reflectivity
    reflectivity: Vec4,
}

impl Material {
    pub fn shade(&self, world: &World, hit: Hit) -> Vec3 {
        let mut color = self.radiance(world, hit);

        // ------------ //
        // Reflectivity //
        let reflectivity = self.reflectivity.w;

        if reflectivity > 0.0 {
            let reflection_color = self.reflectivity.xyz();

            let reflection_dir = {
                let camera_dir = -hit.ray.direction();

                hit.normal * hit.normal.dot(camera_dir) * 2.0 - camera_dir
            };

            let ray_color =
                Ray::new(hit.point, reflection_dir).shade_basic(world);

            color += ray_color * reflection_color * reflectivity;
        }

        // ------------ //
        // Transparency //
        let triangle = world.geometry(hit.triangle_id);

        if triangle.alpha() < 1.0 {
            let ray_color = Ray::new(
                hit.point + 0.1 * hit.ray.direction(),
                hit.ray.direction(),
            )
            .shade_basic(world);

            color =
                color * triangle.alpha() + ray_color * (1.0 - triangle.alpha());
        }

        color
    }

    /// See: [`Ray::shade_basic()`].
    pub fn shade_basic(&self, world: &World, hit: Hit) -> Vec3 {
        self.radiance(world, hit)
    }

    fn radiance(&self, world: &World, hit: Hit) -> Vec3 {
        let color = if self.has_texture() {
            (world.atlas_sample(hit.triangle_id, hit.uv) * self.color)
                .truncate()
        } else {
            self.color.truncate()
        };

        let mut radiance = vec3(0.0, 0.0, 0.0);
        let mut light_idx = 0;

        while light_idx < world.lights.len() {
            let light = world.lights.get(light_idx);
            let ray = Ray::new(hit.point, light.pos() - hit.point);
            let distance = light.pos().distance(hit.point);

            if light.is_spot() {
                let dir_light_to_hit = hit.point - light.pos();
                let dir_light_to_point = light.point_at() - light.pos();

                let angle = dir_light_to_point.angle_between(dir_light_to_hit);

                let cone_factor =
                    map_quadratic_clamped(angle, light.cone_angle());

                let diffuse_factor = if ray.hits_anything_up_to(world, distance)
                {
                    0.0
                } else {
                    ray.direction().dot(hit.normal).max(0.0)
                };

                radiance += cone_factor
                    * diffuse_factor
                    * light.color()
                    * light.intensity()
                    * color;
            } else {
                let diffuse_factor = if ray.hits_anything_up_to(world, distance)
                {
                    0.0
                } else {
                    ray.direction().dot(hit.normal).max(0.0)
                };

                radiance +=
                    diffuse_factor * light.color() * light.intensity() * color;
            }

            light_idx += 1;
        }

        radiance
    }

    pub fn has_texture(&self) -> bool {
        self.color.w == 1.0
    }
}

/// Remaps the given value with a quadratic formula.
/// Such that, the result is between 0.0 and 1.0
/// 1.0 is at v == 0.0
/// 0.0 is at abs(v) == span
fn map_quadratic_clamped(v: f32, span: f32) -> f32 {
    f32::clamp(1.0 - (v / span).powf(2.0), 0.0, 1.0)
}

#[cfg(not(target_arch = "spirv"))]
impl Material {
    pub fn with_color(mut self, color: Vec3) -> Self {
        self.color = color.extend(0.0);
        self
    }

    pub fn with_texture(mut self, val: bool) -> Self {
        self.color.w = if val { 1.0 } else { 0.0 };
        self
    }

    pub fn with_reflectivity(
        mut self,
        reflectivity: f32,
        reflection_color: Vec3,
    ) -> Self {
        self.reflectivity = reflection_color.extend(reflectivity);
        self
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
pub struct MaterialId(usize);

impl MaterialId {
    pub fn new(id: usize) -> Self {
        Self(id)
    }

    pub fn get(self) -> usize {
        self.0
    }
}
