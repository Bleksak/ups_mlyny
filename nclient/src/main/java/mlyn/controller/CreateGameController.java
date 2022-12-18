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

public class CreateGameController extends LobbyView {

    public CreateGameController(Client client, String username) throws IOException {
        super(client);
        client.sendMessage(new Message(MessageType.PLAYER_INIT_CREATE, username.getBytes(StandardCharsets.UTF_8)));

        Task<Message> createTask = new Task<Message>() {
            @Override
            protected Message call() throws Exception {
                return client.getMessage(MessageType.PLAYER_JOIN_NOTIFY, MessageType.NOK, MessageType.SERVER_CRASH);
            }
        };

        createTask.setOnSucceeded(e -> {
            Message msg = createTask.getValue();

            if(msg.type() == MessageType.NOK) {
                String message = new String(msg.data(), StandardCharsets.UTF_8);
                Alert alert = new Alert(AlertType.ERROR);
                alert.setHeaderText(message);
                Platform.runLater(() -> {
                    alert.showAndWait();
                });

                this.close(null);
            }
            else if(msg.type() == MessageType.SERVER_CRASH) {
                Alert alert = new Alert(AlertType.ERROR);
                alert.setHeaderText("Server crashed, game aborted!");
                Platform.runLater(() -> {
                    alert.showAndWait();
                });

                this.close(null);
            }
            else {
                waitForConnection();
            }
        });

        executorService.execute(createTask);
    }
}
