
use bevy::{prelude::*};
use bevy::{app::App, DefaultPlugins};

#[derive(Component)]
struct Camera;

#[derive(Component)]
struct Background;

#[derive(Component)]
struct scoretext;

#[derive(Component, Clone)]
pub struct Player{
    pub animation: i8,
    pub gaming: bool,
    pub positionx: f32,
    pub positiony: f32,
    pub carryingBox: bool,
    pub interacting: bool,
    pub score: i16,
    pub rotation: u8
}

#[derive(Component)]
pub struct Boxes{
    // if boxType = true, Box is banana; this will be loaded accordingly
    pub box_type: bool,
    pub box_value: u8,
    pub in_play: bool,
    pub speed: f32,
    pub picked_up: bool,
    pub laying: bool
}

#[derive(Component)]
pub struct Timer{
    pub time: f64,
    pub delay: f64,
    pub count: f64
}
fn main() {
    App::new()
    // bevy
    .add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Sorting Boxes".into(),
            resolution: (1297., 716.).into(),
            resizable: false,
            mode: bevy::window::WindowMode::Windowed,
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
        .add_system(text_update_system)
        .add_system(respawn_boxes)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>){

    // create the 2d camera
    commands.spawn((
        Camera2dBundle::default(),
        Camera
    ));

    commands.spawn(Timer{
        time: 15.,
        delay: 15.,
        count: 0.
    });

    // create the background image
    commands.spawn(SpriteBundle {
        transform: Transform {scale: Vec3::new(1.125, 1.105, -3.), ..Default::default() },
        texture: asset_server.load("C:/Programming/Python/Rust/Gather/projectAssets/Background.png"),
        ..default()
    });

    // creates player object

    commands
        .spawn( SpriteBundle {
        visibility: Visibility::Visible,
        transform: Transform {translation: Vec3::new(-600., 100., 2.), scale: Vec3::new(0.25, 0.25, 1.), rotation: Quat::from_rotation_z(-1. * 1.570796)},
        texture: asset_server.load("C:/Programming/Python/Rust/Gather/projectAssets/person.png"),
        ..default()
        })
        .insert(Player{
            animation: 0,
            gaming: true,
            positionx: -600.,
            positiony: 100.,
            carryingBox: false,
            interacting: false,
            score: 0,
            rotation: 1
        });

        // creates an even amount of orange and banana boxes based on the uper limit - 1
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
                transform: Transform {translation: Vec3::new(-400., 330., 1.), scale: Vec3::new(0.2, 0.2, -1.), ..default()},
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

        // Text with multiple sections
        commands.spawn((
            // Create a TextBundle that has a Text with a list of sections.
            TextBundle::from_sections([
                TextSection::new(
                    "Score\n0",
                    TextStyle {
                        font: asset_server.load("C:/Programming/Python/Rust/Gather/projectAssets/RetroChildRegular-7BnMl.ttf"),
                        font_size: 60.0,
                        color: Color::BLACK,
                    },
                ),
            ]),
            scoretext,
        )
    )
    .insert(ZIndex::Global(42));


}



// returns a float 32
fn smartRotation(
    mut playerRotation: u8,
    targetRotation: u8
) -> f32 {


// super lazy but efficent check to make sure the player isn't already facing the target direction
    if playerRotation == targetRotation{
        return 0.;
    }


    // for 1 -> 3 turns (A 270 when multipled by 90)
    for _x in 1..4{


        // increase the player rotation
        playerRotation += 1;

        // peform a loop over so that we aren't infinitely counting (overflow fun zone)
        if playerRotation == 4 {
            playerRotation = 0;
        }

        // if player rotation is equal to supplied target rotation; return the total amount of turns taken.
        if playerRotation == targetRotation{
            return -_x as f32;
        }

    }
    return 404.;
}


fn tick(
    time: Res<Time>, 
    mut char_evr : EventReader<ReceivedCharacter>,
    mut query: Query<(&mut Player, &mut Transform)>,
){

    let (mut player, mut transform) = query.single_mut();

    player.positionx = transform.translation.x;
    player.positiony = transform.translation.y;

    // Todo, Create logic for gaming = false to end game
    if char_evr.is_empty() == true{
        player.interacting = false;
    } else {
        for event in char_evr.iter() {
            // println!("x:{}, y:{}", transform.translation.x, transform.translation.y);
            // player interaction handler + code above
            if event.char == 'e'{
                player.interacting = true;
            }
    
            // player movement
            // Todo, remove frame skip issue
            if event.char == 'w' && player.positiony < 327.{
                transform.rotate_z(smartRotation(player.rotation, 0) * 1.570796);
                player.rotation = 0;

                transform.translation.y += 300. * time.delta_seconds();
    
            } else if event.char == 's' && player.positiony > -320.{
                transform.rotate_z(smartRotation(player.rotation, 2) * 1.570796);
                player.rotation = 2;

                transform.translation.y -= 300. * time.delta_seconds();
            }
    
            if event.char == 'd' && player.positionx <  620.{
                transform.rotate_z(smartRotation(player.rotation, 1) * 1.570796);
                player.rotation = 1;

                transform.translation.x += 300. * time.delta_seconds();
            } else if event.char == 'a' && player.positionx > -616.{
                transform.rotate_z(smartRotation(player.rotation, 3) * 1.570796);
                player.rotation = 3;

                transform.translation.x -= 300. * time.delta_seconds();
            }
    
           
    
            
        } 
    
    }
}   


