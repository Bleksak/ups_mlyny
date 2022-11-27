package mlyny.model;

import java.util.Objects;

public record Board(byte[] board) {
    private static final int BOARD_SIZE = 24;

    public Board {
        Objects.requireNonNull(board);
        if(board.length != BOARD_SIZE) {
            throw new IllegalArgumentException("Invalid board size");
        }
    }

    public void setColor(int index, LColor color) {
        board[index] = color.value;
    }
}
