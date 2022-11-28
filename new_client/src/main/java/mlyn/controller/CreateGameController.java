package mlyn.controller;

import java.io.IOException;
import java.nio.charset.StandardCharsets;

import javafx.concurrent.Task;
import mlyn.model.Client;
import mlyn.model.Message;
import mlyn.model.MessageType;

public class CreateGameController extends LobbyView {
    public CreateGameController(String username) throws IOException {
        Client client = new Client();
        client.send(new Message(MessageType.PLAYER_INIT_CREATE, username.getBytes(StandardCharsets.UTF_8)));

        Task<Message> receiverTask = new Task<Message>() {
            @Override
            protected Message call() throws Exception {
                return client.getMessage();
            }
        };


    }
}
