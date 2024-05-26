use valence::interact_block::InteractBlockEvent;
use valence::message::SendMessage;
use valence::prelude::*;

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                init_clients,
                wait_for_events_in_stack,
                when_button_pressed_do,
            ),
        )
        .run();
}

struct WaitingPlayerQuery {
    ret: Option<String>,
    func: Box<dyn Fn(InteractBlockEvent) -> Option<String> + Send + Sync>,
}

#[derive(Resource)]
struct PlayerQueryStack {
    waiting_events: Vec<WaitingPlayerQuery>,
}

const BUTTON_LOCATION: BlockPos = BlockPos::new(1, 66, 0);
const TOGGLE_LOCATION: BlockPos = BlockPos::new(1, 64, 1);

fn wait_for_events_in_stack(
    mut events: EventReader<InteractBlockEvent>,
    // mut layers: Query<&mut ChunkLayer>,
    mut querystack: ResMut<PlayerQueryStack>,
    mut clients: Query<&mut Client>,
) {
    'queryloop: for (i, waiting_query) in querystack.waiting_events.iter_mut().enumerate() {
        if waiting_query.ret.is_none() {
            for event in events.read() {
                if let Some(ret) = (waiting_query.func)(*event) {
                    waiting_query.ret = Some(ret);
                    for mut client in &mut clients {
                        // let cp = ret.copy();
                        client.send_chat_message("wow".to_string());
                    }
                    querystack.waiting_events.remove(i);
                    break 'queryloop;
                }
            }
        }
    }
}

fn create_button(
    mut layers: &mut Query<&mut ChunkLayer>,
) -> Box<dyn Fn(InteractBlockEvent) -> Option<String> + Send + Sync> {
    let mut layer = layers.single_mut();

    let pos = BlockPos { x: 3, y: 65, z: 3 };

    let mut pos_button = pos.clone();

    pos_button.y += 1;

    layer.set_block(pos, BlockState::GOLD_BLOCK);

    layer.set_block(
        pos_button,
        BlockState::STONE_BUTTON.set(PropName::Face, PropValue::Floor),
    );

    return Box::new(move |interaction: InteractBlockEvent| {
        if interaction.position == pos_button {
            return Some("wow".to_string());
        } else {
            return None;
        }
    });
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

    layer.chunk.set_block(
        BUTTON_LOCATION,
        BlockState::STONE_BUTTON.set(PropName::Facing, PropValue::East),
    );

    commands.spawn(layer);

    commands.insert_resource(PlayerQueryStack {
        waiting_events: Vec::new(),
    });
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

fn when_button_pressed_do(
    mut events: EventReader<InteractBlockEvent>,
    mut layers: Query<&mut ChunkLayer>,
    mut querystack: ResMut<PlayerQueryStack>,
) {
    for event in events.read() {
        if event.position == BUTTON_LOCATION {
            let func = create_button(&mut layers);

            querystack.waiting_events.push(WaitingPlayerQuery {
                ret: None,
                func: func,
            });
        }
    }
}
