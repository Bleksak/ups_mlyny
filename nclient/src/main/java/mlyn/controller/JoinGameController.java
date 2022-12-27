package mlyn.controller;

import java.io.IOException;

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

    public JoinGameController(Client client, String username) throws IOException {
        super(client);
        client.sendMessage(new Message(MessageType.PLAYER_INIT_JOIN, username));

        Task<Message> receiverTask = new Task<Message>() {
            @Override
            protected Message call() throws Exception {
                return client.getMessage(MessageType.CRASH, MessageType.READY, MessageType.JOINED, MessageType.NOK);
            }
        };

        receiverTask.setOnSucceeded(e -> {
            Message msg = receiverTask.getValue();
            switch(msg.type()) {
                case NOK: {
                    String message = msg.data().length > 0 ? msg.data()[0] : "Server crashed, game aborted!";
                    Alert alert = new Alert(AlertType.ERROR);
                    alert.setHeaderText(message);
                    Platform.runLater(() -> {
                        alert.showAndWait();
                    });

                    this.close(null);
                } break;

                case JOINED: {
                    waitForConnection();
                } break;

                case READY: {
                    joinGame(msg);
                } break;

                case CRASH: {
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
