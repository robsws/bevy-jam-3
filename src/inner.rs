use crate::settings::Settings;
use bevy::prelude::*;

use self::model::{CardGameModel, CardKind, DemonKind};

pub struct CardGamePlugin;

impl Plugin for CardGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CardGameEvent>()
            .add_startup_system(model::setup)
            .add_startup_system(view::setup);
    }
}

pub enum CardGameEvent {
    DrawCard,
    DiscardCard(usize),
    DiscardAll,
    GainCard(CardKind),
    PlayCard(usize),
    ShuffleDiscardToDeck,
    Cleanup,
    DemonAttack(DemonKind, u32),
    DamageResolve(u32),
    ReduceStun(DemonKind),
}

mod view {

    use bevy::prelude::*;

    use super::model;

    #[derive(Resource, Clone)]
    struct CardImageHandles {
        back: Handle<Image>,
        inspired: Handle<Image>,
        peaceful: Handle<Image>,
    }

    #[derive(Resource, Clone)]
    struct FontHandles {
        regular: Handle<Font>,
    }

    #[derive(Component)]
    struct DeckArea;

    #[derive(Component)]
    struct HandArea;

    #[derive(Component)]
    struct DiscardArea;

    #[derive(Component)]
    struct PlayArea;

    #[derive(Bundle)]
    struct CardBundle {
        model: model::Card,
        image: ImageBundle,
    }

    pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
        // Load images
        let card_image_handles = CardImageHandles {
            back: asset_server.load("images/back.png"),
            inspired: asset_server.load("images/inspired.png"),
            peaceful: asset_server.load("images/peaceful.png"),
        };
        commands.insert_resource(card_image_handles.clone());
        // Load fonts
        let font_handles = FontHandles {
            regular: asset_server.load("fonts/Kenney High.ttf"),
        };
        commands.insert_resource(font_handles.clone());
        // Init UI
        commands.spawn(Camera2dBundle::default());
        setup_ui(&mut commands, &card_image_handles, &font_handles);
    }

    fn setup_ui(
        commands: &mut Commands,
        card_image_handles: &CardImageHandles,
        font_handles: &FontHandles,
    ) {
        // Root node of layout
        commands
            .spawn(NodeBundle {
                style: Style {
                    size: Size::width(Val::Percent(100.0)),
                    flex_direction: FlexDirection::ColumnReverse,
                    justify_content: JustifyContent::FlexEnd,
                    ..default()
                },
                ..default()
            })
            .with_children(|root| {
                // Hand, deck and discard pile area
                root.spawn(NodeBundle {
                    style: Style {
                        size: Size::height(Val::Px(260.0)),
                        justify_content: JustifyContent::SpaceBetween,
                        ..default()
                    },
                    background_color: Color::rgb(0.4, 0.4, 0.4).into(),
                    ..default()
                })
                .with_children(|dock| {
                    // Deck area
                    dock.spawn(NodeBundle {
                        style: Style {
                            size: Size::width(Val::Px(150.0)),
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        background_color: Color::rgb(0.3, 0.3, 0.3).into(),
                        ..default()
                    })
                    .insert(DeckArea);
                    // Hand area
                    dock.spawn(NodeBundle {
                        style: Style {
                            size: Size::width(Val::Percent(100.0)),
                            justify_content: JustifyContent::Start,
                            ..default()
                        },
                        background_color: Color::rgb(0.3, 0.3, 0.0).into(),
                        ..default()
                    })
                    .insert(HandArea)
                    .with_children(|hand| {
                        for _ in 0..3 {
                            create_card_in_hand(
                                hand,
                                model::CardKind::Inspired,
                                card_image_handles,
                            );
                        }
                    });
                    // Discard pile area
                    dock.spawn(NodeBundle {
                        style: Style {
                            size: Size::width(Val::Px(150.0)),
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        background_color: Color::rgb(0.3, 0.3, 0.3).into(),
                        ..default()
                    })
                    .insert(DiscardArea);
                });
                // Play area
                root.spawn(NodeBundle {
                    style: Style {
                        size: Size::height(Val::Percent(100.0)),
                        ..default()
                    },
                    background_color: Color::rgb(0.2, 0.2, 0.2).into(),
                    ..default()
                });
            });
    }

    fn create_card_in_hand(
        hand_area: &mut ChildBuilder,
        kind: model::CardKind,
        card_image_handles: &CardImageHandles,
    ) {
        let card = ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(128.0), Val::Px(256.0)),
                margin: UiRect {
                    left: Val::Px(5.0),
                    ..default()
                },
                ..default()
            },
            image: UiImage {
                texture: match kind {
                    model::CardKind::Peaceful => card_image_handles.peaceful.clone(),
                    model::CardKind::Inspired => card_image_handles.inspired.clone(),
                    _ => card_image_handles.back.clone(),
                },
                ..default()
            },
            ..default()
        };
        hand_area.spawn((card, kind));
    }
}

