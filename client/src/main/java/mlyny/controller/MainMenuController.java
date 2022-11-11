package mlyny.controller;

import javafx.application.Platform;
import javafx.fxml.FXML;
import javafx.scene.Scene;
import javafx.scene.control.Alert;
import javafx.scene.input.MouseEvent;
import mlyny.Main;


public class MainMenuController {

    @FXML
    private Scene self;

    @FXML
    void createGameClicked(MouseEvent event) {
        // try {
        //     Client client = Client.getInstance();
        //     client.beginWrite();
        //     client.writeInt(ServerMessage.CREATE_GAME.value);
        //     client.endWrite();
        //     client.beginRead();
        //     int result = Client.getInstance().readInt();


        //     if(result != ClientMessage.OK.value) {
        //         System.out.println("argggg");
        //         client.endRead();
        //         throw new IOException();
        //     }

        //     Main.setRoot("CreateGameView.fxml");
        // } catch(IOException ignored) {
        //     Alert alert = new Alert(Alert.AlertType.ERROR);
        //     alert.setContentText("Failed to create a game");
        //     alert.show();
        // }
    }

    @FXML
    void joinGameClicked(MouseEvent event) {
        try {
            Main.setRoot("JoinGameView.fxml");
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
