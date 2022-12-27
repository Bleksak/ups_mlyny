package mlyn.controller;

import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;

import javafx.application.Platform;
import javafx.concurrent.Task;
import javafx.event.ActionEvent;
import javafx.geometry.Pos;
import javafx.scene.Scene;
import javafx.scene.control.Alert;
import javafx.scene.control.Button;
import javafx.scene.control.ProgressIndicator;
import javafx.scene.control.Alert.AlertType;
import javafx.scene.layout.BorderPane;
import javafx.scene.layout.HBox;
import javafx.scene.layout.VBox;
import javafx.scene.paint.Color;
import javafx.scene.text.Text;
import javafx.stage.Stage;
import javafx.stage.WindowEvent;
import mlyn.model.Client;
import mlyn.model.Machine;
import mlyn.model.Message;
import mlyn.model.MessageType;
import mlyn.model.Machine.State;

public class LobbyView extends BorderPane {
    protected Client client;
    protected ExecutorService executorService = Executors.newFixedThreadPool(1);

    private final Task<Message> joinTask = new Task<Message>() {
        @Override
        protected Message call() throws Exception {
            return client.getMessage(MessageType.READY, MessageType.CRASH);
        }
    };

    public LobbyView(Client client) {
        this.client = client;
        joinTask.setOnSucceeded(e -> joinGame(joinTask.getValue()));

        Text waitingText = new Text("Waiting for players");
        ProgressIndicator indicator = new ProgressIndicator();
        Button exitButton = new Button("Quit");
        exitButton.setOnAction(this::close);

        VBox vbox = new VBox(waitingText, indicator, exitButton);
        vbox.setAlignment(Pos.CENTER);
        vbox.setSpacing(10);

        HBox hbox = new HBox(vbox);
        hbox.setAlignment(Pos.CENTER);

        setPrefSize(800, 600);
        setMinSize(800, 600);
        setCenter(hbox);
    }

    protected void close(ActionEvent event) {
        executorService.shutdownNow();
        client.disconnect();
        getScene().getWindow().fireEvent(new WindowEvent(this.getScene().getWindow(), WindowEvent.WINDOW_CLOSE_REQUEST));
    }

    protected void waitForConnection() {
        client.getMachine().setState(State.LOBBY);
        executorService.execute(joinTask);
    }

    protected void badArguments() {
        Alert alert = new Alert(AlertType.ERROR);
        alert.setHeaderText("Server crashed, game aborted!");
        Platform.runLater( () -> alert.showAndWait());
        close(null);
    }

    protected void joinGame(Message msg) {

        this.executorService.shutdownNow();

        if(msg.type() == MessageType.CRASH) {
            badArguments();
            return;
        }

        // ByteBuffer buffer = ByteBuffer.wrap(msg.data());
        if(msg.data().length != 4) {
            badArguments();
            return;
        }

        Machine.State state = Machine.State.valueOf(Integer.parseInt(msg.data()[0]));
        if(msg.data()[1].equals("1")) {
            client.setColor(Color.RED);
        } else if(msg.data()[1].equals("2")) {
            client.setColor(Color.BLUE);
        } else {
            badArguments();
            return;
        }

        client.getMachine().setState(state);

        if(msg.data()[2].length() != 24) {
            badArguments();
            return;
        }

        if(msg.data()[3].isEmpty()) {
            badArguments();
            return;
        }

        Platform.runLater(() -> {
            Scene sc = new Scene(new GameController(client, msg.data()[2], msg.data()[3]));
            Stage stage = (Stage) this.getScene().getWindow();
            stage.setScene(sc);
        
            stage.show();
        });
    }

}
