package mlyn.controller;

import java.io.IOException;
import java.nio.charset.StandardCharsets;

import javafx.application.Platform;
import javafx.concurrent.Task;
import javafx.scene.control.Alert;
import javafx.scene.control.Alert.AlertType;
import mlyn.model.Client;
import mlyn.model.Message;
import mlyn.model.MessageType;

public class JoinGameController extends LobbyView {
    public JoinGameController(Client client) {
        super(client);
        waitForConnection();
    }

    public JoinGameController(String username) throws IOException {
        client.send(new Message(MessageType.PLAYER_INIT_JOIN, username.getBytes(StandardCharsets.UTF_8)));

        Task<Message> receiverTask = new Task<Message>() {
            @Override
            protected Message call() throws Exception {
                return client.getMessage(MessageType.SERVER_CRASH, MessageType.READY, MessageType.PLAYER_JOIN_NOTIFY, MessageType.NOK);
            }
        };

        receiverTask.setOnSucceeded(e -> {
            Message msg = receiverTask.getValue();
            switch(msg.type()) {
                case NOK: {
                    String message = new String(msg.data(), StandardCharsets.UTF_8);
                    Alert alert = new Alert(AlertType.ERROR);
                    alert.setHeaderText(message);
                    Platform.runLater(() -> {
                        alert.showAndWait();
                    });

                    this.close(null);
                } break;

                case PLAYER_JOIN_NOTIFY: {
                    waitForConnection();
                } break;

                case READY: {
                    joinGame(msg);
                } break;

                case SERVER_CRASH: {
                    Alert alert = new Alert(AlertType.ERROR);
                    alert.setHeaderText("Server crashed, game aborted!");
                    Platform.runLater(() -> {
                        alert.showAndWait();
                    });

                    this.close(null);
                } break;
                default: {}
            }

        });


        executorService.execute(receiverTask);
    }


}
