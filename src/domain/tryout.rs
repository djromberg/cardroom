use std::collections::HashMap;

use uuid::Uuid;

#[derive(Debug)]
struct Player {
    account_id: Uuid,
    nickname: String,
    stack: u32,
    bet: u32,
}

impl Player {
    pub fn new(account_id: Uuid, nickname: String, stack: u32) -> Self {
        Self { account_id, nickname, stack, bet: 0 }
    }

    pub fn account_id(&self) -> Uuid {
        self.account_id
    }

    pub fn bet(&self) -> u32 {
        self.bet
    }

    pub fn add_bet(&mut self, amount: u32) {
        self.stack -= amount;
        self.bet += amount;
    }
}

#[derive(Debug, Clone)]
pub enum TableEvent {
    Created {
        id: Uuid,
        seat_count: u8
    },
    PlayerSeated {
        account_id: Uuid,
        nickname: String,
        stack: u32,
        position: u8
    },
    PlayerWipedOut {
        position: u8,
    }
}


#[derive(Debug)]
pub struct Table {
    id: Uuid,
    seats: Vec<Option<Player>>,
    events: Vec<TableEvent>
}

impl Table {
    pub fn default() -> Self {
        Self { id: Uuid::new_v4(), seats: vec![], events: vec![] }
    }

    pub fn create(id: Uuid, seat_count: u8) -> Self {
        let mut table = Self::default();
        let event = TableEvent::Created { id, seat_count };
        table.apply_and_push_event(event);
        table
    }

    pub fn events(&self) -> Vec<TableEvent> {
        self.events.clone()
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn has_free_seat(&self) -> bool {
        self.seats.iter().any(|seat| seat.is_none())
    }

    pub fn apply_event(&mut self, event: &TableEvent) {
        match event {
            TableEvent::Created { id, seat_count } => {
                self.id = *id;
                for _ in 0..*seat_count {
                    self.seats.push(None)
                }
            },
            TableEvent::PlayerSeated { account_id, nickname, stack, position } => {
                self.seats[*position as usize] = Some(Player::new(*account_id, nickname.clone(), *stack));
            },
            TableEvent::PlayerWipedOut { position } => {
                let seat = self.seats.get_mut(*position as usize).unwrap();
                seat.take();
            }
        }
    }

    pub fn seat_player(&mut self, account_id: Uuid, nickname: String, stack: u32) {
        // TODO: more domain logic such as verification that player not yet present
        let position = self.find_free_seat_position();
        let event = TableEvent::PlayerSeated { account_id, nickname, stack, position };
        self.apply_and_push_event(event);
    }

    fn find_free_seat_position(&self) -> u8 {
        let (position, _) = self.seats.iter().enumerate().find(|(_, seat)| seat.is_none()).unwrap();
        position as u8
    }

    fn apply_and_push_event(&mut self, event: TableEvent) {
        self.apply_event(&event);
        self.events.push(event);
    }
}


#[derive(Debug, Clone)]
pub enum TournamentEvent {
    Created {
        id: Uuid,
        table_ids: Vec<Uuid>,
        table_seat_count: u8,
        initial_stack: u32,
    },
    PlayerJoined {
        account_id: Uuid,
        table_id: Uuid,
        nickname: String,
        stack: u32,
    }
}

#[derive(Debug)]
pub struct Tournament {
    id: Uuid,
    tables: HashMap<Uuid, u8>,
    players: HashMap<Uuid, Uuid>,
    table_seat_count: u8,
    initial_stack: u32,
    events: Vec<TournamentEvent>,
}

impl Tournament {
    pub fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            tables: HashMap::new(),
            players: HashMap::new(),
            table_seat_count: 0,
            initial_stack: 0,
            events: vec![]
        }
    }

    pub fn create(table_count: usize, table_seat_count: u8) -> Self {
        let mut tournament = Self::default();
        let mut table_ids = vec![];
        for _ in 0..table_count {
            table_ids.push(Uuid::new_v4());
        }
        let event = TournamentEvent::Created { id: tournament.id(), table_ids, table_seat_count, initial_stack: 1500 };
        tournament.apply_and_push_event(event);
        tournament
    }

    pub fn events(&self) -> Vec<TournamentEvent> {
        self.events.clone()
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn join_player(&mut self, account_id: Uuid, nickname: String) {
        // TODO: more business logic here, ensure invariants such as player not already joined and tournament not already running etc.
        //       if everything is good, create, apply, and push event
        let table_id = self.find_free_table_id();
        let event = TournamentEvent::PlayerJoined { account_id, table_id, nickname, stack: self.initial_stack };
        self.apply_and_push_event(event);
    }

    pub fn apply_event(&mut self, event: &TournamentEvent) {
        match event {
            TournamentEvent::Created { id, table_seat_count, table_ids, initial_stack } => {
                self.id = *id;
                self.table_seat_count = *table_seat_count;
                self.initial_stack = *initial_stack;
                for table_id in table_ids {
                    self.tables.insert(*table_id, 0);
                }
            },
            TournamentEvent::PlayerJoined { account_id, table_id, .. } => {
                let player_count = self.tables.get_mut(table_id).unwrap();
                *player_count += 1;
                self.players.insert(*account_id, *table_id);

            }
        }
    }

    fn find_free_table_id(&self) -> Uuid {
        let (table_id, _) = self.tables.iter().find(|(_, player_count)| player_count < &&self.table_seat_count).unwrap();
        *table_id
    }

    fn apply_and_push_event(&mut self, event: TournamentEvent) {
        self.apply_event(&event);
        self.events.push(event);
    }
}


pub trait TableRepository {
    fn save_table(&mut self, table: Table);
    fn load_table(&self, table_id: Uuid) -> Table;
}


pub trait TournamentRepository {
    fn save_tournament(&mut self, tournament: Tournament);
    fn load_tournament(&self, tournament_id: Uuid) -> Tournament;
}


pub fn create_tournament<Tournaments: TournamentRepository, Tables: TableRepository>(
    table_count: usize, table_seat_count: u8, tournaments: &mut Tournaments, tables: &mut Tables
) {
    let tournament = Tournament::create(table_count, table_seat_count);
    let events = tournament.events();
    tournaments.save_tournament(tournament);
    process_tournament_events(events, tables);
}


pub fn act_on_table<Tournaments: TournamentRepository, Tables: TableRepository>(
    account_id: Uuid, table_id: Uuid, tournaments: &mut Tournaments, tables: &mut Tables
) {
    let mut table = tables.load_table(table_id);
    // table.act(...)
    let events = table.events();
    tables.save_table(table);
    process_table_events(events, tournaments);
}


pub fn process_tournament_events<Tables: TableRepository>(events: Vec<TournamentEvent>, tables: &mut Tables) {
    for event in events {
        match event {
            TournamentEvent::Created { id, table_ids, table_seat_count, .. } => {
                for table_id in table_ids {
                    let table = Table::create(table_id, table_seat_count);
                    tables.save_table(table);
                }
            },
            TournamentEvent::PlayerJoined { account_id, table_id, nickname, stack } => {
                let mut table = tables.load_table(table_id);
                table.seat_player(account_id, nickname.clone(), stack);
                tables.save_table(table);
            }
        }
    }
}


pub fn process_table_events<Tournaments: TournamentRepository>(events: Vec<TableEvent>, tournaments: &mut Tournaments) {
    for event in events {
        match event {
            TableEvent::Created { id, seat_count } => {
            },
            TableEvent::PlayerSeated { account_id, nickname, stack, position } => {
            },
            TableEvent::PlayerWipedOut { position } => {
            }
        }
    }
}
