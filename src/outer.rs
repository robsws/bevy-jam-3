pub struct OuterGame {
    // Information about the player
    player_position: Coord,
    player_health: u32,
    status_effects: Vec<StatusEffect>,
    inventory: Vec<Item>,
    // Enemies
    enemies: Vec<Enemy>,
    // Items
    items: Vec<Item>,
}

pub struct Coord {
    x: u32,
    y: u32,
}

pub enum StatusEffect {
    Encumbered,
    Scared,
    Hopeless,
    Doubtful,
    Pumped,
}

pub enum EnemyKind {
    Goblin,
}

pub struct Enemy {
    position: Coord,
    kind: EnemyKind,
}

pub enum ItemKind {
    Food,
    Potion,
    Beer,
    Sword,
    Shield,
}

pub struct Item {
    position: Coord,
    kind: ItemKind,
}
