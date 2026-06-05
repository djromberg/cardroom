use super::tournament::TournamentId;

use uuid::Uuid;


#[derive(Debug, Clone)]
pub struct TournamentEvent {
    pub tournament_id: TournamentId,
    pub event_type: TournamentEventType,
}


#[derive(Debug, Clone)]
pub enum TournamentEventType {
    TableOpened {
        table_id: Uuid,
        seat_count: u8,
    },
    PlayerRegistered {
        table_id: Uuid,
    },
    TournamentStarted,
    TableClosed {
        table_id: Uuid,
        // TODO: remaining players moved to table(s) ...
    },
    TournamentFinished,
}