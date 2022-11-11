package mlyny.controller;

import model.Client;

import java.io.IOException;

public abstract class SocketController {
    protected Client client = Client.getInstance();

    protected SocketController() throws IOException {
    }
}
