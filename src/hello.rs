use bevy::prelude::*;

struct Person;
struct Name(String);
struct GreetTimer(Timer);

fn add_people(mut commands: Commands) {
    commands
        .spawn((Person, Name("Jenk".to_string())))
        .spawn((Person, Name("Brian".to_string())))
        .spawn((Person, Name("Brinty".to_string())))
        .spawn((Person, Name("Jams".to_string())));
}

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<(&Person, &Name)>) {
    
    timer.0.tick(time.delta_seconds);

    if timer.0.finished {
        for (_person, name) in query.iter() {
            println!("hello {0}!", name.0);
        }
    }
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(GreetTimer(Timer::from_seconds(2.0, true)))
            .add_startup_system(add_people.system())
            .add_system(greet_people.system());
    }
}