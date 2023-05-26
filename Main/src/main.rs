use bevy::{prelude::*,};
use bevy::{app::App, DefaultPlugins};

#[derive(Component)]
struct Camera;

#[derive(Component)]
struct Background;

#[derive(Component, Clone)]
pub struct Player{
    pub animation: i8
}

#[derive(Component)]
pub struct Boxes{
    // if boxType = true, Box is banana; this will be loaded accordingly
    pub boxType: bool,
    pub boxValue: i32,
    pub inPlay: bool
}

fn main() {
    App::new()
    // bevy
    .add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Sorting Boxes".into(),
            resolution: (1297., 716.).into(),
            resizable: false,
            focused: true,
            ..default()
        }),
        ..default()
    }))

    // system to run on boot
        .add_startup_system(setup)
    // systems will run each frame
        .add_system(bevy::window::close_on_esc)
        .add_system(tick)
        .add_system(boxMovement)
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

        let mut texturePath: &str = "";
        let mut boxType = false;
        for boxnum in 1..21{
            if boxnum % 2 == 0 {
                boxType = true;
                texturePath = "C:/Programming/Python/Rust/Gather/projectAssets/banana.png";
            } else {
                boxType = false;
                texturePath = "C:/Programming/Python/Rust/Gather/projectAssets/orange.png";
    
            }
            commands.spawn(SpriteBundle{
                transform: Transform {translation: Vec3::new(-400., 330., 1.), scale: Vec3::new(0.2, 0.2, 1.), ..default()},
                texture: asset_server.load(texturePath),
                ..default()
            })
            .insert(Boxes{
                boxType: boxType,
                boxValue: boxnum,
                inPlay: false
            });
        }

}

fn tick(
    time: Res<Time>, 
    mut char_evr : EventReader<ReceivedCharacter>,
    mut query: Query<(&Player, &mut Transform, Entity)>,
    mut commands: Commands,
){
    let (player, mut transform, entity) = query.single_mut();

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


fn boxMovement(
    time: Res<Time>,
    mut query: Query<(&Boxes, &mut Transform)>
){
    for boxy in query.iter(){
        // this will iterate through each boxe that was rendered, this will also let us move certain boxes
        println!("{} is inplay: {}", boxy.0.boxValue, boxy.0.inPlay)
        if boxy.0.inPlay == true{
            
        }
    }
}
