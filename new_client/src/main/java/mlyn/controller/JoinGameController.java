package mlyn.controller;

import java.io.IOException;
import java.nio.charset.StandardCharsets;

import javafx.application.Platform;
import javafx.concurrent.Task;
import javafx.scene.Scene;
import javafx.scene.control.Alert;
import javafx.scene.control.Alert.AlertType;
import javafx.stage.Stage;
import mlyn.model.Message;
import mlyn.model.MessageType;

public class JoinGameController extends LobbyView {
    public JoinGameController(String username) throws IOException {
        client.send(new Message(MessageType.PLAYER_INIT_JOIN, username.getBytes(StandardCharsets.UTF_8)));

        Task<Message> receiverTask = new Task<Message>() {
            @Override
            protected Message call() throws Exception {
                return client.getMessage(MessageType.READY, MessageType.PLAYER_JOIN_NOTIFY, MessageType.NOK);
            }
        };

        Task<Message> joinTask = new Task<Message>() {
            @Override
            protected Message call() throws Exception {
                return client.getMessage(MessageType.READY);
            }
        };

        joinTask.setOnSucceeded(e -> joinGame(joinTask.getValue()));

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
                    executorService.execute(joinTask);
                } break;

                case READY: {
                    joinGame(msg);
                } break;
            }
        });


        executorService.execute(receiverTask);
    }


}
