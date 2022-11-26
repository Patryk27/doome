use crate::prelude::*;

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
        app.add_system(handle_inventory);
    }
}

#[derive(Debug, Default, Component)]
pub struct Inventory {
    pub has_flashlight: bool,
    pub keys: Vec<Key>,
}

impl Inventory {
    pub fn has_key(&self, key: &Key) -> bool {
        self.keys.iter().any(|key2| key2.name() == key.name())
    }

    pub fn remove_key(&mut self, key: &Key) {
        self.keys.drain_filter(|key2| key2.name() == key.name());
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Inventory::default());
}

fn handle_inventory(
    mut commands: Commands,
    inventory: Query<&Inventory, Changed<Inventory>>,
    flashlight: Query<Entity, With<Flashlight>>,
) {
    let Ok(inventory) = inventory.get_single() else { return };

    if let Ok(flashlight) = flashlight.get_single() {
        if !inventory.has_flashlight {
            commands.entity(flashlight).despawn();
        }
    } else {
        if inventory.has_flashlight {
            Flashlight::spawn(&mut commands);
        }
    }
}
