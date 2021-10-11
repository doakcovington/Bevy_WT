use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

mod helpers;

fn startup(
    mut commands: Commands,
    asset_server: Res<AssestServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut map_query: MapQuery,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let texture_handle = asset_server.load("tiles.png");
    let material_handle = materials.add(ColorMaterial::texture(texture_handle));

    // Create map entity and component
    let map_entity = commands.spawn().id();
    let mut map = Map::new(0u16, map_entity);

    // Creates a new layer builder with a layer entity
    let (mut layer_builder, _) = LayerBuilder::new(
        &mut commands,
        LayerSettings::new(
            MapSize(2,2),
            ChunkSize(8,8),
            TileSize(16.0, 16.0),
            TextureSize(96.0,16.0),
        ),
        0u16,
        0u16,
        None
    );
    layer_builder.set_all(TileBundle::default());
}