pub struct Machine {
    
}

enum State {
    Init,
    InLobby,
    InGamePut,
    InGameTake,
    InGameMove,
    GameOver,
}