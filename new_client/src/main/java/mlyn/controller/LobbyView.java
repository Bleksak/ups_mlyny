package mlyn.controller;

import java.io.IOException;
import java.nio.ByteBuffer;
import java.nio.charset.StandardCharsets;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;

import javafx.application.Platform;
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

public class LobbyView extends BorderPane {
    protected Client client;
    protected ExecutorService executorService = Executors.newFixedThreadPool(1);

    public LobbyView() throws IOException {
        client = new Client();
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

    protected void joinGame(Message msg) {

        this.executorService.shutdownNow();

        if(msg.type() == MessageType.NOK) {
            Alert alert = new Alert(AlertType.ERROR);
            alert.setHeaderText(new String(msg.data(), StandardCharsets.UTF_8));
            Platform.runLater( () -> alert.showAndWait());
            close(null);
        }

        ByteBuffer buffer = ByteBuffer.wrap(msg.data());
        Machine.State state = Machine.State.valueOf(buffer.getInt());
        Color color = buffer.get() == 1 ? Color.RED : Color.BLUE;
        byte[] board = buffer.array();

        client.getMachine().setState(state);
        client.setColor(color);

        Platform.runLater(() -> {
            Scene sc = new Scene(new GameController(client, board));
            Stage stage = (Stage) this.getScene().getWindow();
            stage.setScene(sc);
        
            stage.show();
        });
    }

}
