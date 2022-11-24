package mlyny.controller;

import javafx.fxml.FXML;
import javafx.scene.Scene;
import javafx.scene.control.TextField;
import javafx.scene.input.MouseEvent;
import javafx.scene.text.Text;
import mlyny.Main;
import mlyny.model.Client;
import mlyny.model.Message;
import mlyny.model.MessageType;

public class CreateGameController implements INotifiableController {

    public void receivedMessage(Message message) {
        System.out.println(message.type());

        if(message.type() == MessageType.PLAYER_JOIN_NOTIFY) {
            Main.setRoot("CreateGameView");
        }
    }

    private static final int MAX_PLAYERS = 2;
    private int currentPlayers = 1;

    @FXML
    private Scene self;

    @FXML
    private TextField inviteLinkTextField;

    @FXML
    void goBackClicked(MouseEvent event) {
        Main.setRoot("controller/MainMenuView");
    }

    @FXML
    void initialize() {
    }
}
