use bevy::prelude::*;
use doome_bevy::physics::Collider;
use doome_nav::{NavData, NavDataBuilder};

pub struct EnemiesPlugin;

#[derive(Component)]
pub struct Enemy;

#[derive(Clone, Copy, Debug)]
pub struct RecalculateNavData;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
        app.add_event::<RecalculateNavData>();
        app.add_system(recalculate_nav_data);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Hivemind { nav_data: None });
}

#[derive(Component)]
struct Hivemind {
    nav_data: Option<NavData>,
}

fn recalculate_nav_data(
    mut event_reader: EventReader<RecalculateNavData>,
    mut hivemind: Query<&mut Hivemind>,
    walls: Query<(&Transform, &Collider)>,
) {
    if event_reader.iter().count() == 0 {
        return;
    }

    let mut hivemind = hivemind.single_mut();

    let build_time_start = std::time::Instant::now();
    let mut nav_data_builder = NavDataBuilder::new(0.6);

    for (transform, collider) in walls.iter() {
        let polygon = collider.to_polygon(transform);
        log::info!("Adding polygon: {:?}", polygon);
        nav_data_builder.push_polygon(polygon);
    }

    let insert_polygons_time = build_time_start.elapsed();

    let nav_data = nav_data_builder.build();

    let build_time = build_time_start.elapsed();

    hivemind.nav_data = Some(nav_data);

    log::info!(
        "Building nav data complete - insert polygons: {:?}, build: {:?}",
        insert_polygons_time,
        build_time
    );
}
