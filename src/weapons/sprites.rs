use doome_bevy::prelude::DoomeAssetHandle;
use image::RgbaImage;

#[derive(Debug, Clone)]
pub struct WeaponSprites {
    pub ui_icon: DoomeAssetHandle<RgbaImage>,
    pub idle: DoomeAssetHandle<RgbaImage>,
    pub animation: Vec<DoomeAssetHandle<RgbaImage>>,
}
