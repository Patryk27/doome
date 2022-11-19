use bevy::prelude::*;
use doome_bevy::assets::{AssetHandle, Assets};
use doome_bevy::components::Player;
use doome_bevy::doome::DoomeRenderer;
use doome_bevy::physics::RayCast;
use doome_bevy::text::Text;
use doome_engine::{Canvas, HEIGHT};
use image::RgbaImage;

use crate::interaction::TextInteraction;

pub struct UiAnd2dPlugin;

impl Plugin for UiAnd2dPlugin {
    fn build(&self, app: &mut App) {
        let assets = app.world.resource::<Assets>();
        let gun_image = assets.load_image("gun_1");
        app.insert_resource(Data { gun_image })
            .add_system(render_ui_and_2d);
    }
}

#[derive(Resource)]
struct Data {
    gun_image: AssetHandle<RgbaImage>,
}

fn render_ui_and_2d(
    assets: Res<Assets>,
    data: Res<Data>,
    mut doome_renderer: ResMut<DoomeRenderer>,
    text: Res<Text>,
    raycaster: Query<(&Player, &RayCast)>,
    interactables: Query<&TextInteraction>,
) {
    let frame = &mut doome_renderer.pixels.image_data;
    let mut canvas = Canvas::new(&text.text_engine, frame);

    canvas.clear();

    canvas.blit(0, 0, assets.image(data.gun_image));

    let (_player, raycast) = raycaster.single();

    if let Some(hit) = raycast.hit.as_ref() {
        if let Ok(interactable) = interactables.get(hit.entity) {
            canvas.text(10, HEIGHT - 30, &interactable.content);
        }
    }
}
