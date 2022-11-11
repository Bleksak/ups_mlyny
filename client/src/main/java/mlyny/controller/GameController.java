package mlyny.controller;

import javafx.fxml.FXML;
import javafx.scene.input.MouseButton;
import javafx.scene.input.MouseEvent;
import javafx.scene.shape.Circle;

import java.util.List;

public class GameController {

    private int lastSelectedIndex = -1;

    @FXML
    private List<Circle> stoneList;

    public void placeStone(MouseEvent mouseEvent) {
        MouseButton btn = mouseEvent.getButton();

        int index = stoneList.indexOf(mouseEvent.getTarget());

        if(btn == MouseButton.PRIMARY) {
            placePrimaryBtn(index);
        } else if(btn == MouseButton.SECONDARY) {
            placeSecondaryBtn(index);
        }

        System.out.println(index);
    }

    private void placePrimaryBtn(int index) {
//        TODO: move
    }

    private void placeSecondaryBtn(int index) {
//        TODO: place

    }
}
