use super::card::Card;
use super::player::Player;
use super::player::PlayerData;

use uuid::Uuid;


#[derive(Debug, Clone)]
pub struct Table {
    id: Uuid,
    seats: Vec<Option<Player>>,
    button: u8,
    state: TableState,
    events: Vec<TableEvent>,
}

impl Table {
    pub fn new(id: Uuid, seat_count: u8) -> Self {
        assert!(seat_count >= 2 && seat_count <= 10);
        let mut seats = vec![];
        for _ in 0..seat_count {
            // seats.push(Seat::new(position));
            seats.push(None);
        }
        let button_position = seat_count - 1;
        Self {
            id,
            seats,
            button: button_position,
            state: TableState::Idle,
            events: vec![
                TableEvent {
                    table_id: id,
                    event_type: TableEventType::TableOpened {
                        seat_count,
                        button_position,
                    }
                }
            ],
        }
    }

    pub fn consume_events(&mut self) -> Vec<TableEvent> {
        std::mem::take(&mut self.events)
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn seat_player(&mut self, player_data: PlayerData) {
        // assert!(!self.has_player(data.player_id()));
        let index = self.seats.iter().position(|seat| seat.is_none()).unwrap();
        let player = Player::new(index as u8, player_data.clone());
        _ = self.seats[index].insert(player);
        let event_type = TableEventType::PlayerSeated {
            position: index as u8,
            player_data,
        };
        self.add_event(event_type);
    }

    pub fn start_game(&mut self) {
        assert!(matches!(self.state, TableState::Idle));
        self.forward_button();
        self.start_hands();
        self.pay_blinds();
    }

    fn forward_button(&mut self) {
        let mut index = SeatIndex::new((self.button + 1) as usize, self.seats.len());
        while self.seats[index.position].is_none() {
            index = index.next();
        }
        self.button = index.position as u8;
        self.add_event(TableEventType::ButtonMoved { button_position: self.button });
    }

    fn start_hands(&mut self) {
        for player in self.seats.iter_mut().flatten() {
            player.start_hand();
        }
    }

    fn pay_blinds(&mut self) {
        self.pay_small_blind();
        self.pay_big_blind();
    }

    fn pay_small_blind(&mut self) {
        let button_distance = if self.player_count() > 2 { 1u8 } else { 0u8 };
        let position = self.player_position(button_distance);
        self.place_bet(position, 25);
    }

    fn pay_big_blind(&mut self) {
        let button_distance = if self.player_count() > 2 { 2u8 } else { 1u8 };
        let position = self.player_position(button_distance);
        self.place_bet(position, 50);
    }

    fn place_bet(&mut self, position: usize, amount: u32) {
        let player = self.seats[position].as_mut().unwrap();
        let real_amount = player.place_bet(amount);
        let event_type = TableEventType::BetPlaced {
            position: player.position(),
            amount: real_amount,
            current_bet_sum: player.current_bet_sum().unwrap(),
            remaining_stack: player.stack(),
        };
        self.add_event(event_type);
    }

    fn player_position(&self, button_distance: u8) -> usize {
        let mut count = 0u8;
        let mut index = SeatIndex::new(self.button as usize, self.seats.len());
        while count != button_distance {
            index = index.next();
            if self.seats[index.position].is_some() {
                count += 1;
            }
        }
        index.position
    }

    fn player_count(&self) -> u8 {
        self.seats.iter().flatten().count() as u8
    }

    fn add_event(&mut self, event_type: TableEventType) {
        self.events.push(
            TableEvent { table_id: self.id, event_type }
        );
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct TableEvent {
    table_id: Uuid,
    event_type: TableEventType,
}


#[derive(Debug, Clone, PartialEq)]
pub enum TableEventType {
    TableOpened {
        seat_count: u8,
        button_position: u8,
    },
    PlayerSeated {
        position: u8,
        player_data: PlayerData,
    },
    ButtonMoved {
        button_position: u8,
    },
    BetPlaced {
        position: u8,
        amount: u32,
        current_bet_sum: u32,
        remaining_stack: u32,
    }
}


#[derive(Debug, Clone)]
struct SeatIndex {
    position: usize,
    seat_count: usize,
}

impl SeatIndex {
    pub fn new(position: usize, seat_count: usize) -> Self {
        Self { position: position % seat_count, seat_count }
    }

    pub fn next(&self) -> Self {
        let next_pos = (self.position + 1) % self.seat_count;
        Self::new(next_pos, self.seat_count)
    }
}


#[derive(Debug, Clone)]
enum TableState {
    Idle,
    Playing(Game),
    Paused,
}


#[derive(Debug, Clone)]
struct Game {
    deck: u8,
    board: Vec<Card>,
    pots: Vec<u32>,
    request: Option<u8>,
}

impl Game {
    pub fn new() -> Self {
        Self { deck: 2, board: vec![], pots: vec![], request: None }
    }

    pub fn pay(&mut self, player_id: Uuid, amount: u32) {

    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let table_id = Uuid::new_v4();
        let mut table = Table::new(table_id, 5);
        assert_eq!(table.id(), table_id);
        let events = table.consume_events();
        assert_eq!(events, vec![
            TableEvent {
                table_id,
                event_type: TableEventType::TableOpened {
                    seat_count: 5,
                    button_position: 4,
                }
            }
        ]);
    }

    #[test]
    fn seat_player() {
        let mut table = create_table(5);
        let player_data = create_player_data("Daniel");
        table.seat_player(player_data.clone());
        assert_eq!(table.consume_events(), vec![
            TableEvent {
                table_id: table.id(),
                event_type: TableEventType::PlayerSeated { position: 0, player_data },
            }
        ]);

        let player_data = create_player_data("Maria");
        table.seat_player(player_data.clone());
        assert_eq!(table.consume_events(), vec![
            TableEvent {
                table_id: table.id(),
                event_type: TableEventType::PlayerSeated { position: 1, player_data },
            }
        ]);
    }

    #[test]
    fn start_game() {
        let player_datas = vec![
            create_player_data("Daniel"),
            create_player_data("Maria"),
            create_player_data("Tillmann"),
        ];
        let mut table = create_table_with_seated_players(player_datas);

        table.start_game();

        assert_eq!(table.consume_events(), vec![
            TableEvent {
                table_id: table.id(),
                event_type: TableEventType::ButtonMoved { button_position: 0 },
            },
            TableEvent { // small blind
                table_id: table.id(),
                event_type: TableEventType::BetPlaced {
                    position: 1,
                    amount: 25,
                    current_bet_sum: 25,
                    remaining_stack: 1475
                },
            },
            TableEvent { // big blind
                table_id: table.id(),
                event_type: TableEventType::BetPlaced {
                    position: 2,
                    amount: 50,
                    current_bet_sum: 50,
                    remaining_stack: 1450
                },
            }
        ]);
    }

    fn create_table_with_seated_players(player_datas: Vec<PlayerData>) -> Table {
        let mut table = create_table(5);
        let expected_seated_events: Vec<TableEvent> = player_datas
            .iter()
            .enumerate()
            .map(|(index, player_data)| TableEvent {
                table_id: table.id(),
                event_type: TableEventType::PlayerSeated {
                    position: index as u8,
                    player_data: player_data.clone(),
                }
            })
            .collect();
        for player_data in player_datas {
            table.seat_player(player_data);
        }
        assert_eq!(table.consume_events(), expected_seated_events);
        table
    }

    fn create_table(seat_count: u8) -> Table {
        let mut table = Table::new(Uuid::new_v4(), seat_count);
        assert_eq!(table.consume_events(), vec![
            TableEvent {
                table_id: table.id(),
                event_type: TableEventType::TableOpened {
                    seat_count,
                    button_position: seat_count - 1,
                }
            }
        ]);
        table
    }

    fn create_player_data(nickname: &str) -> PlayerData {
        PlayerData::new(Uuid::new_v4(), nickname.to_string(), 1500)
    }
}
