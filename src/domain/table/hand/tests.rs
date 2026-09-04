use super::*;
fn hand(count: u8) -> Hand {
    Hand::new(
        Deck::new(std::array::from_fn(|i| Card::from_index(i as u8))).unwrap(),
        Blinds {
            small: Chips(50),
            big: Chips(100),
        },
        (0..count)
            .map(|i| ParticipantInfo {
                seat_no: SeatNo(i),
                stack: Chips(1000),
            })
            .collect(),
    )
}
#[test]
fn starts_three_handed() {
    let mut h = hand(3);
    assert_eq!(
        h.start().last(),
        Some(&HandEvent::ActionRequested {
            seat_no: SeatNo(0),
            to_call: Chips(100),
            min_raise_to: Chips(200)
        })
    );
}

#[test]
fn four_handed_hand_is_recorded_from_blinds_through_showdown() {
    let mut hand = hand(4);
    let mut events = hand.start();

    for (seat_no, action) in [
        (3, Action::Call),
        (0, Action::Call),
        (1, Action::Call),
        (2, Action::Check),
        (1, Action::Check),
        (2, Action::Bet(Chips(100))),
        (3, Action::Fold),
        (0, Action::Call),
        (1, Action::Fold),
        (2, Action::Check),
        (0, Action::Check),
        (2, Action::Check),
        (0, Action::Check),
    ] {
        events.extend(hand.act(SeatNo(seat_no), action).unwrap());
    }

    assert_eq!(
        events,
        vec![
            HandEvent::BlindPosted {
                seat_no: SeatNo(1),
                blind: Blind::Small,
            },
            HandEvent::ChipsCommitted {
                seat_no: SeatNo(1),
                amount: Chips(50),
                current_bet: Chips(50),
                remaining_stack: Chips(950),
            },
            HandEvent::BlindPosted {
                seat_no: SeatNo(2),
                blind: Blind::Big,
            },
            HandEvent::ChipsCommitted {
                seat_no: SeatNo(2),
                amount: Chips(100),
                current_bet: Chips(100),
                remaining_stack: Chips(900),
            },
            HandEvent::HoleCardsDealt {
                seat_nos: vec![SeatNo(1), SeatNo(2), SeatNo(3), SeatNo(0)],
            },
            HandEvent::ActionRequested {
                seat_no: SeatNo(3),
                to_call: Chips(100),
                min_raise_to: Chips(200),
            },
            HandEvent::PlayerActed {
                seat_no: SeatNo(3),
                action: Action::Call,
            },
            HandEvent::ChipsCommitted {
                seat_no: SeatNo(3),
                amount: Chips(100),
                current_bet: Chips(100),
                remaining_stack: Chips(900),
            },
            HandEvent::ActionRequested {
                seat_no: SeatNo(0),
                to_call: Chips(100),
                min_raise_to: Chips(200),
            },
            HandEvent::PlayerActed {
                seat_no: SeatNo(0),
                action: Action::Call,
            },
            HandEvent::ChipsCommitted {
                seat_no: SeatNo(0),
                amount: Chips(100),
                current_bet: Chips(100),
                remaining_stack: Chips(900),
            },
            HandEvent::ActionRequested {
                seat_no: SeatNo(1),
                to_call: Chips(50),
                min_raise_to: Chips(200),
            },
            HandEvent::PlayerActed {
                seat_no: SeatNo(1),
                action: Action::Call,
            },
            HandEvent::ChipsCommitted {
                seat_no: SeatNo(1),
                amount: Chips(50),
                current_bet: Chips(100),
                remaining_stack: Chips(900),
            },
            HandEvent::ActionRequested {
                seat_no: SeatNo(2),
                to_call: Chips(0),
                min_raise_to: Chips(200),
            },
            HandEvent::PlayerActed {
                seat_no: SeatNo(2),
                action: Action::Check,
            },
            HandEvent::BettingRoundCompleted {
                street: Street::Preflop,
                pots: vec![Pot {
                    amount: Chips(400),
                    eligible_seats: vec![SeatNo(0), SeatNo(1), SeatNo(2), SeatNo(3)],
                }],
            },
            HandEvent::CommunityCardsDealt {
                street: Street::Flop,
                cards: vec![
                    Card::from_index(9),
                    Card::from_index(10),
                    Card::from_index(11)
                ],
            },
            HandEvent::ActionRequested {
                seat_no: SeatNo(1),
                to_call: Chips(0),
                min_raise_to: Chips(100),
            },
            HandEvent::PlayerActed {
                seat_no: SeatNo(1),
                action: Action::Check,
            },
            HandEvent::ActionRequested {
                seat_no: SeatNo(2),
                to_call: Chips(0),
                min_raise_to: Chips(100),
            },
            HandEvent::PlayerActed {
                seat_no: SeatNo(2),
                action: Action::Bet(Chips(100)),
            },
            HandEvent::ChipsCommitted {
                seat_no: SeatNo(2),
                amount: Chips(100),
                current_bet: Chips(100),
                remaining_stack: Chips(800),
            },
            HandEvent::ActionRequested {
                seat_no: SeatNo(3),
                to_call: Chips(100),
                min_raise_to: Chips(200),
            },
            HandEvent::PlayerActed {
                seat_no: SeatNo(3),
                action: Action::Fold,
            },
            HandEvent::ActionRequested {
                seat_no: SeatNo(0),
                to_call: Chips(100),
                min_raise_to: Chips(200),
            },
            HandEvent::PlayerActed {
                seat_no: SeatNo(0),
                action: Action::Call,
            },
            HandEvent::ChipsCommitted {
                seat_no: SeatNo(0),
                amount: Chips(100),
                current_bet: Chips(100),
                remaining_stack: Chips(800),
            },
            HandEvent::ActionRequested {
                seat_no: SeatNo(1),
                to_call: Chips(100),
                min_raise_to: Chips(200),
            },
            HandEvent::PlayerActed {
                seat_no: SeatNo(1),
                action: Action::Fold,
            },
            HandEvent::BettingRoundCompleted {
                street: Street::Flop,
                pots: vec![Pot {
                    amount: Chips(600),
                    eligible_seats: vec![SeatNo(0), SeatNo(2)],
                }],
            },
            HandEvent::CommunityCardsDealt {
                street: Street::Turn,
                cards: vec![Card::from_index(13)],
            },
            HandEvent::ActionRequested {
                seat_no: SeatNo(2),
                to_call: Chips(0),
                min_raise_to: Chips(100),
            },
            HandEvent::PlayerActed {
                seat_no: SeatNo(2),
                action: Action::Check,
            },
            HandEvent::ActionRequested {
                seat_no: SeatNo(0),
                to_call: Chips(0),
                min_raise_to: Chips(100),
            },
            HandEvent::PlayerActed {
                seat_no: SeatNo(0),
                action: Action::Check,
            },
            HandEvent::BettingRoundCompleted {
                street: Street::Turn,
                pots: vec![Pot {
                    amount: Chips(600),
                    eligible_seats: vec![SeatNo(0), SeatNo(2)],
                }],
            },
            HandEvent::CommunityCardsDealt {
                street: Street::River,
                cards: vec![Card::from_index(15)],
            },
            HandEvent::ActionRequested {
                seat_no: SeatNo(2),
                to_call: Chips(0),
                min_raise_to: Chips(100),
            },
            HandEvent::PlayerActed {
                seat_no: SeatNo(2),
                action: Action::Check,
            },
            HandEvent::ActionRequested {
                seat_no: SeatNo(0),
                to_call: Chips(0),
                min_raise_to: Chips(100),
            },
            HandEvent::PlayerActed {
                seat_no: SeatNo(0),
                action: Action::Check,
            },
            HandEvent::BettingRoundCompleted {
                street: Street::River,
                pots: vec![Pot {
                    amount: Chips(600),
                    eligible_seats: vec![SeatNo(0), SeatNo(2)],
                }],
            },
            HandEvent::ShowdownStarted {
                seat_nos: vec![SeatNo(0), SeatNo(2)],
            },
            HandEvent::HoleCardsShown {
                seat_no: SeatNo(0),
                cards: [Card::from_index(3), Card::from_index(7)],
                hand: EvaluatedHand {
                    category: HandCategory::Flush,
                    best_five: [
                        Card::from_index(9),
                        Card::from_index(10),
                        Card::from_index(11),
                        Card::from_index(3),
                        Card::from_index(7),
                    ],
                },
            },
            HandEvent::HoleCardsShown {
                seat_no: SeatNo(2),
                cards: [Card::from_index(1), Card::from_index(5)],
                hand: EvaluatedHand {
                    category: HandCategory::Flush,
                    best_five: [
                        Card::from_index(9),
                        Card::from_index(10),
                        Card::from_index(11),
                        Card::from_index(1),
                        Card::from_index(5),
                    ],
                },
            },
            HandEvent::PotAwarded {
                amount: Chips(600),
                eligible_seats: vec![SeatNo(0), SeatNo(2)],
                awards: vec![PotAward {
                    seat_no: SeatNo(0),
                    amount: Chips(600),
                }],
            },
            HandEvent::HandFinished,
        ]
    );
}
#[test]
fn fold_ends_heads_up_hand() {
    let mut h = hand(2);
    h.start();
    let events = h.act(SeatNo(0), Action::Fold).unwrap();
    assert!(events.contains(&HandEvent::ChipsReturned {
        seat_no: SeatNo(1),
        amount: Chips(50),
        remaining_stack: Chips(950),
    }));
    assert!(events.contains(&HandEvent::BettingRoundCompleted {
        street: Street::Preflop,
        pots: vec![Pot {
            amount: Chips(100),
            eligible_seats: vec![SeatNo(1)],
        }],
    }));
    assert!(events.contains(&HandEvent::PotAwarded {
        amount: Chips(100),
        eligible_seats: vec![SeatNo(1)],
        awards: vec![PotAward {
            seat_no: SeatNo(1),
            amount: Chips(100),
        }],
    }));
}

