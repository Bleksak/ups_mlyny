package controller;

import javafx.application.Platform;
import javafx.fxml.FXML;
import javafx.fxml.FXMLLoader;
import javafx.scene.Scene;
import javafx.scene.control.Alert;
import javafx.scene.control.TextField;
import javafx.scene.input.MouseEvent;
import javafx.scene.text.Text;
import javafx.stage.Stage;

import java.util.Objects;

public class JoinGameController {

    @FXML
    private Scene self;

    @FXML
    private TextField invitationCode;

    @FXML
    private Text playersJoinedText;

    @FXML
    private Text playerCountText;

    @FXML
    void backToMenuClicked(MouseEvent event) {
        Stage stage = (Stage) self.getWindow();

        try {
            Scene scene = FXMLLoader.load(Objects.requireNonNull(MainMenuController.class.getResource("MainMenuView.fxml")));
            stage.setScene(scene);
        } catch(Exception ex) {
            Alert alert = new Alert(Alert.AlertType.ERROR);
            alert.setContentText("Unexpected error has occured, the application will crash now");
            alert.show();
            Platform.exit();
        }
    }

    @FXML
    void joinGameClicked(MouseEvent event) {
//        TODO: send request, update text

    }

    @FXML
    void initialize() {

    }

}
