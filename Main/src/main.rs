use bevy::ecs::query;
use bevy::{prelude::*, transform};
use bevy::{app::App, DefaultPlugins};

#[derive(Component)]
struct Camera;

#[derive(Component)]
struct Background;

#[derive(Component, Clone)]
pub struct Player{
    pub animation: i8
}

fn main() {
    App::new()
    // bevy
        .add_plugins(DefaultPlugins)

    // system to run on boot
        .add_startup_system(setup)
    // systems will run each frame
        .add_system(bevy::window::close_on_esc)
        .add_system(tick)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>){
    // create the 2d camera
    commands.spawn((
        Camera2dBundle::default(),
        Camera
    ));


    // create the background image
    commands.spawn(SpriteBundle {
        transform: Transform {scale: Vec3::new(1.125, 1.105, 1.), ..Default::default() },
        texture: asset_server.load("C:/Programming/Python/Rust/Gather/projectAssets/Background.png"),
        ..default()
    });

    // creates player object

    commands
        .spawn( SpriteBundle {
        visibility: Visibility::Visible,
        transform: Transform {translation: Vec3::new(-600., 100., 1.), scale: Vec3::new(0.25, 0.25, 1.), rotation: Quat::from_rotation_z(3. * 1.570796)},
        texture: asset_server.load("C:/Programming/Python/Rust/Gather/projectAssets/person.png"),
        ..default()
        })
        .insert(Player{
            animation: 0
        });

}

fn tick(
    time: Res<Time>, 
    mut char_evr : EventReader<ReceivedCharacter>,
    mut query: Query<(&Player, &mut Transform)>
){
    let (player, mut transform) = query.single_mut();




    for event in char_evr.iter() {
        // player movement
        if event.char == 'w'{
            transform.translation.y += 300. * time.delta_seconds();
        } else if event.char == 's'{
            transform.translation.y -= 300. * time.delta_seconds();
        }

        if event.char == 'd' {
            transform.translation.x += 300. * time.delta_seconds();
        } else if event.char == 'a' {
            transform.translation.x -= 300. * time.delta_seconds();
        }

        
    }
    
}   
