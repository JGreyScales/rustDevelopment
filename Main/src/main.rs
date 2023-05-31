
use bevy::{prelude::*};
use bevy::{app::App, DefaultPlugins};

#[derive(Component)]
struct Camera;

#[derive(Component)]
struct Background;

#[derive(Component, Clone)]
pub struct Player{
    pub animation: i8,
    pub gaming: bool,
    pub positionx: f32,
    pub positiony: f32,
    pub carryingBox: bool,
    pub interacting: bool
}

#[derive(Component)]
pub struct Boxes{
    // if boxType = true, Box is banana; this will be loaded accordingly
    pub box_type: bool,
    pub box_value: i32,
    pub in_play: bool,
    pub speed: f32,
    pub picked_up: bool,
    pub laying: bool
}
#[derive(Component)]
struct text{
    value:i8
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
        .add_system(pickupBoxes)
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
        transform: Transform {translation: Vec3::new(-600., 100., 2.), scale: Vec3::new(0.25, 0.25, 1.), rotation: Quat::from_rotation_z(3. * 1.570796)},
        texture: asset_server.load("C:/Programming/Python/Rust/Gather/projectAssets/person.png"),
        ..default()
        })
        .insert(Player{
            animation: 0,
            gaming: true,
            positionx: -600.,
            positiony: 100.,
            carryingBox: false,
            interacting: false
        });

        let mut texturePath: &str = "";
        let mut boxType = false;
        let mut inPlay = true;
        for boxnum in 1..21{
            if boxnum % 2 == 0 {
                boxType = true;
                inPlay = false;
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
                box_type: boxType,
                box_value: boxnum,
                in_play: inPlay,
                speed: 1.,
                picked_up: false,
                laying: false
            });
        }


}

fn tick(
    time: Res<Time>, 
    mut char_evr : EventReader<ReceivedCharacter>,
    mut query: Query<(&mut Player, &mut Transform)>,
    mut commands: Commands,
){

    let (mut player, mut transform) = query.single_mut();

    player.positionx = transform.translation.x;
    player.positiony = transform.translation.y;

    // Todo, Create logic for gaming = false to end game
    if char_evr.is_empty() == true{
        player.interacting = false;
    } else {
        for event in char_evr.iter() {
            // player interaction handler + code above
            if event.char == 'e'{
                player.interacting = true;
            }
    
            // player movement
            // Todo, remove frame skip issue
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
}   


fn boxMovement(
    time: Res<Time>,
    mut query: Query<(&mut Boxes, &mut Transform)>,
    mut playerQuery: Query<(&mut Player)>
){

    query.for_each_mut( |mut boxx| {

        let (mut boxy, mut transform) = boxx;
        
        if boxy.in_play == true && boxy.laying == false{
            if boxy.picked_up == true{

            } else {
                // move the box down the conveyor
                if transform.translation.y > 283.{
                    transform.translation.y -= 10. * time.delta_seconds();

                // if the box has gone off the screen game over. Set the players struct to turn off the game
                } else if transform.translation.x > 677.{
                    let mut player = playerQuery.single_mut();
                    player.gaming = false;

                // move the box accross the conveyor
                } else {
                    transform.translation.x += 10. * time.delta_seconds();
                }
            }

        } else {
            transform.translation.z = -1.
        }


    }
    );
}


fn pickupBoxes(
    mut playerQuery: Query<&mut Player>,
    mut boxQuery: Query<(&mut Boxes, &mut Transform)>
){
            //         // if  (player - box).abs().floor() < 10; player is in range, this must be compared for both x and y
    let mut player: Mut<Player> = playerQuery.single_mut();
    if player.interacting == true{
        boxQuery.for_each_mut(|mut boxx|{
            if (player.positionx - boxx.1.translation.x).abs().floor() < 25. && (player.positiony - boxx.1.translation.y).abs().floor() < 25.{
                if player.carryingBox == false {
                    boxx.0.picked_up = true;
                    player.carryingBox = true;
                    boxx.0.laying = false;
                } else {
                    boxx.0.picked_up = false;
                    player.carryingBox = false;
                    boxx.0.laying = true;
                }

            }
            // else {}
        })
    }
    
}