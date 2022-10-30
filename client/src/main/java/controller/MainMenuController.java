package controller;

import javafx.application.Platform;
import javafx.fxml.FXML;
import javafx.fxml.FXMLLoader;
import javafx.scene.Scene;
import javafx.scene.control.Alert;
import javafx.scene.input.MouseEvent;
import javafx.stage.Stage;
import model.Client;
import model.ClientMessage;
import model.ServerMessage;

import java.io.IOException;
import java.util.Objects;

public class MainMenuController {

    @FXML
    private Scene self;

    @FXML
    void createGameClicked(MouseEvent event) {
        Stage stage = (Stage) self.getWindow();
        try {
            Client client = Client.getInstance();
            client.beginWrite();
            client.writeInt(ServerMessage.CREATE_GAME.value);
            client.endWrite();
            client.beginRead();
            int result = Client.getInstance().readInt();


            if(result != ClientMessage.OK.value) {
                System.out.println("argggg");
                client.endRead();
                throw new IOException();
            }

            Scene scene = FXMLLoader.load(Objects.requireNonNull(CreateGameController.class.getResource("CreateGameView.fxml")));
            System.out.println("WHAT");
            stage.setScene(scene);
        } catch(IOException ignored) {
            Alert alert = new Alert(Alert.AlertType.ERROR);
            alert.setContentText("Failed to create a game");
            alert.show();
        }
    }

    @FXML
    void joinGameClicked(MouseEvent event) {
        Stage stage = (Stage) self.getWindow();
        try {
            Scene scene = FXMLLoader.load(Objects.requireNonNull(CreateGameController.class.getResource("JoinGameView.fxml")));
            stage.setScene(scene);
        } catch(Exception ex) {
            Alert alert = new Alert(Alert.AlertType.ERROR);
            alert.setContentText("Failed to create a game");
            alert.show();
        }
    }

    @FXML
    void quitClicked(MouseEvent event) {
        Platform.exit();
    }

}