// The card game can be represented entirely by this state object.
// Bevy will manage the actual objects on screen and interactions,
// but this model stores the actual state of the game and performs
// mutations on it.

// The model and the view will communicate via bevy events, which
// will be queued up by the model and then displayed at a sensible
// speed for the player.

mod model {

    use bevy::prelude::*;
    use rand::seq::SliceRandom;
    use rand::thread_rng;

    use crate::settings::Settings;

    #[derive(Resource)]
    pub struct CardGameModel {
        pub demons: Vec<Demon>,
        pub player_resolve: u32,
        pub deck: Vec<Card>,
        pub discard_pile: Vec<Card>,
        pub hand: Vec<Card>,
        pub in_play: Vec<Card>,
    }

    impl CardGameModel {
        pub fn new(
            demons: Vec<DemonKind>,
            starter_cards: Vec<CardKind>,
            settings: &Settings,
        ) -> CardGameModel {
            let mut card_game_model = CardGameModel {
                demons: demons
                    .iter()
                    .map(|kind| Demon {
                        kind: *kind,
                        power: settings.game.inner.starting_demon_power,
                        stun_time: settings.game.inner.starting_demon_stun_time,
                    })
                    .collect(),
                player_resolve: settings.game.inner.starting_resolve,
                deck: starter_cards
                    .iter()
                    .map(|kind| Card { kind: *kind })
                    .collect(),
                discard_pile: Vec::new(),
                hand: Vec::new(),
                in_play: Vec::new(),
            };
            card_game_model.deck.shuffle(&mut thread_rng());
            card_game_model
        }
    }

    pub fn setup(mut commands: Commands, settings: Res<Settings>) {
        let card_game_model = CardGameModel::new(
            vec![DemonKind::Fear, DemonKind::Despair, DemonKind::Doubt],
            vec![
                CardKind::Inspired,
                CardKind::Inspired,
                CardKind::Inspired,
                CardKind::Peaceful,
                CardKind::Peaceful,
                CardKind::Peaceful,
                CardKind::Peaceful,
                CardKind::Peaceful,
                CardKind::Peaceful,
                CardKind::Peaceful,
            ],
            &settings,
        );
        commands.insert_resource(card_game_model);
    }

    #[derive(Copy, Clone, Debug)]
    pub enum DemonKind {
        Fear,
        Despair,
        Doubt,
    }

    #[derive(Component)]
    pub struct Demon {
        pub kind: DemonKind,
        // How much damage the demon does to Resolve at the end of the turn
        pub power: u32,
        // How many turns the demon is stunned for
        pub stun_time: u32,
    }

    #[derive(Component, Copy, Clone, Debug)]
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
        pub kind: CardKind,
    }

    pub fn start_turn() {}

    pub fn draw(mut game: CardGameModel) -> CardGameModel {
        // If there are no cards to draw, shuffle discard pile into deck
        if game.deck.is_empty() {
            game.deck.append(&mut game.discard_pile);
            game.deck.shuffle(&mut thread_rng());
        }
        let card = game.deck.pop().unwrap();
        game.hand.push(card);
        game
    }

    pub fn discard(mut game: CardGameModel, hand_index: usize) -> CardGameModel {
        let card = game.hand.remove(hand_index);
        game.discard_pile.push(card);
        game
    }

    pub fn gain(mut game: CardGameModel, kind: CardKind) -> CardGameModel {
        let card = Card { kind };
        game.discard_pile.push(card);
        game
    }

    pub fn play(hand_index: usize) {}

    fn cleanup(mut game: CardGameModel) -> CardGameModel {
        // All cards in play are discarded
        game.discard_pile.append(&mut game.in_play);
        game
    }

    pub fn end_turn(mut game: CardGameModel) -> CardGameModel {
        // Discard remaining cards in hand
        game.discard_pile.append(&mut game.hand);
        game = cleanup(game);
        game = demon_attack(game);
        game
    }

    fn demon_attack(mut game: CardGameModel) -> CardGameModel {
        for demon in game.demons.iter_mut() {
            if demon.stun_time > 0 {
                demon.stun_time -= 1;
            } else {
                game.player_resolve -= demon.power;
            }
        }
        game
    }
}
