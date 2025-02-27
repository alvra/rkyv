use rand::Rng;
use rand_pcg::Lcg64Xsh32;
use rkyv::{
    archived_root, check_archived_root,
    ser::{
        serializers::{AlignedSerializer, BufferScratch, CompositeSerializer},
        Serializer,
    },
    AlignedVec, Archive, Deserialize, Infallible, Serialize,
};
use std::collections::HashMap;
trait Generate {
    fn generate<R: Rng>(rng: &mut R) -> Self;
}

impl Generate for () {
    fn generate<R: Rng>(_: &mut R) -> Self {}
}

impl Generate for bool {
    fn generate<R: Rng>(rng: &mut R) -> Self {
        rng.gen_bool(0.5)
    }
}

impl Generate for u32 {
    fn generate<R: Rng>(rng: &mut R) -> Self {
        rng.gen()
    }
}

impl Generate for f32 {
    fn generate<R: Rng>(rng: &mut R) -> Self {
        rng.gen()
    }
}

impl Generate for f64 {
    fn generate<R: Rng>(rng: &mut R) -> Self {
        rng.gen()
    }
}

impl<T1: Generate, T2: Generate> Generate for (T1, T2) {
    fn generate<R: Rng>(rng: &mut R) -> Self {
        (T1::generate(rng), T2::generate(rng))
    }
}

impl<T1: Generate, T2: Generate, T3: Generate> Generate for (T1, T2, T3) {
    fn generate<R: Rng>(rng: &mut R) -> Self {
        (T1::generate(rng), T2::generate(rng), T3::generate(rng))
    }
}

impl<T: Generate> Generate for [T; 4] {
    fn generate<R: Rng>(rng: &mut R) -> Self {
        [
            T::generate(rng),
            T::generate(rng),
            T::generate(rng),
            T::generate(rng),
        ]
    }
}

impl<T: Generate> Generate for Option<T> {
    fn generate<R: Rng>(rng: &mut R) -> Self {
        if rng.gen_bool(0.5) {
            Some(T::generate(rng))
        } else {
            None
        }
    }
}

fn generate_vec<R: Rng, T: Generate>(rng: &mut R, range: core::ops::Range<usize>) -> Vec<T> {
    let len = rng.gen_range(range);
    let mut result = Vec::with_capacity(len);
    for _ in 0..len {
        result.push(T::generate(rng));
    }
    result
}

#[derive(
    Debug, Archive, Serialize, Clone, Copy, Deserialize, serde::Deserialize, serde::Serialize,
)]
#[archive(check_bytes)]
#[repr(u8)]
pub enum GameType {
    Survival,
    Creative,
    Adventure,
    Spectator,
}

impl Generate for GameType {
    fn generate<R: Rng>(rand: &mut R) -> Self {
        match rand.gen_range(0..4) {
            0 => GameType::Survival,
            1 => GameType::Creative,
            2 => GameType::Adventure,
            3 => GameType::Spectator,
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}

#[derive(Debug, Archive, Serialize, Deserialize, serde::Deserialize, serde::Serialize)]
#[archive(check_bytes)]
pub struct Item {
    count: i8,
    slot: u8,
    id: String,
}

impl Generate for Item {
    fn generate<R: Rng>(rng: &mut R) -> Self {
        const IDS: [&str; 8] = [
            "dirt",
            "stone",
            "pickaxe",
            "sand",
            "gravel",
            "shovel",
            "chestplate",
            "steak",
        ];
        Self {
            count: rng.gen(),
            slot: rng.gen(),
            id: IDS[rng.gen_range(0..IDS.len())].to_string(),
        }
    }
}

#[derive(
    Debug, Archive, Serialize, Clone, Copy, Deserialize, serde::Serialize, serde::Deserialize,
)]
#[archive(check_bytes)]
pub struct Abilities {
    walk_speed: f32,
    fly_speed: f32,
    may_fly: bool,
    flying: bool,
    invulnerable: bool,
    may_build: bool,
    instabuild: bool,
}

impl Generate for Abilities {
    fn generate<R: Rng>(rng: &mut R) -> Self {
        Self {
            walk_speed: rng.gen(),
            fly_speed: rng.gen(),
            may_fly: rng.gen_bool(0.5),
            flying: rng.gen_bool(0.5),
            invulnerable: rng.gen_bool(0.5),
            may_build: rng.gen_bool(0.5),
            instabuild: rng.gen_bool(0.5),
        }
    }
}

