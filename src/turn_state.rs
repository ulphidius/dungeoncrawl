#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TurnState {
    AwaitingInput,
    PlayerTurn,
    MonsterTurn,
    NewFloor,
    GameOver,
    Victory,
}
