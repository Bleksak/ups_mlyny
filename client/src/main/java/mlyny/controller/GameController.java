package mlyny.controller;

import javafx.event.ActionEvent;
import javafx.fxml.FXML;
import javafx.scene.input.MouseButton;
import javafx.scene.input.MouseEvent;
import javafx.scene.paint.Color;
import javafx.scene.shape.Circle;
import mlyny.Main;
import mlyny.model.Board;
import mlyny.model.Client;
import mlyny.model.LColor;

import java.io.IOException;
import java.util.List;

public class GameController implements INotifiableController {

    private Board m_board;
    private LColor m_color;

    // 0 invalid, 1 put, 2 take, 3 move
    private int last_op_type = 0;
    private int last_op_pos1 = -1;
    private int last_op_pos2 = -1;

    @FXML
    private List<Circle> stoneList;

    public void exitGame(ActionEvent event) {
        Main.exit();
    }

    public synchronized void placeStone(MouseEvent mouseEvent) {
        MouseButton btn = mouseEvent.getButton();
        System.out.println("clicked");

        int index = stoneList.indexOf(mouseEvent.getTarget());

        last_op_pos1 = index;

        if(btn == MouseButton.PRIMARY) {
            placePrimaryBtn(index);
        } else if(btn == MouseButton.SECONDARY) {
            placeSecondaryBtn(index);
        }

        System.out.println(index);
    }

    private void placePrimaryBtn(int index) {
        try {
            Client.getInstance().put(index);
        } catch (IOException e) {
            e.printStackTrace();
        }
    }

    private void placeSecondaryBtn(int index) {

    }

    public void setBoard(Board board) {
        m_board = board;
        redraw();
    }

    public void setColor(LColor color) {
        m_color = color;
    }

    public void redraw() {
        Color[] colors = new Color[]{Color.BLACK, Color.RED, Color.BLUE};

        for(int i = 0; i < m_board.board().length; ++i) {
            System.out.println("DRAWING PICO MORE");
            // m_board.board()[i]
            int index = (int) m_board.board()[i];
            stoneList.get(i).setFill(colors[index]);
        }
    }

    @FXML
    void initialize() {

    }

    public synchronized void confirmLastOperation() {
        switch(last_op_type) {
            case 1: m_board.setColor(last_op_pos1, m_color); break;
            case 2: m_board.setColor(last_op_pos1, LColor.NONE); break;
            case 3: {
                m_board.setColor(last_op_pos1, LColor.NONE);
                m_board.setColor(last_op_pos2, m_color);
            } break;
        }

        last_op_pos1 = -1;
        last_op_pos2 = -1;

        redraw();
    }
}
