use doome_bevy::prelude::AssetHandle;
use image::RgbaImage;

#[derive(Debug, Clone)]
pub struct WeaponSprites {
    pub ui_icon: AssetHandle<RgbaImage>,
    pub idle: AssetHandle<RgbaImage>,
    pub animation: Vec<AssetHandle<RgbaImage>>,
}
