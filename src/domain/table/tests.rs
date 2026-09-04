use super::super::shared::PlayerId;
use super::card::Card;
use super::*;

fn deck() -> Deck {
    Deck::new(std::array::from_fn(|index| Card::from_index(index as u8))).unwrap()
}

#[test]
fn table_controls_hand_and_updates_stacks() {
    let mut table = Table::open(TableId(7), 2);
    table.seat_player(PlayerInfo::new(PlayerId(1), "one".into(), Chips(1000)));
    table.seat_player(PlayerInfo::new(PlayerId(2), "two".into(), Chips(1000)));
    let started = table
        .start_hand(
            deck(),
            Blinds {
                small: Chips(50),
                big: Chips(100),
            },
        )
        .unwrap();
    assert!(matches!(
        started.first(),
        Some(TableEvent::HandStarted {
            dealer_seat: SeatNo(0),
            ..
        })
    ));
    assert_eq!(table.seats[0].player().unwrap().stack(), None);
    assert_eq!(table.seats[1].player().unwrap().stack(), None);

    table.act(SeatNo(0), Action::Fold).unwrap();
    assert!(table.hand.is_none());
    assert_eq!(table.seats[0].player().unwrap().stack(), Some(Chips(950)));
    assert_eq!(table.seats[1].player().unwrap().stack(), Some(Chips(1050)));
}

#[test]
fn cannot_replace_an_active_hand() {
    let mut table = Table::open(TableId(7), 2);
    table.seat_player(PlayerInfo::new(PlayerId(1), "one".into(), Chips(1000)));
    table.seat_player(PlayerInfo::new(PlayerId(2), "two".into(), Chips(1000)));
    table
        .start_hand(
            deck(),
            Blinds {
                small: Chips(50),
                big: Chips(100),
            },
        )
        .unwrap();
    assert_eq!(
        table.start_hand(
            deck(),
            Blinds {
                small: Chips(50),
                big: Chips(100)
            }
        ),
        Err(TableError::HandInProgress)
    );
}
