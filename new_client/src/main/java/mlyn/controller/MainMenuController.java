package mlyn.controller;

import java.io.IOException;

import javafx.event.ActionEvent;
import javafx.geometry.Pos;
import javafx.scene.Node;
import javafx.scene.Scene;
import javafx.scene.control.Button;
import javafx.scene.control.Label;
import javafx.scene.control.TextField;
import javafx.scene.image.Image;
import javafx.scene.image.ImageView;
import javafx.scene.layout.BorderPane;
import javafx.scene.layout.HBox;
import javafx.scene.layout.VBox;
import javafx.stage.Stage;

public class MainMenuController extends BorderPane {

    private TextField usernameField = new TextField();

    public MainMenuController() {
        Button createButton = new Button("Create game");
        Button joinButton = new Button("Join game");
        Button quitButton = new Button("Quit");
        createButton.setMinHeight(50);
        createButton.setMinWidth(128);
        createButton.setOnAction(this::createGameClicked);

        joinButton.setMinHeight(50);
        joinButton.setMinWidth(128);
        joinButton.setOnAction(this::joinGameClicked);

        quitButton.setMinHeight(50);
        quitButton.setMinWidth(128);
        quitButton.setOnAction(this::quitClicked);

        Label usernameLabel = new Label("Username");
        usernameLabel.setAlignment(Pos.CENTER);

        usernameField.setAlignment(Pos.CENTER);
        usernameField.setMaxWidth(128);
        
        VBox vbox = new VBox(usernameLabel, usernameField, createButton, joinButton, quitButton);
        vbox.setAlignment(Pos.CENTER);
        vbox.setSpacing(10);
        vbox.setMinWidth(128);

        ImageView image = new ImageView(new Image(getClass().getResource("meme.png").toExternalForm()));
        image.setFitWidth(286);
        image.setFitHeight(312);
        image.setPickOnBounds(true);
        image.setPreserveRatio(true);

        HBox hbox = new HBox(vbox, image);
        hbox.setSpacing(10);
        hbox.setAlignment(Pos.CENTER);

        setPrefSize(800, 600);
        setMinSize(800, 600);
        setCenter(hbox);
    }

    public void getVBox() {
        Button createButton = new Button("Create game");
        Button joinButton = new Button("Join game");
        Button quitButton = new Button("Quit");

        VBox box = new VBox(createButton, joinButton, quitButton);
        box.setAlignment(Pos.CENTER);
        box.setLayoutX(336);
        box.setLayoutY(200);
        box.setSpacing(10);

    }

    void createGameClicked(ActionEvent event) {
        CreateGameController view = null;

        try {
            view = new CreateGameController(usernameField.getText());
            Stage stage = (Stage) ((Node) event.getSource()).getScene().getWindow();
            stage.setScene(new Scene(view));
            stage.show();
        } catch(IOException e) {}
    }

    void joinGameClicked(ActionEvent event) {

    }

    void quitClicked(ActionEvent event) {

    }
}
