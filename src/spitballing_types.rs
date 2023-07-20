#![allow(dead_code)]

struct Build {
    name: String,
    game_version: String,
    class: Class,
    bandit_reward: Option<Bandit>,
    pantheon_major_god: Option<MajorGod>,
    pantheon_minor_god: Option<MinorGod>,
    target_level: u8,
    passive_tree: PassiveSpec,
    actives: Option<Vec<Skill>>,
    gear: [Option<Eqpmt>; 11],
    sims: Vec<SimConfig>,
    custom: Option<Vec<Mod>>
}

struct Class {
    name: String,
    base_str: u32,
    base_dex: u32,
    base_int: u32,
    origin_nodes: Vec<TreeNode>,
    ascendancies: Vec<Ascendancy>
}

struct Bandit {
    name: String,
    modifiers: Vec<Mod>
}

struct MajorGod {
    name: String,
    modifiers: Vec<Mod>
}
struct MinorGod {
    name: String,
    modifiers: Vec<Mod>    
}

struct PassiveSpec {
    name: String,
    allocated_nodes: Vec<TreeNode>
}

struct Eqpmt {
    name: String,
    rarity: String,
    quality: u8,
    base_type: EqpmtBase,
    ilvl: u8,
    affixes: ([Prefix; 3], [Suffix; 3]),
    implicits: Vec<Modifier>,
    influence: Influence,
    enchantment: Modifier,
    sockets: String,
    lvl_req: u8,
    attr_req: [u32; 3]
}

struct Mod {
    name: String,
    mod_type: String,
    value: ModValue,
    source: ModSource,
    flags: Vec<ModFlag>,
    keywords: Vec<KeywordFlag>,
    extra_tags: Vec<ModTag>
}

enum ModValue {
    
}

fn main() {
    println!("Tala Moana, Exile!");
}