#[test]
fn returns_uncalled_chips_and_completes_only_the_actual_betting_round() {
    let mut hand = Hand::new(
        Deck::new(std::array::from_fn(|index| Card::from_index(index as u8))).unwrap(),
        Blinds {
            small: Chips(50),
            big: Chips(100),
        },
        vec![
            ParticipantInfo {
                seat_no: SeatNo(0),
                stack: Chips(1000),
            },
            ParticipantInfo {
                seat_no: SeatNo(1),
                stack: Chips(1000),
            },
            ParticipantInfo {
                seat_no: SeatNo(2),
                stack: Chips(500),
            },
        ],
    );
    hand.start();
    hand.act(SeatNo(0), Action::RaiseTo(Chips(1000))).unwrap();
    hand.act(SeatNo(1), Action::Fold).unwrap();

    let events = hand.act(SeatNo(2), Action::Call).unwrap();

    let returned = events
        .iter()
        .position(|event| matches!(event, HandEvent::ChipsReturned { .. }))
        .unwrap();
    let completed = events
        .iter()
        .position(|event| matches!(event, HandEvent::BettingRoundCompleted { .. }))
        .unwrap();
    let flop = events
        .iter()
        .position(|event| {
            matches!(
                event,
                HandEvent::CommunityCardsDealt {
                    street: Street::Flop,
                    ..
                }
            )
        })
        .unwrap();
    assert!(returned < completed && completed < flop);
    assert_eq!(
        events[returned],
        HandEvent::ChipsReturned {
            seat_no: SeatNo(0),
            amount: Chips(500),
            remaining_stack: Chips(500),
        }
    );
    assert_eq!(
        events[completed],
        HandEvent::BettingRoundCompleted {
            street: Street::Preflop,
            pots: vec![Pot {
                amount: Chips(1050),
                eligible_seats: vec![SeatNo(0), SeatNo(2)],
            }],
        }
    );
    assert_eq!(
        events
            .iter()
            .filter(|event| matches!(event, HandEvent::BettingRoundCompleted { .. }))
            .count(),
        1
    );
}

