use glam::{vec2, Vec2};
use maplit::btreemap;
use raytracer::{ATLAS_HEIGHT, ATLAS_WIDTH};
use rectangle_pack::{
    contains_smallest_box, volume_heuristic, GroupedRectsToPlace, RectToInsert,
    TargetBin,
};

use super::PipelineBuilder;
use crate::pipeline::Pipeline;

const DEPTH: u32 = 1;

// const SAVE_ATLAS_FOR_PREVIEW: Option<&'static str> = Some("atlas.png");
const SAVE_ATLAS_FOR_PREVIEW: Option<&'static str> = None;

impl PipelineBuilder {
    pub fn build(mut self) -> Pipeline {
        let mut rects_to_place: GroupedRectsToPlace<String, ()> =
            GroupedRectsToPlace::new();

        for (id, (texture, _affected_models)) in &self.textures {
            rects_to_place.push_rect(
                id.clone(),
                None,
                RectToInsert::new(texture.width(), texture.height(), DEPTH),
            );
        }

        let mut target_bins = btreemap! {
            0u32 => TargetBin::new(ATLAS_WIDTH, ATLAS_HEIGHT, DEPTH),
        };

        let result = rectangle_pack::pack_rects(
            &rects_to_place,
            &mut target_bins,
            &volume_heuristic,
            &contains_smallest_box,
        )
        .expect("Failed to pack textures");

        let mut atlas = image::RgbaImage::new(ATLAS_WIDTH, ATLAS_HEIGHT);

        for (tex_id, (_bin_id, location)) in result.packed_locations().iter() {
            let (texture, affected_models) = self
                .textures
                .get(tex_id)
                .as_ref()
                .expect("texture should exist");

            let (x, y) = (location.x(), location.y());

            // TODO: Optimization - write by row instead of by pixel
            for (x_offset, y_offset, pixel) in texture.enumerate_pixels() {
                atlas.put_pixel(x + x_offset, y + y_offset, *pixel);
            }

            let old_tex_size =
                vec2(texture.width() as f32, texture.height() as f32);

            let new_tex_size = vec2(ATLAS_WIDTH as f32, ATLAS_HEIGHT as f32);
            let offset_in_new_tex = vec2(x as f32, y as f32);

            for model_handle in affected_models {
                let model = &mut self.models[model_handle.0];

                for (_, tri_map) in &mut model.triangles {
                    tri_map.uv0 = remap_uv(
                        tri_map.uv0,
                        old_tex_size,
                        new_tex_size,
                        offset_in_new_tex,
                    );

                    tri_map.uv1 = remap_uv(
                        tri_map.uv1,
                        old_tex_size,
                        new_tex_size,
                        offset_in_new_tex,
                    );

                    tri_map.uv2 = remap_uv(
                        tri_map.uv2,
                        old_tex_size,
                        new_tex_size,
                        offset_in_new_tex,
                    );
                }
            }
        }

        if let Some(filename) = SAVE_ATLAS_FOR_PREVIEW {
            atlas.save(filename).expect("Failed to save image");
        }

        Pipeline {
            models: self.models,
            atlas,
        }
    }
}

fn remap_uv(
    mut old_uv: Vec2,
    old_tex_size: Vec2,
    new_tex_size: Vec2,
    offset_in_new_tex: Vec2,
) -> Vec2 {
    old_uv.y = 1.0 - old_uv.y;

    let pixel_coords = old_uv * old_tex_size;
    let new_uv = (offset_in_new_tex + pixel_coords) / new_tex_size;

    new_uv
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::*;

    #[test_case(vec2(0.0, 0.0), vec2(1.0, 1.0), vec2(2.0, 2.0), vec2(0.0, 0.0) => vec2(0.0, 0.5) ; "zero without offset")]
    #[test_case(vec2(0.0, 0.0), vec2(1.0, 1.0), vec2(2.0, 2.0), vec2(1.0, 1.0) => vec2(0.5, 1.0) ; "zero with offset")]
    #[test_case(vec2(1.0, 1.0), vec2(2.0, 2.0), vec2(4.0, 4.0), vec2(0.0, 0.0) => vec2(0.5, 0.0) ; "one without offset")]
    #[test_case(vec2(0.5, 0.5), vec2(2.0, 2.0), vec2(4.0, 4.0), vec2(1.0, 1.0) => vec2(0.5, 0.5) ; "half with offset")]
    fn test_remap_uv(
        old_uv: Vec2,
        old_tex_size: Vec2,
        new_tex_size: Vec2,
        offset_in_new_tex: Vec2,
    ) -> Vec2 {
        remap_uv(old_uv, old_tex_size, new_tex_size, offset_in_new_tex)
    }
}
