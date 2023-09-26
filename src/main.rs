use bevy::prelude::*;
use std::f32::consts::TAU;

const PLAYERSIZE: f32 = 1.0;
const PLAYERCOLOR: Color = Color::rgb(1.0, 0.5, 0.5);
const PLAYER_MOVEMENT_SPEED: f32 = 2.5;
const PLAYER_ROTATION_SPEED: f32 = 1.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, rotate_camera)
        .add_systems(Update, move_player.after(rotate_camera))
        .run()
}

#[derive(Component)]
struct Health{
    hp: f32,
    shield: f32
}

#[derive(Component)]
struct Experience{
    exp: f32
}

#[derive(Component)]
struct MyPlayer;

#[derive(Bundle)]
struct PlayerBundle{
    player: MyPlayer,
    hp: Health,
    xp: Experience,
    body: PbrBundle,
}


fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    //spawn Player
    commands.spawn(PlayerBundle{
        player: MyPlayer,
        hp: Health { hp: 100.0, shield: 0.0 },
        xp: Experience { exp: 0.0 },
        body: PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Capsule { radius: 0.175 * PLAYERSIZE, depth: 0.4 * PLAYERSIZE, ..default()})),
            material: materials.add(PLAYERCOLOR.into()),
            transform: Transform::from_xyz(0.0, 0.3, 0.0),
            ..default()
        }
    });

    //floor
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(5.0).into()),
        material: materials.add(Color::rgb(0.9, 0.9, 0.9).into()),
        ..default()
    });

    //Light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            color: Color::rgb(1.0, 1.0, 1.0),
            ..default()
        },
        transform: Transform::from_xyz(0.0, 8.0, 0.0),
        ..default()
    });

    //main Camera
    commands.spawn( Camera3dBundle {
        transform: Transform::from_xyz(0.0, 2.0, 2.5).looking_at(Vec3::new(0.0, 0.5, 0.0), Vec3::Y),
        ..default()
    });


}

fn rotate_camera(
    mut commands: Commands,
    mut keyboard_input: Res<Input<KeyCode>>,
    mut set: ParamSet<(Query<&mut Transform, With<Camera3d>>, Query<&mut Transform, With<MyPlayer>>)>,
    timer: Res<Time>
){
    if keyboard_input.pressed(KeyCode::A) | keyboard_input.pressed(KeyCode::Left){
        let player_translation = set.p1().single().translation;
        set.p0().single_mut().rotate_around(player_translation,Quat::from_axis_angle(Vec3 { x: 0.0, y: 1.0, z: 0.0 }, PLAYER_ROTATION_SPEED * TAU * timer.delta_seconds())); 
        set.p1().single_mut().rotate_axis(Vec3::Y, (PLAYER_ROTATION_SPEED) * TAU * timer.delta_seconds());
    }
    if keyboard_input.pressed(KeyCode::D) | keyboard_input.pressed(KeyCode::Right) {
        let player_translation = set.p1().single().translation;
        set.p0().single_mut().rotate_around(player_translation,Quat::from_axis_angle(Vec3 { x: 0.0, y: 1.0, z: 0.0 }, (-PLAYER_ROTATION_SPEED) * TAU * timer.delta_seconds())); 
        set.p1().single_mut().rotate_axis(Vec3::Y, (-PLAYER_ROTATION_SPEED) * TAU * timer.delta_seconds());
    }
}


fn move_player(
    mut commands: Commands,
    mut keyboard_input: Res<Input<KeyCode>>,
    mut set: ParamSet<(Query<&mut Transform, With<Camera3d>>, Query<&mut Transform, With<MyPlayer>>)>,
    timer: Res<Time>
) {
    if keyboard_input.pressed(KeyCode::W) | keyboard_input.pressed(KeyCode::Up){
        let forward = set.p1().single().forward();
        set.p1().single_mut().translation += forward * timer.delta_seconds() * PLAYER_MOVEMENT_SPEED;
        set.p0().single_mut().translation += forward * timer.delta_seconds() * PLAYER_MOVEMENT_SPEED;
    }
    if keyboard_input.pressed(KeyCode::S) | keyboard_input.pressed(KeyCode::Down){
        let back = set.p1().single().back();
        set.p1().single_mut().translation += back * timer.delta_seconds() * PLAYER_MOVEMENT_SPEED;
        set.p0().single_mut().translation += back * timer.delta_seconds() * PLAYER_MOVEMENT_SPEED;
    }
}


//TODO: make event driven movement
//TODO: simulate gravity and check for collisions
//TODO: build map