#[test]
fn start_finishes_hand_when_both_participants_are_all_in_from_blinds() {
    let mut hand = Hand::new(
        Deck::new(std::array::from_fn(|index| Card::from_index(index as u8))).unwrap(),
        Blinds {
            small: Chips(50),
            big: Chips(100),
        },
        vec![
            ParticipantInfo {
                seat_no: SeatNo(0),
                stack: Chips(50),
            },
            ParticipantInfo {
                seat_no: SeatNo(1),
                stack: Chips(50),
            },
        ],
    );

    let events = hand.start();

    assert_eq!(
        &events[..5],
        &[
            HandEvent::BlindPosted {
                seat_no: SeatNo(0),
                blind: Blind::Small,
            },
            HandEvent::ChipsCommitted {
                seat_no: SeatNo(0),
                amount: Chips(50),
                current_bet: Chips(50),
                remaining_stack: Chips(0),
            },
            HandEvent::BlindPosted {
                seat_no: SeatNo(1),
                blind: Blind::Big,
            },
            HandEvent::ChipsCommitted {
                seat_no: SeatNo(1),
                amount: Chips(50),
                current_bet: Chips(50),
                remaining_stack: Chips(0),
            },
            HandEvent::HoleCardsDealt {
                seat_nos: vec![SeatNo(1), SeatNo(0)],
            },
        ]
    );
    assert!(events.iter().any(|event| matches!(
        event,
        HandEvent::CommunityCardsDealt {
            street: Street::Flop,
            ..
        }
    )));
    assert!(events.iter().any(|event| matches!(
        event,
        HandEvent::CommunityCardsDealt {
            street: Street::Turn,
            ..
        }
    )));
    assert!(events.iter().any(|event| matches!(
        event,
        HandEvent::CommunityCardsDealt {
            street: Street::River,
            ..
        }
    )));
    assert!(events.contains(&HandEvent::ShowdownStarted {
        seat_nos: vec![SeatNo(0), SeatNo(1)],
    }));
    assert!(
        !events
            .iter()
            .any(|event| matches!(event, HandEvent::ActionRequested { .. }))
    );
    assert_eq!(events.last(), Some(&HandEvent::HandFinished));
    assert!(hand.is_finished());
    assert_eq!(
        hand.stacks().iter().map(|(_, stack)| stack.0).sum::<u64>(),
        100
    );
}

