#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TurnState {
    AwaitingInput,
    PlayerTurn,
    MonsterTurn,
    NextLevel,
    GameOver,
    Victory,
}
