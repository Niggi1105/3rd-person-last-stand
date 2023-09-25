use bevy::prelude::*;

const PLAYERSIZE: Vec3 = Vec3::new(100., 100., 100.);
const PLAYERCOLOR: Color = Color::rgb(1.0, 0.5, 0.5);

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
enum GameState {
    Running,
    GameOver,
    #[default]
    Menu,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Score { score: 0 })
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (move_player, focus_camera).run_if(in_state(GameState::Running)),
        )
        .add_systems(OnEnter(GameState::Menu), main_menu);
}

#[derive(Default)]
struct Player {
    entity: Option<Entity>,
    x_pos: f32,
    y_pos: f32,
}

#[derive(Resource)]
struct Score {
    score: i32,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera3dBundle::default());

    commands.spawn(
        (SpriteBundle {
            transform: Transform {
                translation: Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                scale: PLAYERSIZE,
                ..default()
            },
            sprite: Sprite {
                color: PLAYERCOLOR,
                ..default()
            },
            ..default()
        }),
    );
}

fn main_menu() {}

fn focus_camera() {}

fn move_player(commands: Commands, keyboard_input: Res<Input<KeyCode>>) {}
