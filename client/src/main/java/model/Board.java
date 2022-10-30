package model;

import java.util.Objects;

public record Board(int[] board) {
    private static final int BOARD_SIZE = 24;

    public Board {
        Objects.requireNonNull(board);
        if(board.length != BOARD_SIZE) {
            throw new IllegalArgumentException("Invalid board size");
        }
    }

    public void setColor(int index, Color color) {
        board[index] = color.value;
    }
}
