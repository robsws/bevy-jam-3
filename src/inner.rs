use bevy::prelude::*;

use self::model::{CardKind, DemonKind};

pub struct CardGamePlugin;

impl Plugin for CardGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CardGameEvent>()
            .add_startup_system(model::setup)
            .add_startup_system(view::setup)
            .add_system(view::hand_card_interaction);
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

    use std::collections::HashMap;

    use bevy::prelude::*;

    use super::model;

    #[derive(Resource, Clone)]
    pub struct CardPrefabs(HashMap<model::CardKind, Card>);

    impl CardPrefabs {
        pub fn new() -> CardPrefabs {
            CardPrefabs(HashMap::new())
        }
    }

    #[derive(Resource, Clone)]
    struct FontHandles {
        regular: Handle<Font>,
    }

    #[derive(Component, Clone)]
    pub struct Card {
        model: model::Card,
        image_handles: CardImageHandles,
    }

    #[derive(Bundle)]
    struct CardBundle {
        prefab: Card,
        view: ButtonBundle,
    }

    #[derive(Clone)]
    struct CardImageHandles {
        face_up: Handle<Image>,
        face_down: Handle<Image>,
        hover: Handle<Image>,
    }

    #[derive(Component)]
    pub struct DeckArea;

    #[derive(Component)]
    pub struct HandArea;

    #[derive(Component)]
    pub struct DiscardArea;

    #[derive(Component)]
    pub struct PlayArea;

    pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
        // Load images
        let img_card_back = asset_server.load("images/back.png");
        let img_card_inspired = asset_server.load("images/inspired.png");
        let img_card_inspired_hover = asset_server.load("images/inspired_hover.png");
        let img_card_peaceful = asset_server.load("images/peaceful.png");
        let img_card_peaceful_hover = asset_server.load("images/peaceful_hover.png");

        // Initialise card prefabs
        let mut card_prefabs = CardPrefabs::new();
        // Inspired card
        card_prefabs.0.insert(
            model::CardKind::Inspired,
            Card {
                model: model::Card {
                    kind: model::CardKind::Inspired,
                },
                image_handles: CardImageHandles {
                    face_up: img_card_inspired.clone(),
                    face_down: img_card_back.clone(),
                    hover: img_card_inspired_hover.clone(),
                },
            },
        );
        // Peaceful card
        card_prefabs.0.insert(
            model::CardKind::Peaceful,
            Card {
                model: model::Card {
                    kind: model::CardKind::Peaceful,
                },
                image_handles: CardImageHandles {
                    face_up: img_card_peaceful.clone(),
                    face_down: img_card_back.clone(),
                    hover: img_card_peaceful_hover.clone(),
                },
            },
        );
        commands.insert_resource(card_prefabs.clone());
        // Load fonts
        let font_handles = FontHandles {
            regular: asset_server.load("fonts/Kenney High.ttf"),
        };
        commands.insert_resource(font_handles.clone());
        // Init UI
        commands.spawn(Camera2dBundle::default());
        setup_ui(&mut commands, &card_prefabs, &font_handles);
    }

    fn setup_ui(commands: &mut Commands, card_prefabs: &CardPrefabs, font_handles: &FontHandles) {
        // Root node of layout
        commands
            .spawn(NodeBundle {
                style: Style {
                    size: Size::width(Val::Percent(100.0)),
                    flex_direction: FlexDirection::ColumnReverse,
                    justify_content: JustifyContent::End,
                    ..default()
                },
                ..default()
            })
            .with_children(|root| {
                // Hand, deck and discard pile area
                root.spawn(NodeBundle {
                    style: Style {
                        size: Size::height(Val::Px(138.0)),
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
                            size: Size::width(Val::Px(100.0)),
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
                                card_prefabs.0.get(&model::CardKind::Inspired).unwrap(),
                            );
                        }
                    });
                    // Discard pile area
                    dock.spawn(NodeBundle {
                        style: Style {
                            size: Size::width(Val::Px(100.0)),
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        background_color: Color::rgb(0.3, 0.3, 0.3).into(),
                        ..default()
                    })
                    .insert(DiscardArea);
                });
                // HUD area 1
                root.spawn(NodeBundle {
                    style: Style {
                        size: Size::height(Val::Px(30.0)),
                        ..default()
                    },
                    background_color: Color::rgb(0.3, 0.1, 0.7).into(),
                    ..default()
                });
                // Play area
                root.spawn(NodeBundle {
                    style: Style {
                        size: Size::height(Val::Px(138.0)),
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    background_color: Color::rgb(0.1, 0.1, 0.1).into(),
                    ..default()
                })
                .insert(PlayArea);
                // HUD area 2
                root.spawn(NodeBundle {
                    style: Style {
                        size: Size::height(Val::Px(30.0)),
                        ..default()
                    },
                    background_color: Color::rgb(0.3, 0.1, 0.7).into(),
                    ..default()
                });
                // Demon area
                root.spawn(NodeBundle {
                    style: Style {
                        size: Size::height(Val::Px(400.0)),
                        ..default()
                    },
                    background_color: Color::rgb(0.8, 0.8, 0.8).into(),
                    ..default()
                });
            });
    }

    fn create_card_in_hand(hand_area: &mut ChildBuilder, prefab: &Card) {
        let card = CardBundle {
            prefab: prefab.clone(),
            view: ButtonBundle {
                style: Style {
                    size: Size::new(Val::Px(64.0), Val::Px(128.0)),
                    margin: UiRect {
                        left: Val::Px(5.0),
                        ..default()
                    },
                    ..default()
                },
                image: UiImage {
                    texture: prefab.image_handles.face_up.clone(),
                    ..default()
                },
                ..default()
            },
        };
        hand_area.spawn(card);
    }

    pub fn hand_card_interaction(
        mut commands: Commands,
        mut q_interaction: Query<
            (Entity, &Interaction, &mut UiImage, &Card),
            (Changed<Interaction>, With<Button>),
        >,
        mut q_play_area: Query<Entity, With<PlayArea>>,
        mut q_hand_area: Query<Entity, With<HandArea>>,
    ) {
        let mut e_play_area = q_play_area.single_mut();
        let mut e_hand_area = q_hand_area.single_mut();
        for (e_card, interaction, mut image, card) in &mut q_interaction {
            match *interaction {
                Interaction::Clicked => {
                    // Reparent the card to PlayArea
                    reparent(&e_card, &e_hand_area, &e_play_area, &mut commands);
                    // Do the effect of the card
                }
                Interaction::Hovered => image.texture = card.image_handles.hover.clone(),
                Interaction::None => image.texture = card.image_handles.face_up.clone(),
            }
        }
    }

    fn reparent(child: &Entity, from_parent: &Entity, to_parent: &Entity, commands: &mut Commands) {
        commands.entity(*from_parent).remove_children(&[*child]);
        commands.entity(*to_parent).push_children(&[*child]);
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
        pub player_defense: u32,
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
                player_defense: 0,
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

    pub struct Demon {
        pub kind: DemonKind,
        // How much damage the demon does to Resolve at the end of the turn
        pub power: u32,
        // How many turns the demon is stunned for
        pub stun_time: u32,
    }

    #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
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

    #[derive(Clone)]
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
