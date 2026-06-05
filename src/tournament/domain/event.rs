use super::player::PlayerSpec;
use super::table::TableId;
use super::table::TableSpec;
use super::tournament::TournamentId;


#[derive(Debug, Clone)]
pub struct TournamentEvent {
    pub tournament_id: TournamentId,
    pub event_type: TournamentEventType,
}


#[derive(Debug, Clone)]
pub enum TournamentEventType {
    TableOpened {
        table_id: TableId,
        table_spec: TableSpec,
    },
    PlayerSeatedAtTable {
        player_spec: PlayerSpec,
        stack: u32,
        table_id: TableId,
    },
    TournamentStarted,
    TableClosed {
        table_id: TableId,
        // TODO: remaining players moved to table(s) ...
    },
    TournamentFinished,
}