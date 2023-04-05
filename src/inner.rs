use bevy::prelude::*;

use self::model::{CardKind, DemonKind};

pub struct CardGamePlugin;

impl Plugin for CardGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CardGameEvent>()
            .add_startup_system(model::setup)
            .add_startup_system(view::setup)
            .add_system(view::hand_card_interaction)
            .add_system(view::end_turn_btn_interaction)
            .add_system(
                view::refresh_from_model
                    .after(view::hand_card_interaction)
                    .after(view::end_turn_btn_interaction),
            );
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
    pub struct ImageHandles {
        end_turn_btn: ButtonImageHandles,
        cards: HashMap<model::CardKind, CardImageHandles>,
        card_back: Handle<Image>,
    }

    #[derive(Clone)]
    struct ButtonImageHandles {
        normal: Handle<Image>,
        hover: Handle<Image>,
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
        card: Card,
        button: ButtonBundle,
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
    pub struct DeckTop;

    #[derive(Component)]
    pub struct HandArea;

    #[derive(Component)]
    pub struct DiscardArea;

    #[derive(Component)]
    pub struct DiscardTop;

    #[derive(Component)]
    pub struct PlayArea;

    #[derive(Component)]
    pub struct HudArea1;

    #[derive(Component)]
    pub struct HudArea2;

    #[derive(Component)]
    pub struct EndTurnBtn;

    pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
        // Load images
        let img_card_back = asset_server.load("images/Card Back.png");
        let img_card_inspired = asset_server.load("images/Inspired.png");
        let img_card_inspired_hover = asset_server.load("images/Card Back.png");
        let img_card_peaceful = asset_server.load("images/Peaceful.png");
        let img_card_peaceful_hover = asset_server.load("images/Card Back.png");
        let img_btn_end_turn = asset_server.load("images/end_turn_btn.png");
        let img_btn_end_turn_hover = asset_server.load("images/end_turn_btn_hover.png");

        // Initialise card image handles
        let mut card_image_handles = HashMap::new();
        // Inspired card
        card_image_handles.insert(
            model::CardKind::Inspired,
            CardImageHandles {
                face_up: img_card_inspired.clone(),
                face_down: img_card_back.clone(),
                hover: img_card_inspired_hover.clone(),
            },
        );
        // Peaceful card
        card_image_handles.insert(
            model::CardKind::Peaceful,
            CardImageHandles {
                face_up: img_card_peaceful.clone(),
                face_down: img_card_back.clone(),
                hover: img_card_peaceful_hover.clone(),
            },
        );

        // Add image handles as resources
        let image_handles = ImageHandles {
            end_turn_btn: ButtonImageHandles {
                normal: img_btn_end_turn.clone(),
                hover: img_btn_end_turn_hover.clone(),
            },
            cards: card_image_handles,
            card_back: img_card_back.clone(),
        };
        commands.insert_resource(image_handles.clone());

        // Load fonts
        let font_handles = FontHandles {
            regular: asset_server.load("fonts/Kenney High.ttf"),
        };
        commands.insert_resource(font_handles.clone());
        // Init UI
        commands.spawn(Camera2dBundle::default());
        setup_ui(&mut commands, &font_handles, &image_handles);
    }

    fn setup_ui(commands: &mut Commands, font_handles: &FontHandles, image_handles: &ImageHandles) {
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
                        size: Size::height(Val::Px(154.0)),
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
                            size: Size::width(Val::Px(138.0)),
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        background_color: Color::rgb(0.3, 0.3, 0.3).into(),
                        ..default()
                    })
                    .with_children(|deck_area| {
                        deck_area
                            .spawn(ImageBundle {
                                style: Style {
                                    size: Size::new(Val::Px(64.0), Val::Px(72.0)),
                                    ..default()
                                },
                                image: UiImage {
                                    texture: image_handles.card_back.clone(),
                                    ..default()
                                },
                                ..default()
                            })
                            .insert(DeckTop);
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
                    .insert(HandArea);
                    // Discard pile area
                    dock.spawn(NodeBundle {
                        style: Style {
                            size: Size::width(Val::Px(138.0)),
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        background_color: Color::rgb(0.3, 0.3, 0.3).into(),
                        ..default()
                    })
                    .insert(DiscardArea)
                    .with_children(|discard_area| {
                        discard_area
                            .spawn(ImageBundle {
                                style: Style {
                                    size: Size::new(Val::Px(64.0), Val::Px(72.0)),
                                    ..default()
                                },
                                image: UiImage {
                                    texture: image_handles.card_back.clone(),
                                    ..default()
                                },
                                visibility: Visibility::Hidden,
                                ..default()
                            })
                            .insert(DiscardTop);
                    });
                });
                // HUD area 1
                root.spawn(NodeBundle {
                    style: Style {
                        size: Size::height(Val::Px(30.0)),
                        ..default()
                    },
                    background_color: Color::rgb(0.3, 0.1, 0.7).into(),
                    ..default()
                })
                .insert(HudArea1);
                // Play area
                root.spawn(NodeBundle {
                    style: Style {
                        size: Size::height(Val::Px(154.0)),
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
                        size: Size::height(Val::Px(60.0)),
                        justify_content: JustifyContent::End,
                        ..default()
                    },
                    background_color: Color::rgb(0.3, 0.1, 0.7).into(),
                    ..default()
                })
                .with_children(|hud2| {
                    hud2.spawn(ButtonBundle {
                        style: Style {
                            size: Size::width(Val::Px(120.0)),
                            margin: UiRect {
                                left: Val::Px(5.0),
                                top: Val::Px(5.0),
                                ..default()
                            },
                            ..default()
                        },
                        image: UiImage {
                            texture: image_handles.end_turn_btn.normal.clone(),
                            ..default()
                        },
                        ..default()
                    })
                    .insert(EndTurnBtn);
                })
                .insert(HudArea2);
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

    fn create_card(card_model: model::Card, image_handles: &ImageHandles) -> CardBundle {
        let kind = card_model.kind;
        let card = Card {
            model: card_model,
            image_handles: image_handles.cards.get(&kind).unwrap().clone(),
        };
        let start_texture = card.image_handles.face_up.clone();
        CardBundle {
            card,
            button: ButtonBundle {
                style: Style {
                    size: Size::new(Val::Px(128.0), Val::Px(144.0)),
                    margin: UiRect {
                        left: Val::Px(5.0),
                        ..default()
                    },
                    ..default()
                },
                image: UiImage {
                    texture: start_texture,
                    ..default()
                },
                ..default()
            },
        }
    }

    pub fn refresh_from_model(
        mut commands: Commands,
        mut q_hand_area: Query<Entity, With<HandArea>>,
        mut q_play_area: Query<Entity, With<PlayArea>>,
        mut q_deck_top_visibility: Query<&mut Visibility, (With<DeckTop>, Without<DiscardTop>)>,
        mut q_disc_top: Query<
            (&mut Visibility, &mut UiImage),
            (Without<DeckTop>, With<DiscardTop>),
        >,
        q_cards: Query<(Entity, &Card)>,
        image_handles: Res<ImageHandles>,
        game_model: Res<model::CardGameModel>,
    ) {
        // Catalog the card objects that already exist in the scene
        // to avoid despawning and creating them every frame
        let mut visible_cards: HashMap<u32, Entity> = HashMap::new();
        for (entity, card) in q_cards.iter() {
            visible_cards.insert(card.model.id, entity);
        }

        // Clear all card areas of their children
        commands.entity(q_hand_area.single_mut()).clear_children();
        commands.entity(q_play_area.single_mut()).clear_children();

        {
            // Update the hand
            let mut hand_area = commands.entity(q_hand_area.single_mut());
            for card in game_model.hand.iter() {
                match visible_cards.get(&card.id) {
                    Some(entity) => {
                        // Set card's parent as the hand area
                        hand_area.push_children(&[*entity]);
                    }
                    None => {
                        // Spawn a new card object
                        hand_area.with_children(|parent| {
                            let new_card_obj = create_card(card.clone(), &image_handles);
                            parent.spawn(new_card_obj);
                        });
                    }
                }
            }
        }
        {
            // Update the play area
            let mut play_area = commands.entity(q_play_area.single_mut());
            for card in game_model.in_play.iter() {
                match visible_cards.get(&card.id) {
                    Some(entity) => {
                        // Set card's parent as the hand area
                        play_area.push_children(&[*entity]);
                    }
                    None => {
                        // Spawn a new card object
                        play_area.with_children(|parent| {
                            let new_card_obj = create_card(card.clone(), &image_handles);
                            parent.spawn(new_card_obj);
                        });
                    }
                }
            }
        }

        // Update the deck
        let mut deck_top_visibility = q_deck_top_visibility.single_mut();
        if game_model.deck.is_empty() {
            *deck_top_visibility = Visibility::Hidden;
        } else {
            *deck_top_visibility = Visibility::Visible;
        }

        // Update the discard pile
        let (mut disc_top_visibility, mut disc_top_image) = q_disc_top.single_mut();
        if game_model.discard_pile.is_empty() {
            *disc_top_visibility = Visibility::Hidden;
        } else {
            *disc_top_visibility = Visibility::Visible;
            // Set the discard top's image to the card on top of the discard pile
            let top_card_kind = game_model.discard_pile.last().unwrap().kind;
            disc_top_image.texture = image_handles
                .cards
                .get(&top_card_kind)
                .unwrap()
                .face_up
                .clone();
        }

        {
            // Clean up cards in either the deck or discard pile
            for (card_entity, card) in q_cards.iter() {
                if game_model
                    .deck
                    .iter()
                    .find(|c| c.id == card.model.id)
                    .is_some()
                    || game_model
                        .discard_pile
                        .iter()
                        .find(|c| c.id == card.model.id)
                        .is_some()
                {
                    commands.entity(card_entity).despawn_recursive();
                }
            }
        }
    }

    pub fn hand_card_interaction(
        mut q_interaction: Query<
            (Entity, &Interaction, &mut UiImage, &'static Card),
            (Changed<Interaction>, With<Button>),
        >,
        mut game_model: ResMut<model::CardGameModel>,
    ) {
        for (e_card, interaction, mut image, card) in &mut q_interaction {
            match *interaction {
                Interaction::Clicked => {
                    game_model.play(card.model.id);
                }
                Interaction::Hovered => image.texture = card.image_handles.hover.clone(),
                Interaction::None => image.texture = card.image_handles.face_up.clone(),
            }
        }
    }

    pub fn end_turn_btn_interaction(
        mut q_interaction: Query<
            (&Interaction, &mut UiImage),
            (Changed<Interaction>, With<Button>, With<EndTurnBtn>),
        >,
        mut game_model: ResMut<model::CardGameModel>,
        image_handles: Res<ImageHandles>,
    ) {
        for (interaction, mut image) in &mut q_interaction {
            match *interaction {
                Interaction::Clicked => {
                    game_model.end_turn();
                }
                Interaction::Hovered => image.texture = image_handles.end_turn_btn.hover.clone(),
                Interaction::None => image.texture = image_handles.end_turn_btn.normal.clone(),
            }
        }
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
        next_card_id: u32,
    }

    impl CardGameModel {
        pub fn new(
            demons: Vec<DemonKind>,
            starter_cards: Vec<CardKind>,
            settings: &Settings,
        ) -> Self {
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
                    .zip((0..))
                    .map(|(kind, id)| Card { id, kind: *kind })
                    .collect(),
                discard_pile: Vec::new(),
                hand: Vec::new(),
                in_play: Vec::new(),
                next_card_id: starter_cards.len() as u32,
            };
            card_game_model.deck.shuffle(&mut thread_rng());
            for _ in 0..5 {
                card_game_model.draw();
            }
            card_game_model
        }

        pub fn start_turn(&mut self) {}

        pub fn draw(&mut self) {
            // If there are no cards to draw, shuffle discard pile into deck
            if self.deck.is_empty() {
                self.deck.append(&mut self.discard_pile);
                self.deck.shuffle(&mut thread_rng());
            }
            let card = self.deck.pop().unwrap();
            self.hand.push(card);
        }

        fn find_card_in_hand(&self, card_id: u32) -> usize {
            // Find the index of the card with the given card_id
            self.hand
                .iter()
                .zip((0..))
                .find(|(c, i)| c.id == card_id)
                .unwrap()
                .1
        }

        pub fn discard(&mut self, card_id: u32) {
            let card_index = self.find_card_in_hand(card_id);
            let card = self.hand.remove(card_index);
            self.discard_pile.push(card);
        }

        pub fn gain(&mut self, kind: CardKind) {
            let card = Card {
                id: self.next_card_id,
                kind,
            };
            self.discard_pile.push(card);
            self.next_card_id += 1;
        }

        pub fn play(&mut self, card_id: u32) {
            let card_index = self.find_card_in_hand(card_id);
            let card = self.hand.remove(card_index);
            self.in_play.push(card);
            // TODO: implement effects of cards
        }

        fn cleanup(&mut self) {
            // All cards in play are discarded
            self.discard_pile.append(&mut self.in_play);
        }

        pub fn end_turn(&mut self) {
            self.cleanup();
            self.demon_attack();
            // Draw up to 5
            for _ in 0..(5 - self.hand.len()) {
                self.draw();
            }
        }

        fn demon_attack(&mut self) {
            for demon in self.demons.iter_mut() {
                if demon.stun_time > 0 {
                    demon.stun_time -= 1;
                } else {
                    self.player_resolve -= demon.power;
                }
            }
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
        pub id: u32,
        pub kind: CardKind,
    }
}
