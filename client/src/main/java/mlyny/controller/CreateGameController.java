package mlyny.controller;

import javafx.event.ActionEvent;
import javafx.fxml.FXML;
import mlyny.Main;

public class CreateGameController implements INotifiableController {
    @FXML
    void exitClick(ActionEvent event) {
        System.out.println("called exit");
        Main.exit();
    }

    @FXML
    void initialize() {
        System.out.println("init");
    }
}
