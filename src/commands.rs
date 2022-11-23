use bevy::app::AppExit;
use bevy::prelude::*;

pub struct Commands;

pub enum Command {
    Quit,
}

impl Plugin for Commands {
    fn build(&self, app: &mut App) {
        app.add_event::<Command>();
        app.add_system(handle_commands);
    }
}

fn handle_commands(// mut commands: Commands,
    // mut game_commands: EventReader<Command>,
    // mut exit: EventWriter<AppExit>,
) {
    // for cmd in game_commands.iter() {
    //     match cmd {
    //         Command::Quit => {
    //             exit.send(AppExit);
    //         }
    //     }
    // }
}