#[test]
fn five_handed_hand_with_two_flop_folds_reaches_three_way_showdown() {
    let mut hand = hand(5);
    let mut events = hand.start();

    // Preflop: UTG calls, the cutoff raises, and all five players see the flop.
    for (seat_no, action) in [
        (3, Action::Call),
        (4, Action::RaiseTo(Chips(200))),
        (0, Action::Call),
        (1, Action::Call),
        (2, Action::Call),
        (3, Action::Call),
    ] {
        events.extend(hand.act(SeatNo(seat_no), action).unwrap());
    }
    assert!(events.contains(&HandEvent::CommunityCardsDealt {
        street: Street::Flop,
        cards: vec![
            Card::from_index(11),
            Card::from_index(12),
            Card::from_index(13)
        ],
    }));

    // The big blind leads, one player folds, the button raises, and a second
    // player folds. Seats 0, 2, and 3 continue to the turn.
    for (seat_no, action) in [
        (1, Action::Check),
        (2, Action::Bet(Chips(100))),
        (3, Action::Call),
        (4, Action::Fold),
        (0, Action::RaiseTo(Chips(300))),
        (1, Action::Fold),
        (2, Action::Call),
        (3, Action::Call),
    ] {
        events.extend(hand.act(SeatNo(seat_no), action).unwrap());
    }
    assert!(events.contains(&HandEvent::CommunityCardsDealt {
        street: Street::Turn,
        cards: vec![Card::from_index(15)],
    }));

    // The remaining three players check the turn and river to showdown.
    for seat_no in [2, 3, 0, 2, 3, 0] {
        events.extend(hand.act(SeatNo(seat_no), Action::Check).unwrap());
    }

    assert!(events.contains(&HandEvent::CommunityCardsDealt {
        street: Street::River,
        cards: vec![Card::from_index(17)],
    }));
    assert!(events.contains(&HandEvent::ShowdownStarted {
        seat_nos: vec![SeatNo(0), SeatNo(2), SeatNo(3)],
    }));
    let seat_zero_winnings = events
        .iter()
        .filter_map(|event| match event {
            HandEvent::PotAwarded { awards, .. } => awards
                .iter()
                .find(|award| award.seat_no == SeatNo(0))
                .map(|award| award.amount.0),
            _ => None,
        })
        .sum::<u64>();
    assert_eq!(seat_zero_winnings, 1900);
    let awarded_pots: Vec<_> = events
        .iter()
        .filter_map(|event| match event {
            HandEvent::PotAwarded { amount, awards, .. } => Some((amount, awards)),
            _ => None,
        })
        .collect();
    let final_pot_count = events
        .iter()
        .rev()
        .find_map(|event| match event {
            HandEvent::BettingRoundCompleted { pots, .. } => Some(pots.len()),
            _ => None,
        })
        .unwrap();
    assert_eq!(awarded_pots.len(), final_pot_count);
    assert!(awarded_pots.iter().all(|(amount, awards)| {
        awards.iter().map(|award| award.amount.0).sum::<u64>() == amount.0
    }));
    assert_eq!(events.last(), Some(&HandEvent::HandFinished));
    assert!(hand.is_finished());
    assert_eq!(
        hand.stacks(),
        vec![
            (SeatNo(0), Chips(2400)),
            (SeatNo(1), Chips(800)),
            (SeatNo(2), Chips(500)),
            (SeatNo(3), Chips(500)),
            (SeatNo(4), Chips(800)),
        ]
    );
}

