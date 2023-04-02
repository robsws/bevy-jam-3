use bevy::prelude::*;

pub struct InnerGamePlugin;

impl Plugin for InnerGamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(InnerPlayer {
            deck: starting_deck(),
            resolve: start..Default::default(),
        })
    }
}

pub enum InnerPhase {
    PlayCards,
    DemonsAttack,
    Cleanup,
}

#[derive(Resource, Default)]
pub struct InnerPlayer {
    deck: Vec<Card>,
    discard_pile: Vec<Card>,
    hand: Vec<Card>,
    in_play: Vec<Card>,
    resolve: u32,
}

#[derive(Component)]
pub struct Demon {
    // How much damage the demon does to Resolve at the end of the turn
    power: u32,
    // How many turns the demon is stunned for
    stun_time: u32,
}

pub enum CardKind {
    Angry,
    Inspired,
    Tired,
    Stressed,
    Satisfied,
    Proud,
    Determined,
    Peaceful,
    Dizzy,
    Hungover,
}

#[derive(Component)]
pub struct Card {
    kind: CardKind,
}
