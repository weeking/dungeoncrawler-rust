#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TurnState {
    AwaitingInput,
    PlayerTurn,
    EnemyTurn,
    WorldTurn,
    GameOver,
    Victory,
    NextLevel,
}