#[test]
fn pocket_kings_crack_pocket_aces_after_both_players_are_all_in_preflop() {
    // Heads-up cards are dealt to seat 1 first. The first four cards therefore
    // give kings to seat 1 and aces to seat 0. Card 37 puts a third king on
    // the flop; the mixed-suit board cannot improve the aces past three kings.
    // Positions 4, 8, and 10 are burn cards.
    let prefix = [11, 12, 24, 25, 0, 37, 14, 28, 1, 43, 2, 19];
    let mut card_values = prefix.to_vec();
    card_values.extend((0..52).filter(|value| !prefix.contains(value)));
    let deck = Deck::new(std::array::from_fn(|index| {
        Card::from_index(card_values[index])
    }))
    .unwrap();
    let mut hand = Hand::new(
        deck,
        Blinds {
            small: Chips(50),
            big: Chips(100),
        },
        vec![
            ParticipantInfo {
                seat_no: SeatNo(0),
                stack: Chips(1000),
            },
            ParticipantInfo {
                seat_no: SeatNo(1),
                stack: Chips(1000),
            },
        ],
    );

    hand.start();
    assert_eq!(
        hand.participants[0].cards,
        vec![Card::from_index(12), Card::from_index(25)]
    );
    assert_eq!(
        hand.participants[1].cards,
        vec![Card::from_index(11), Card::from_index(24)]
    );

    hand.act(SeatNo(0), Action::RaiseTo(Chips(1000))).unwrap();
    let events = hand.act(SeatNo(1), Action::Call).unwrap();

    assert!(events.contains(&HandEvent::CommunityCardsDealt {
        street: Street::Flop,
        cards: vec![
            Card::from_index(37),
            Card::from_index(14),
            Card::from_index(28)
        ],
    }));
    assert!(events.contains(&HandEvent::ShowdownStarted {
        seat_nos: vec![SeatNo(0), SeatNo(1)],
    }));
    assert!(
        events.contains(&HandEvent::PotAwarded {
            amount: Chips(2000),
            eligible_seats: vec![SeatNo(0), SeatNo(1)],
            awards: vec![PotAward {
                seat_no: SeatNo(1),
                amount: Chips(2000),
            }],
        }),
        "unexpected settlement events: {events:?}"
    );
    assert_eq!(events.last(), Some(&HandEvent::HandFinished));
    assert_eq!(
        hand.stacks(),
        vec![(SeatNo(0), Chips(0)), (SeatNo(1), Chips(2000))]
    );
}

