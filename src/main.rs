//WDGTR
//NCWH
//41-38

use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*, asset::HandleId,
};
use bevy_tilemap::prelude::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .init_resource::<ButtonMaterials>()
        .add_state(AppState::Menu)
        .add_system_set(SystemSet::on_enter(AppState::Menu).with_system(setup_menu.system()))
        .add_system_set(SystemSet::on_update(AppState::Menu).with_system(menu.system()))
        .add_system_set(SystemSet::on_exit(AppState::Menu).with_system(cleanup_menu.system()))
        .add_system_set(SystemSet::on_enter(AppState::InGame).with_system(setup_game.system()))
        .add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(movement.system())
                .with_system(change_color.system()),
        )
        .run();
}


#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    Menu,
    InGame,
}

struct MenuData {
    button_entity: Entity,
}

struct ButtonMaterials {
    normal: Handle<ColorMaterial>,
    hovered: Handle<ColorMaterial>,
    pressed: Handle<ColorMaterial>,
}

// A unit struct to help identify the FPS UI component, since there may be many Text components
struct FpsText;

// A unit struct to help identify the color-changing Text component
struct ColorText;

fn button_system(
    button_materials: Res<ButtonMaterials>,
    mut interaction_query: Query<
        (&Interaction, &mut Handle<ColorMaterial>, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut material, children) in interaction_query.iter_mut() {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                text.sections[0].value = "Press".to_string();
                *material = button_materials.pressed.clone();
            }
            Interaction::Hovered => {
                text.sections[0].value = "Start".to_string();
                *material = button_materials.hovered.clone();
            }
            Interaction::None => {
                text.sections[0].value = "Start".to_string();
                *material = button_materials.normal.clone();
            }
        }
    }
}

fn setup_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    button_materials: Res<ButtonMaterials>,
) {
    // ui camera
    commands.spawn_bundle(UiCameraBundle::default());
    let button_entity = commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                // center button
                margin: Rect::all(Val::Auto),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..Default::default()
            },
            material: button_materials.normal.clone(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "Play",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                    Default::default(),
                ),
                ..Default::default()
            });
        })
        .id();
    commands.insert_resource(MenuData { button_entity });
}

fn menu (
    mut state: ResMut<State<AppState>>,
    button_materials: Res<ButtonMaterials>,
    mut interaction_query: Query<
        (&Interaction, &mut Handle<ColorMaterial>),
        (Changed<Interaction>, With<Button>),>,
) {
    for (interaction, mut material) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *material = button_materials.pressed.clone();
                state.set(AppState::InGame).unwrap();
            }
            Interaction::Hovered => {
                *material = button_materials.hovered.clone();
            }
            Interaction::None => {
                *material = button_materials.normal.clone();
            }
        }
    }
}

fn cleanup_menu(mut commands: Commands, menu_data: Res<MenuData>) {
    commands.entity(menu_data.button_entity).despawn_recursive();
}

fn setup_game(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let texture_handle = asset_server.load("Images/tux.png");
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(SpriteBundle {
        material: materials.add(texture_handle.into()),
        ..Default::default()
    });
}

const SPEED: f32 = 100.0;
fn movement(
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Sprite>>,
) {
    for mut transform in query.iter_mut() {
        let mut direction = Vec3::ZERO;
        if input.pressed(KeyCode::Left) {
            direction.x += 1.0;
        }
        if input.pressed(KeyCode::Right) {
            direction.x -= 1.0;
        }
        if input.pressed(KeyCode::Up) {
            direction.y += 1.0;
        }
        if input.pressed(KeyCode::Down) {
            direction.y -= 1.0;
        }

        if direction != Vec3::ZERO {
            transform.translation += direction.normalize() * SPEED * time.delta_seconds();
        }
    }
}

fn change_color(
    time: Res<Time>,
    mut assets: ResMut<Assets<ColorMaterial>>,
    query: Query<&Handle<ColorMaterial>, With<Sprite>>,
) {
    for handle in query.iter() {
        let material = assets.get_mut(handle).unwrap();
        material
            .color
            .set_b((time.seconds_since_startup() * 5.0).sin() as f32 + 2.0);
    }
}

fn tile_map() {
    // Build a default Tilemap with 32x32 pixel tiles.
    let mut tilemap = Tilemap::default();

    // We need a Asset<TextureAtlas>. For this example we get a random one as a placeholder.
    let texture_atlas_handle = Handle::weak(HandleId::random::<TextureAtlas>());

    // Set the texture atlas for the Tilemap
    tilemap.set_texture_atlas(texture_atlas_handle);

    // Create tile data
    let tile = Tile {
        // 2D location x,y (units are in tiles)
        point: (16,16),

        // Which tile from the TextureAtlas
        sprite_index: 0,

        // Which z-layer in the Tilemap (0-up)
        sprite_order: 0,

        // Give the tile an optional green tint
        tint: bevy::render::color::Color::GREEN,
    };

    // Insert a single tile
    tilemap.insert_tile( tile);
}

impl FromWorld for ButtonMaterials {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
        ButtonMaterials {
            normal: materials.add(Color::rgb(0.15, 0.15, 0.15).into()),
            hovered: materials.add(Color::rgb(0.25, 0.25, 0.25).into()),
            pressed: materials.add(Color::rgb(0.35, 0.75, 0.35).into()),
        }
    }
}



