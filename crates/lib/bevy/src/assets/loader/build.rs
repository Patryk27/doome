use doome_raytracer::{ATLAS_HEIGHT, ATLAS_WIDTH};
use maplit::btreemap;
use rectangle_pack::{
    contains_smallest_box, volume_heuristic, GroupedRectsToPlace, RectToInsert,
    TargetBin,
};

use super::{
    AssetStorageBuilder, DoomeAssetHandle, DoomeAssets, DoomeAssetsLoader,
    Texture,
};

const DEPTH: u32 = 1;

impl DoomeAssetsLoader {
    pub fn build(self) -> DoomeAssets {
        let mut rects: GroupedRectsToPlace<_, ()> = GroupedRectsToPlace::new();

        for (texture_handle, texture) in self.textures.iter() {
            rects.push_rect(
                texture_handle.id(),
                None,
                RectToInsert::new(texture.width(), texture.height(), DEPTH),
            );
        }

        let mut target_bins = btreemap! {
            0u32 => TargetBin::new(ATLAS_WIDTH, ATLAS_HEIGHT, DEPTH),
        };

        let rects = rectangle_pack::pack_rects(
            &rects,
            &mut target_bins,
            &volume_heuristic,
            &contains_smallest_box,
        )
        .unwrap();

        let mut atlas = image::RgbaImage::new(ATLAS_WIDTH, ATLAS_HEIGHT);
        let mut textures = AssetStorageBuilder::default();

        for (texture_id, (_, location)) in rects.packed_locations().iter() {
            let texture_id = DoomeAssetHandle::new(*texture_id);
            let (texture, texture_name) = self.textures.by_handle(texture_id);
            let (atlas_offset_x, atlas_offset_y) = (location.x(), location.y());

            // TODO: Optimization - write by row instead of by pixel
            for (dx, dy, pixel) in texture.enumerate_pixels() {
                atlas.put_pixel(
                    atlas_offset_x + dx,
                    atlas_offset_y + dy,
                    *pixel,
                );
            }

            textures.push(
                texture_name,
                Texture {
                    width: texture.width(),
                    height: texture.height(),
                    atlas_offset_x,
                    atlas_offset_y,
                },
            );
        }

        DoomeAssets {
            models: self.models.build(),
            images: self.images.build(),
            textures: textures.build(),
            atlas,
        }
    }
}
