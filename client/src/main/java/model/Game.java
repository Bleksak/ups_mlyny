package model;

import java.util.Objects;

public record Game(long id, Player player) {
    public Game {
        Objects.requireNonNull(player);
        if(id == 0) {
            throw new IllegalArgumentException("Game ID cannot be zero");
        }
    }
}
