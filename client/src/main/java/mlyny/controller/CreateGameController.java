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
import model.Client;

import java.io.IOException;
import java.util.Objects;

public class CreateGameController {

    private static final int MAX_PLAYERS = 2;
    private int currentPlayers = 1;

    private Client client;

    @FXML
    private Scene self;

    @FXML
    private Text connectedPlayersText;

    @FXML
    private TextField inviteLinkTextField;

    @FXML
    void goBackClicked(MouseEvent event) {
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
    void initialize() {

        // try {
        //     client = Client.getInstance();
        //     long game_id = client.readLong();
        //     long player_id = client.readLong();
        //     client.endRead();

        //     inviteLinkTextField.setText(String.format("%d", game_id));
        //     System.out.println(game_id);
        //     System.out.println(player_id);
        // } catch (IOException e) {
        //     throw new RuntimeException(e);
        // }

        // we check if game was created successfully
        // then check every 5 seconds if player has joined

        connectedPlayersText.setText(createTextField());
    }

    private String createTextField() {
        return String.format("%d/%d", currentPlayers, MAX_PLAYERS);
    }

}