#[test]
fn big_blind_straight_beats_small_blind_three_of_a_kind_after_preflop_all_in() {
    // Heads-up cards are dealt to the big blind (seat 1) first. Seat 0 gets
    // pocket deuces and finds a third deuce on the flop, while seat 1's five
    // and six complete a nine-high straight on the board. Positions 4, 8,
    // and 10 are burn cards.
    let prefix = [3, 0, 17, 13, 1, 26, 5, 19, 2, 33, 4, 50];
    let mut card_values = prefix.to_vec();
    card_values.extend((0..52).filter(|value| !prefix.contains(value)));
    let deck = Deck::new(std::array::from_fn(|index| {
        Card::from_index(card_values[index])
    }))
    .unwrap();
    let mut hand = Hand::new(
        deck,
        Blinds {
            small: Chips(50),
            big: Chips(100),
        },
        vec![
            ParticipantInfo {
                seat_no: SeatNo(0),
                stack: Chips(1000),
            },
            ParticipantInfo {
                seat_no: SeatNo(1),
                stack: Chips(1000),
            },
        ],
    );

    let mut events = hand.start();
    events.extend(hand.act(SeatNo(0), Action::RaiseTo(Chips(1000))).unwrap());
    events.extend(hand.act(SeatNo(1), Action::Call).unwrap());

    assert_eq!(
        events,
        vec![
            HandEvent::BlindPosted {
                seat_no: SeatNo(0),
                blind: Blind::Small,
            },
            HandEvent::ChipsCommitted {
                seat_no: SeatNo(0),
                amount: Chips(50),
                current_bet: Chips(50),
                remaining_stack: Chips(950),
            },
            HandEvent::BlindPosted {
                seat_no: SeatNo(1),
                blind: Blind::Big,
            },
            HandEvent::ChipsCommitted {
                seat_no: SeatNo(1),
                amount: Chips(100),
                current_bet: Chips(100),
                remaining_stack: Chips(900),
            },
            HandEvent::HoleCardsDealt {
                seat_nos: vec![SeatNo(1), SeatNo(0)],
            },
            HandEvent::ActionRequested {
                seat_no: SeatNo(0),
                to_call: Chips(50),
                min_raise_to: Chips(200),
            },
            HandEvent::PlayerActed {
                seat_no: SeatNo(0),
                action: Action::RaiseTo(Chips(1000)),
            },
            HandEvent::ChipsCommitted {
                seat_no: SeatNo(0),
                amount: Chips(950),
                current_bet: Chips(1000),
                remaining_stack: Chips(0),
            },
            HandEvent::ActionRequested {
                seat_no: SeatNo(1),
                to_call: Chips(900),
                min_raise_to: Chips(1900),
            },
            HandEvent::PlayerActed {
                seat_no: SeatNo(1),
                action: Action::Call,
            },
            HandEvent::ChipsCommitted {
                seat_no: SeatNo(1),
                amount: Chips(900),
                current_bet: Chips(1000),
                remaining_stack: Chips(0),
            },
            HandEvent::BettingRoundCompleted {
                street: Street::Preflop,
                pots: vec![Pot {
                    amount: Chips(2000),
                    eligible_seats: vec![SeatNo(0), SeatNo(1)],
                }],
            },
            HandEvent::CommunityCardsDealt {
                street: Street::Flop,
                cards: vec![
                    Card::from_index(26),
                    Card::from_index(5),
                    Card::from_index(19)
                ],
            },
            HandEvent::CommunityCardsDealt {
                street: Street::Turn,
                cards: vec![Card::from_index(33)],
            },
            HandEvent::CommunityCardsDealt {
                street: Street::River,
                cards: vec![Card::from_index(50)],
            },
            HandEvent::ShowdownStarted {
                seat_nos: vec![SeatNo(0), SeatNo(1)],
            },
            HandEvent::HoleCardsShown {
                seat_no: SeatNo(0),
                cards: [Card::from_index(0), Card::from_index(13)],
                hand: EvaluatedHand {
                    category: HandCategory::ThreeOfAKind,
                    best_five: [
                        Card::from_index(26),
                        Card::from_index(33),
                        Card::from_index(50),
                        Card::from_index(0),
                        Card::from_index(13),
                    ],
                },
            },
            HandEvent::HoleCardsShown {
                seat_no: SeatNo(1),
                cards: [Card::from_index(3), Card::from_index(17)],
                hand: EvaluatedHand {
                    category: HandCategory::Straight,
                    best_five: [
                        Card::from_index(5),
                        Card::from_index(19),
                        Card::from_index(33),
                        Card::from_index(3),
                        Card::from_index(17),
                    ],
                },
            },
            HandEvent::PotAwarded {
                amount: Chips(2000),
                eligible_seats: vec![SeatNo(0), SeatNo(1)],
                awards: vec![PotAward {
                    seat_no: SeatNo(1),
                    amount: Chips(2000),
                }],
            },
            HandEvent::HandFinished,
        ]
    );
    assert_eq!(
        hand.stacks(),
        vec![(SeatNo(0), Chips(0)), (SeatNo(1), Chips(2000))]
    );
}