#[derive(Debug, Archive, Serialize, Deserialize, serde::Deserialize, serde::Serialize)]
#[archive(check_bytes)]
pub struct Entity {
    id: String,
    pos: (f64, f64, f64),
    motion: (f64, f64, f64),
    rotation: (f32, f32),
    fall_distance: f32,
    fire: u16,
    air: u16,
    on_ground: bool,
    no_gravity: bool,
    invulnerable: bool,
    portal_cooldown: i32,
    uuid: [u32; 4],
    custom_name: Option<String>,
    custom_name_visible: bool,
    silent: bool,
    glowing: bool,
}

impl Generate for Entity {
    fn generate<R: Rng>(rng: &mut R) -> Self {
        const IDS: [&str; 8] = [
            "cow", "sheep", "zombie", "skeleton", "spider", "creeper", "parrot", "bee",
        ];
        const CUSTOM_NAMES: [&str; 8] = [
            "rainbow", "princess", "steve", "johnny", "missy", "coward", "fairy", "howard",
        ];

        Self {
            id: IDS[rng.gen_range(0..IDS.len())].to_string(),
            pos: <(f64, f64, f64) as Generate>::generate(rng),
            motion: <(f64, f64, f64) as Generate>::generate(rng),
            rotation: <(f32, f32) as Generate>::generate(rng),
            fall_distance: rng.gen(),
            fire: rng.gen(),
            air: rng.gen(),
            on_ground: rng.gen_bool(0.5),
            no_gravity: rng.gen_bool(0.5),
            invulnerable: rng.gen_bool(0.5),
            portal_cooldown: rng.gen(),
            uuid: <[u32; 4] as Generate>::generate(rng),
            custom_name: <Option<()> as Generate>::generate(rng)
                .map(|_| CUSTOM_NAMES[rng.gen_range(0..CUSTOM_NAMES.len())].to_string()),
            custom_name_visible: rng.gen_bool(0.5),
            silent: rng.gen_bool(0.5),
            glowing: rng.gen_bool(0.5),
        }
    }
}

#[derive(Debug, Archive, Serialize, Deserialize, serde::Deserialize, serde::Serialize)]
#[archive(check_bytes)]
pub struct RecipeBook {
    recipes: Vec<String>,
    to_be_displayed: Vec<String>,
    is_filtering_craftable: bool,
    is_gui_open: bool,
    is_furnace_filtering_craftable: bool,
    is_furnace_gui_open: bool,
    is_blasting_furnace_filtering_craftable: bool,
    is_blasting_furnace_gui_open: bool,
    is_smoker_filtering_craftable: bool,
    is_smoker_gui_open: bool,
}

impl Generate for RecipeBook {
    fn generate<R: Rng>(rng: &mut R) -> Self {
        const RECIPES: [&str; 8] = [
            "pickaxe",
            "torch",
            "bow",
            "crafting table",
            "furnace",
            "shears",
            "arrow",
            "tnt",
        ];
        const MAX_RECIPES: usize = 30;
        const MAX_DISPLAYED_RECIPES: usize = 10;
        Self {
            recipes: generate_vec::<_, ()>(rng, 0..MAX_RECIPES)
                .iter()
                .map(|_| RECIPES[rng.gen_range(0..RECIPES.len())].to_string())
                .collect(),
            to_be_displayed: generate_vec::<_, ()>(rng, 0..MAX_DISPLAYED_RECIPES)
                .iter()
                .map(|_| RECIPES[rng.gen_range(0..RECIPES.len())].to_string())
                .collect(),
            is_filtering_craftable: rng.gen_bool(0.5),
            is_gui_open: rng.gen_bool(0.5),
            is_furnace_filtering_craftable: rng.gen_bool(0.5),
            is_furnace_gui_open: rng.gen_bool(0.5),
            is_blasting_furnace_filtering_craftable: rng.gen_bool(0.5),
            is_blasting_furnace_gui_open: rng.gen_bool(0.5),
            is_smoker_filtering_craftable: rng.gen_bool(0.5),
            is_smoker_gui_open: rng.gen_bool(0.5),
        }
    }
}

#[derive(Debug, Archive, Serialize, Deserialize, serde::Deserialize, serde::Serialize)]
#[archive(check_bytes)]
pub struct Player {
    game_type: GameType,
    previous_game_type: GameType,
    score: u64,
    dimension: String,
    selected_item_slot: u32,
    selected_item: Item,
    spawn_dimension: Option<String>,
    spawn_x: i64,
    spawn_y: i64,
    spawn_z: i64,
    spawn_forced: Option<bool>,
    sleep_timer: u16,
    food_exhaustion_level: f32,
    food_saturation_level: f32,
    food_tick_timer: u32,
    xp_level: u32,
    xp_p: f32,
    xp_total: i32,
    xp_seed: i32,
    inventory: Vec<Item>,
    ender_items: Vec<Item>,
    abilities: Abilities,
    entered_nether_position: Option<(f64, f64, f64)>,
    root_vehicle: Option<([u32; 4], Entity)>,
    shoulder_entity_left: Option<Entity>,
    shoulder_entity_right: Option<Entity>,
    seen_credits: bool,
    recipe_book: RecipeBook,
}

