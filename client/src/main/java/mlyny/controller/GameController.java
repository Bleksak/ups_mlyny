package mlyny.controller;

import javafx.event.ActionEvent;
import javafx.fxml.FXML;
import javafx.scene.input.MouseButton;
import javafx.scene.input.MouseEvent;
import javafx.scene.shape.Circle;
import mlyny.Main;
import mlyny.model.Client;

import java.io.IOException;
import java.util.List;

public class GameController implements INotifiableController {

    @FXML
    private List<Circle> stoneList;

    public void exitGame(ActionEvent event) {
        Main.exit();
    }

    public void placeStone(MouseEvent mouseEvent) {
        MouseButton btn = mouseEvent.getButton();
        System.out.println("clicked");

        int index = stoneList.indexOf(mouseEvent.getTarget());

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
}
