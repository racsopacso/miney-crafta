use valence::prelude::*;
use valence::interact_block::InteractBlockEvent;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                init_clients,
                when_button_pressed_do,
            ),
        )
        .run();
}

const BUTTON_LOCATION: BlockPos = BlockPos::new(1, 66, 0);
const TOGGLE_LOCATION: BlockPos = BlockPos::new(1, 64, 1);

fn when_button_pressed_do(mut events: EventReader<InteractBlockEvent>, mut layers: Query<&mut ChunkLayer>){
    for event in events.read(){
        if event.position == BUTTON_LOCATION {
            let mut layer = layers.single_mut();

            let blockref = layer.block(TOGGLE_LOCATION);

            let block_to_set = if blockref.is_some() && blockref.unwrap().state == BlockState::NETHERRACK{
                BlockState::GLOWSTONE
            } else {
                BlockState::NETHERRACK
            };

            layer.set_block(TOGGLE_LOCATION, block_to_set);
            
        }
    }
}

fn setup(
    mut commands: Commands,
    dimensions: Res<DimensionTypeRegistry>,
    biomes: Res<BiomeRegistry>,
    server: Res<Server>,
) {
    let mut layer: LayerBundle =
        LayerBundle::new(ident!("overworld"), &dimensions, &biomes, &server);

    for z in -5..5 {
        for x in -5..5 {
            layer.chunk.insert_chunk([x, z], UnloadedChunk::new());
        }
    }

    for z in -50..50 {
        for x in -50..50 {
            layer.chunk.set_block([x, 64, z], BlockState::GRASS_BLOCK);
        }
    }

    for y in 65..67 {
        layer.chunk.set_block([0, y, 0], BlockState::STONE);
    }

    layer.chunk.set_block(BUTTON_LOCATION, BlockState::STONE_BUTTON.set(PropName::Facing, PropValue::East));

    commands.spawn(layer);
}

fn init_clients(
    mut clients: Query<
        (
            &mut EntityLayerId,
            &mut VisibleChunkLayer,
            &mut VisibleEntityLayers,
            &mut Position,
            &mut GameMode,
        ),
        Added<Client>,
    >,
    layers: Query<Entity, With<ChunkLayer>>,
) {
    for (
        mut layer_id,
        mut visible_chunk_layer,
        mut visible_entity_layers,
        mut pos,
        mut game_mode,
    ) in &mut clients
    {
        let layer = layers.single();

        layer_id.0 = layer;
        visible_chunk_layer.0 = layer;
        visible_entity_layers.0.insert(layer);
        pos.set([0.5, 65.0, 0.5]);
        *game_mode = GameMode::Creative;
    }
}