impl Generate for Player {
    fn generate<R: Rng>(rng: &mut R) -> Self {
        const DIMENSIONS: [&str; 3] = ["overworld", "nether", "end"];
        const MAX_ITEMS: usize = 40;
        const MAX_ENDER_ITEMS: usize = 27;
        Self {
            game_type: GameType::generate(rng),
            previous_game_type: GameType::generate(rng),
            score: rng.gen(),
            dimension: DIMENSIONS[rng.gen_range(0..DIMENSIONS.len())].to_string(),
            selected_item_slot: rng.gen(),
            selected_item: Item::generate(rng),
            spawn_dimension: <Option<()> as Generate>::generate(rng)
                .map(|_| DIMENSIONS[rng.gen_range(0..DIMENSIONS.len())].to_string()),
            spawn_x: rng.gen(),
            spawn_y: rng.gen(),
            spawn_z: rng.gen(),
            spawn_forced: <Option<bool> as Generate>::generate(rng),
            sleep_timer: rng.gen(),
            food_exhaustion_level: rng.gen(),
            food_saturation_level: rng.gen(),
            food_tick_timer: rng.gen(),
            xp_level: rng.gen(),
            xp_p: rng.gen(),
            xp_total: rng.gen(),
            xp_seed: rng.gen(),
            inventory: generate_vec(rng, 0..MAX_ITEMS),
            ender_items: generate_vec(rng, 0..MAX_ENDER_ITEMS),
            abilities: Abilities::generate(rng),
            entered_nether_position: <Option<(f64, f64, f64)> as Generate>::generate(rng),
            root_vehicle: <Option<([u32; 4], Entity)> as Generate>::generate(rng),
            shoulder_entity_left: <Option<Entity> as Generate>::generate(rng),
            shoulder_entity_right: <Option<Entity> as Generate>::generate(rng),
            seen_credits: rng.gen_bool(0.5),
            recipe_book: RecipeBook::generate(rng),
        }
    }
}

fn generate_player_name<R: Rng>(rng: &mut R) -> String {
    const LEGAL_CHARS: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz_";

    let len = rng.gen_range(10..40);
    let mut result = String::new();

    for _ in 0..len {
        result.push(LEGAL_CHARS[rng.gen_range(0..LEGAL_CHARS.len())] as char);
    }

    result
}

fn main() {
    const PLAYERS: usize = 500;
    const STATE: u64 = 3141592653;
    const STREAM: u64 = 5897932384;

    type Players = HashMap<String, Player>;
    let mut players: Players = HashMap::with_capacity(PLAYERS);
    let mut rng = Lcg64Xsh32::new(STATE, STREAM);
    for _ in 0..PLAYERS {
        let mut name = generate_player_name(&mut rng);
        while players.contains_key(&name) {
            name = generate_player_name(&mut rng);
        }
        players.insert(name, Player::generate(&mut rng));
    }

    const BUFFER_LEN: usize = 10_000_000;
    const SCRATCH_LEN: usize = 512_000;

    let mut serialize_buffer = AlignedVec::with_capacity(BUFFER_LEN);
    let mut serialize_scratch = AlignedVec::with_capacity(SCRATCH_LEN);
    unsafe {
        serialize_scratch.set_len(SCRATCH_LEN);
    }
    serialize_buffer.clear();
    let mut serializer = CompositeSerializer::new(
        AlignedSerializer::new(&mut serialize_buffer),
        BufferScratch::new(&mut serialize_scratch),
        Infallible,
    );
    serializer.serialize_value(&players).unwrap();

    let buf = serializer.into_serializer().into_inner();

    unsafe { archived_root::<Players>(buf.as_ref()) };
    check_archived_root::<Players>(buf.as_ref()).unwrap();

    let value = unsafe { archived_root::<Players>(buf.as_ref()) };
    let deserialized: Players = value.deserialize(&mut Infallible).unwrap();

    println!("{:?}", deserialized);

    let value = check_archived_root::<Players>(buf.as_ref()).unwrap();
    let deserialized: Players = value.deserialize(&mut Infallible).unwrap();

    println!("{:?}", deserialized);
}