fn boxMovement(
    time: Res<Time>,
    mut query: Query<(&mut Boxes, &mut Transform)>,
    mut playerQuery: Query<&mut Player>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
){
    let mut player = playerQuery.single_mut();

    query.for_each_mut( |boxx| {

        let (mut boxy, mut transform) = boxx;
        if boxy.in_play == true{
            if boxy.picked_up == true{
                transform.translation.y = player.positiony;
                transform.translation.x = player.positionx + 45.;
            } else if boxy.laying == true {
                // do nothing if the boxy is laying
                if transform.translation.y < -105. {
                    if transform.translation.x < -229. && transform.translation.x > -392. && boxy.box_type == false{
                        println!("Box Scored");
                        boxy.in_play = false;
                        player.score += 1;
                        audio.play(asset_server.load("C:/Programming/Python/Rust/Gather/projectAssets/1up.ogg"));
                    } else if  transform.translation.x < 369. && transform.translation.x > 152. && boxy.box_type == true{
                        println!("Box Scored");
                        boxy.in_play = false;
                        player.score += 1;
                        audio.play(asset_server.load("C:/Programming/Python/Rust/Gather/projectAssets/1up.ogg"));
                    }
                }
            }
            else {
                // move the box down the conveyor
                if transform.translation.y > 283.{
                    transform.translation.y -= 10. * time.delta_seconds();

                   

                // if the box has gone off the screen game over. Set the players struct to turn off the game
                } else if transform.translation.x > 677.{
                    player.gaming = false;

                // move the box accross the conveyor
                } else {
                    transform.translation.x += 10. * time.delta_seconds();
                }
            }

        } else {
            transform.translation.z = -10.
        }
    });
}


fn pickupBoxes(
    mut playerQuery: Query<&mut Player>,
    mut boxQuery: Query<(&mut Boxes, &mut Transform)>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>
){
        // if  (player - box).abs().floor() < 10; player is in range, this must be compared for both x and y
    let mut player: Mut<Player> = playerQuery.single_mut();
    
    if player.interacting == true{
        let mut pass: bool = false;
        boxQuery.for_each_mut(|mut boxx: (Mut<Boxes>, Mut<Transform>)|{
            if pass == true {

            } else {
                if (player.positionx - boxx.1.translation.x).abs().floor() < 30. && (player.positiony - boxx.1.translation.y).abs().floor() < 30. && player.carryingBox == false && boxx.0.in_play == true{
                        println!("Box {} picked up", boxx.0.box_value);
                        audio.play(asset_server.load("C:/Programming/Python/Rust/Gather/projectAssets/pickupbox.ogg"));
                        boxx.0.picked_up = true;
                        boxx.0.laying = true;
                        player.carryingBox = true;
                        pass = true;
    
                }
                else if player.carryingBox == true && boxx.0.picked_up == true{
                    println!("Box {} dropped", boxx.0.box_value);
                    audio.play(asset_server.load("C:/Programming/Python/Rust/Gather/projectAssets/dropbox.ogg"));
                    boxx.0.picked_up = false;
                    player.carryingBox = false;
                    pass = true
            }

            }
        })
    }
}

fn text_update_system(
    mut query: Query<&mut Text, With<scoretext>>,
    mut playerquery: Query<&mut Player>,
    mut timerQuery: Query<&mut Timer>,


) {
    let mut timer = timerQuery.single_mut();
    let player: Mut<Player> = playerquery.single_mut();
    for mut text in &mut query {
                text.sections[0].value = format!("Score\n{}\nTime\n{}", player.score, timer.count.floor());
    }
}


fn respawn_boxes(
    mut boxQuery: Query<(&mut Boxes, &mut Transform)>,
    mut timerQuery: Query<&mut Timer>,
    delta: Res<Time>,
){

    let mut timer = timerQuery.single_mut();

    if  0. > timer.time{

        let mut outOfPlay: Vec<u8> = Vec::new();

        boxQuery.for_each_mut(|mut boxx:(Mut<Boxes>, Mut<Transform>)|{
            // for each box, if not in play; add to outOfPlay. 
            if false == boxx.0.in_play{
                outOfPlay.push(boxx.0.box_value);
            }
        });
    
    
        let maxValue = outOfPlay.len() as f64;
    
        if 0. < maxValue{
            let boxChosen =  (maxValue * f64::sin(rand::random::<f64>()) - 1.).round() as usize;
    
            boxQuery.for_each_mut(|mut boxx:(Mut<Boxes>, Mut<Transform>)|{
                if boxx.0.box_value == outOfPlay[boxChosen]{
                    boxx.1.translation.x = -400.;
                    boxx.1.translation.y = 330.;
                    boxx.1.translation.z = 1.;
                    boxx.0.in_play = true;
                    boxx.0.laying = false;
                    boxx.0.picked_up = false;
                }
            });

        timer.time = timer.delay;
        
        // move the time delay down by 0.5 each time. By doing it this method I do not need to store the value of x; I can figure out what x was
        // by deconstructing previous value
        if 3.5 < timer.delay{
            timer.delay = -0.5 * ((15.5 - timer.delay + 0.5)/0.5) + 15.5;
        }

    // if timer is not counted down
    } 

    }else{
        timer.time -= delta.raw_delta_seconds_f64();
    }

    timer.count += delta.raw_delta_seconds_f64();



}