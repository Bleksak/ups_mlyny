package mlyn.controller;

import javafx.geometry.Pos;
import javafx.scene.control.Button;
import javafx.scene.control.ProgressIndicator;
import javafx.scene.layout.BorderPane;
import javafx.scene.layout.HBox;
import javafx.scene.layout.VBox;
import javafx.scene.text.Text;

public class LobbyView extends BorderPane {
    public LobbyView() {
        Text waitingText = new Text("Waiting for players");
        ProgressIndicator indicator = new ProgressIndicator();
        Button exitButton = new Button("Quit");

        VBox vbox = new VBox(waitingText, indicator, exitButton);
        vbox.setAlignment(Pos.CENTER);
        vbox.setSpacing(10);

        HBox hbox = new HBox(vbox);
        hbox.setAlignment(Pos.CENTER);

        setPrefSize(800, 600);
        setMinSize(800, 600);
        setCenter(hbox);
    }
}
