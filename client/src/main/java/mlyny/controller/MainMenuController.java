package mlyny.controller;

import java.io.IOException;

import javafx.application.Platform;
import javafx.fxml.FXML;
import javafx.scene.control.Alert;
import javafx.scene.control.TextField;
import javafx.scene.control.Alert.AlertType;
import javafx.scene.input.MouseEvent;
import mlyny.Main;
import mlyny.model.Client;
import mlyny.model.Message;
import mlyny.model.MessageType;


public class MainMenuController implements INotifiableController {

    public void receivedMessage(Message message) {
        Platform.runLater(() -> {
            if(message.type() == MessageType.PLAYER_INIT_USERNAME_INVALID) {
                System.out.println("invalid username");
                Alert alert = new Alert(AlertType.WARNING);
                alert.setHeaderText("Invalid username!");
                alert.showAndWait();
                return;
            }

            if(message.type() == MessageType.PLAYER_INIT_USERNAME_USED) {
                System.out.println("username used");

                Alert alert = new Alert(AlertType.WARNING);
                alert.setHeaderText("Username is taken!");
                alert.showAndWait();
            }

            if(message.type() == MessageType.OK) {
                Main.setRoot("controller/CreateGameView");
            }
        });
    }

    @FXML
    private TextField username;

    private boolean usernameValid() {
        String usernameString = username.getText();

        if(usernameString.isEmpty()) {
            Alert alert = new Alert(AlertType.WARNING);
            alert.setHeaderText("Username cannot be empty");
            alert.showAndWait();
            return false;
        }

        if(usernameString.contains("\n")) {
            Alert alert = new Alert(AlertType.WARNING);
            alert.setHeaderText("Invalid username! (cannot contain newline)");
            alert.showAndWait();
            return false;
        }

        return true;
    }

    @FXML
    void createGameClicked(MouseEvent event) {
        if(!usernameValid()) {
            return;
        }

        try {
            Client.getInstance().createGameRequest(username.getText());
        } catch(IOException ex) {}
    }

    @FXML
    void joinGameClicked(MouseEvent event) {
    }


    @FXML
    void quitClicked(MouseEvent event) {
        Main.exit();
    }

}
