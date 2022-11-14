package mlyny.controller;

import javafx.application.Platform;
import javafx.fxml.FXML;
import javafx.fxml.FXMLLoader;
import javafx.scene.Scene;
import javafx.scene.control.Alert;
import javafx.scene.control.TextField;
import javafx.scene.input.MouseEvent;
import javafx.scene.text.Text;
import javafx.stage.Stage;
import mlyny.Main;

import java.util.Objects;

public class JoinGameController implements INotifiableController {

    @FXML
    private TextField invitationCode;

    @FXML
    private Text playersJoinedText;

    @FXML
    private Text playerCountText;

    @FXML
    void backToMenuClicked(MouseEvent event) {
        Main.setRoot("MainMenuView");
    }

    @FXML
    void joinGameClicked(MouseEvent event) {
        
    }

    @FXML
    void initialize() {

    }

}
