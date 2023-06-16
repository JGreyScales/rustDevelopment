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
    pub score: i16
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
pub struct Timer{
    pub time: f64,
    pub delay: f64
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
        .add_system(text_update_system)
        .add_system(respawn_boxes)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut timerQuery: Query<(&mut Timer)>){

    // create the 2d camera
    commands.spawn((
        Camera2dBundle::default(),
        Camera
    ));


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
        transform: Transform {translation: Vec3::new(-600., 100., 2.), scale: Vec3::new(0.25, 0.25, 1.), rotation: Quat::from_rotation_z(0. * 1.570796)},
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
            score: 0
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


    let mut timer: Mut<Timer> = timerQuery.single_mut();
    timer.delay = 15.;
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
            println!("x:{}, y:{}", transform.translation.x, transform.translation.y);
            // player interaction handler + code above
            if event.char == 'e'{
                player.interacting = true;
            }
    
            // player movement
            // Todo, remove frame skip issue
            if event.char == 'w' && player.positiony < 327.{
                transform.translation.y += 300. * time.delta_seconds();
    
            } else if event.char == 's' && player.positiony > -320.{
                transform.translation.y -= 300. * time.delta_seconds();
            }
    
            if event.char == 'd' && player.positionx <  620.{
                transform.translation.x += 300. * time.delta_seconds();
            } else if event.char == 'a' && player.positionx > -616.{
                transform.translation.x -= 300. * time.delta_seconds();
            }
    
           
    
            
        } 
    
    }
}   


fn boxMovement(
    time: Res<Time>,
    mut query: Query<(&mut Boxes, &mut Transform)>,
    mut playerQuery: Query<(&mut Player)>,
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
                    } else if  transform.translation.x < -229. && transform.translation.x > -392. && boxy.box_type == true{
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
            //         // if  (player - box).abs().floor() < 10; player is in range, this must be compared for both x and y
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
    mut playerquery: Query<&mut Player>
) {

    let player: Mut<Player> = playerquery.single_mut();
    for mut text in &mut query {
                text.sections[0].value = format!("Score\n{}", player.score);
    }
}


fn respawn_boxes(
    mut boxQuery: Query<(&mut Boxes, &mut Transform)>,
    mut timerQuery: Query<(&mut Timer)>,
    delta: Res<Time>,
){

    let mut outOfPlay: Vec<u8> = Vec::new();
    let timer: Mut<Timer> = timerQuery.single_mut();

    boxQuery.for_each_mut(|mut boxx:(Mut<Boxes>, Mut<Transform>)|{
        // for each box, if not in play; add to outOfPlay. 
    });
    //Select random number from outOfPlay 20 * Sin(timer / deltatime * 42 - timer)
    //place box at correct position
    //render box to scene
    // decrease delay by using a log function to create a exponentionally small curve in timing
    // move timer down


}