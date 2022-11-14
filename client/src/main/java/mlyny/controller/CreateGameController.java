package mlyny.controller;

import javafx.fxml.FXML;
import javafx.scene.Scene;
import javafx.scene.control.TextField;
import javafx.scene.input.MouseEvent;
import javafx.scene.text.Text;
import mlyny.Main;
import mlyny.model.Client;
import mlyny.model.Message;

public class CreateGameController implements INotifiableController {

    public void receivedMessage(Message message) {
        
    }

    private static final int MAX_PLAYERS = 2;
    private int currentPlayers = 1;

    @FXML
    private Scene self;

    @FXML
    private Text connectedPlayersText;

    @FXML
    private TextField inviteLinkTextField;

    @FXML
    void goBackClicked(MouseEvent event) {
        Main.setRoot("controller/MainMenuView");
    }

    @FXML
    void initialize() {
        connectedPlayersText.setText(createTextField());
    }

    private String createTextField() {
        return String.format("%d/%d", currentPlayers, MAX_PLAYERS);
    }

}
