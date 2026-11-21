//! Small helpers: the saved compound's typed reads and writes, items by name, block positions.

use mod_api::*;
use std::collections::HashMap;
use std::sync::OnceLock;

pub(crate) type Pos = [i32; 3];

pub(crate) fn java(s: &str) -> JavaString {
    JavaString::from(s)
}

pub(crate) fn get_i(nbt: &Compound, key: &str, default: i32) -> i32 {
    match nbt.get(&java(key)) {
        Some(Tag::Int(v)) => *v,
        _ => default,
    }
}

pub(crate) fn set_i(nbt: &mut Compound, key: &str, value: i32) {
    if get_i(nbt, key, value.wrapping_add(1)) != value {
        nbt.insert(java(key), Tag::Int(value));
    }
}

pub(crate) fn get_f(nbt: &Compound, key: &str) -> f64 {
    match nbt.get(&java(key)) {
        Some(Tag::Double(bits)) => f64::from_bits(*bits),
        _ => 0.0,
    }
}

pub(crate) fn set_f(nbt: &mut Compound, key: &str, value: f64) {
    if !matches!(nbt.get(&java(key)), Some(Tag::Double(bits)) if *bits == value.to_bits()) {
        nbt.insert(java(key), Tag::Double(value.to_bits()));
    }
}

pub(crate) fn int_array(nbt: &Compound, key: &str, len: usize) -> Vec<i32> {
    let mut out = match nbt.get(&java(key)) {
        Some(Tag::IntArray(a)) => a.to_vec(),
        _ => Vec::new(),
    };
    out.resize(len, 0);
    out
}

pub(crate) fn set_int_array(nbt: &mut Compound, key: &str, values: Vec<i32>) {
    if int_array(nbt, key, values.len()) != values || !matches!(nbt.get(&java(key)), Some(Tag::IntArray(_))) {
        nbt.insert(java(key), Tag::IntArray(values.into()));
    }
}

/// Item keys by id, looked up once (keys stay for the process).
fn item_ids(host: &dyn Host) -> &'static HashMap<String, ItemKey> {
    static IDS: OnceLock<HashMap<String, ItemKey>> = OnceLock::new();
    IDS.get_or_init(|| host.item_keys().into_iter().map(|k| (k.id(host), k)).collect())
}

pub(crate) fn item_key(host: &dyn Host, id: &str) -> Option<ItemKey> {
    item_ids(host).get(id).copied()
}

/// `count` of item `id`.
pub(crate) fn stack_of(host: &dyn Host, id: &str, count: u32) -> Option<ItemStack> {
    ItemStack::new(item_key(host, id)?, count)
}

pub(crate) fn id_of(host: &dyn Host, stack: &ItemStack) -> String {
    stack.item().id(host)
}

pub(crate) fn is(host: &dyn Host, stack: &Option<ItemStack>, id: &str) -> bool {
    stack.as_ref().is_some_and(|s| id_of(host, s) == id)
}

/// The item's own stack limit (`getMaxStackSize`), at most 64 as a container's.
pub(crate) fn max_stack(host: &dyn Host, stack: &ItemStack) -> u32 {
    host.max_stack_size(stack.item()).unwrap_or(64).clamp(1, 64)
}

/// `stack` with `count`, none for 0.
pub(crate) fn with_count(stack: &ItemStack, count: u32) -> Option<ItemStack> {
    let mut out = stack.clone();
    out.set_count(count).ok()?;
    Some(out)
}

pub(crate) fn shrink(slot: &mut Option<ItemStack>, by: u32) {
    if let Some(s) = slot.take() {
        *slot = s.shrink(by);
    }
}

pub(crate) const DIRECTIONS: [Direction; 6] =
    [Direction::Down, Direction::Up, Direction::North, Direction::South, Direction::West, Direction::East];

pub(crate) fn dir_index(d: Direction) -> usize {
    DIRECTIONS.iter().position(|x| *x == d).unwrap_or(0)
}

pub(crate) fn step(i: usize) -> Pos {
    [[0, -1, 0], [0, 1, 0], [0, 0, -1], [0, 0, 1], [-1, 0, 0], [1, 0, 0]][i]
}

pub(crate) fn opposite(i: usize) -> usize {
    [1, 0, 3, 2, 5, 4][i]
}

pub(crate) fn offset(pos: Pos, dir: usize) -> Pos {
    let s = step(dir);
    [pos[0] + s[0], pos[1] + s[1], pos[2] + s[2]]
}

pub(crate) fn centre(pos: Pos) -> [f64; 3] {
    [f64::from(pos[0]) + 0.5, f64::from(pos[1]) + 0.5, f64::from(pos[2]) + 0.5]
}

pub(crate) fn lang(host: &dyn ClientHost, key: &str, english: &str) -> String {
    host.translate(key).unwrap_or_else(|| english.to_owned())
}

/// A compound in the stack's `minecraft:custom_data` component.
pub(crate) fn custom_data(stack: &ItemStack) -> Option<Compound> {
    match stack.components()?.as_nbt().get(&java("minecraft:custom_data")) {
        Some(Tag::Compound(c)) => Some(c.clone()),
        _ => None,
    }
}

pub(crate) fn set_custom_data(stack: &mut ItemStack, data: Compound) {
    let mut patch: Compound = stack.components().map(Compound::from).unwrap_or_default();
    patch.insert(java("minecraft:custom_data"), Tag::Compound(data));
    let _ = stack.set_components(patch);
}

pub(crate) fn sneaking(host: &dyn Host, player: PlayerRef) -> bool {
    let entity = player.entity();
    entity.is_shift_key_down(host).unwrap_or(false) || entity.is_crouching(host).unwrap_or(false)
}

/// A slot of a screen at `position` (GUI pixels from the screen's corner), with no empty-slot icon: Sunrise's screens
/// take `ScreenSlot`s since 2d81263.
pub(crate) fn slot_at(position: [f32; 2]) -> ScreenSlot {
    ScreenSlot { position, no_item_icon: None }
}
