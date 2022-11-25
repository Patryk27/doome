use bevy::prelude::*;

use crate::prelude::Flashlight;

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
        app.add_system(handle_inventory);
    }
}
#[derive(Debug, Component)]
pub struct Inventory {
    pub has_flashlight: bool,
}

fn setup(mut commands: Commands) {
    commands.spawn((Inventory {
        has_flashlight: false,
    },));
}

fn handle_inventory(
    mut commands: Commands,
    mut inventory: Query<&mut Inventory, Changed<Inventory>>,
    mut flashlight: Query<(Entity, &Flashlight)>,
) {
    let Ok(mut inventory) = inventory.get_single_mut() else { return };
    let flashlight = flashlight.get_single();

    if inventory.has_flashlight && flashlight.is_err() {
        Flashlight::spawn(&mut commands);
    } else {
        let Ok((entity, _)) = flashlight else { return };
        commands.entity(entity).despawn();
    }
}
