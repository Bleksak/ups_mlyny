package mlyn.controller;

import javafx.application.Platform;
import javafx.event.ActionEvent;
import javafx.geometry.Pos;
import javafx.scene.control.Button;
import javafx.scene.control.ProgressIndicator;
import javafx.scene.layout.BorderPane;
import javafx.scene.layout.HBox;
import javafx.scene.layout.VBox;
import javafx.scene.text.Text;

public class ConnectingController extends BorderPane {
    public ConnectingController() {
        
        Text reconnectText = new Text("Attempting to reconnect");

        Button quitButton = new Button("Quit");
        quitButton.setMinWidth(128);
        quitButton.setMinHeight(50);
        quitButton.setOnAction(this::quitClicked);

        ProgressIndicator indicator = new ProgressIndicator();
        indicator.setMinSize(200, 200);

        VBox vbox = new VBox(reconnectText, indicator, quitButton);
        vbox.setMinWidth(200);
        vbox.setAlignment(Pos.CENTER);
        vbox.setSpacing(15);

        HBox hbox = new HBox(vbox);
        hbox.setAlignment(Pos.CENTER);

        setPrefSize(800, 600);
        setMinSize(800, 600);
        setCenter(hbox);
    }

    void quitClicked(ActionEvent event) {
        Platform.exit();
    }
}
