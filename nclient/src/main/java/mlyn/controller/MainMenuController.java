package mlyn.controller;

import java.io.IOException;
import java.util.Set;

import javafx.application.Platform;
import javafx.event.ActionEvent;
import javafx.geometry.Pos;
import javafx.scene.Scene;
import javafx.scene.control.Alert;
import javafx.scene.control.Alert.AlertType;
import javafx.scene.control.Button;
import javafx.scene.control.Label;
import javafx.scene.control.TextField;
import javafx.scene.image.Image;
import javafx.scene.image.ImageView;
import javafx.scene.layout.BorderPane;
import javafx.scene.layout.HBox;
import javafx.scene.layout.VBox;
import javafx.stage.Stage;
import mlyn.model.Client;

public class MainMenuController extends BorderPane {

    private TextField usernameField = new TextField();
    private TextField ipField = new TextField();
    private TextField portField = new TextField();

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

        Label ipLabel = new Label("IP Address");
        ipLabel.setAlignment(Pos.CENTER);

        Label portLabel = new Label("Port number");
        portLabel.setAlignment(Pos.CENTER);

        usernameField.setAlignment(Pos.CENTER);
        usernameField.setMaxWidth(128);
        
        VBox vbox = new VBox(ipLabel, ipField, portLabel, portField, usernameLabel, usernameField, createButton, joinButton, quitButton);
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

    void createGameClicked(ActionEvent event) {
        try {
            Stage stage = new Stage();
            
            short port = Short.parseShort(portField.getText());

            Client client = new Client(usernameField.getText(), ipField.getText(), port);
            Scene scene = new Scene(new CreateGameController(client, usernameField.getText()), 800, 600);
            stage.setScene(scene);
            stage.setResizable(false);
            stage.centerOnScreen();
            stage.setTitle("Mill - Game");

            stage.show();
        } catch (IOException e) {
            Alert alert = new Alert(AlertType.ERROR);
            alert.setHeaderText("Failed to connect to the server");
            alert.showAndWait();
        }
        catch (NumberFormatException e) {
            Alert alert = new Alert(AlertType.ERROR);
            alert.setHeaderText("Invalid port!");
            alert.showAndWait();
        }
    }

    void joinGameClicked(ActionEvent event) {
        try {
            Stage stage = new Stage();
            short port = Short.parseShort(portField.getText());

            Client client = new Client(usernameField.getText(), ipField.getText(), port);
            Scene scene = new Scene(new JoinGameController(client, usernameField.getText()), 800, 600);
            stage.setScene(scene);
            stage.setResizable(false);
            stage.centerOnScreen();
            stage.setTitle("Mill - Game");

            stage.show();
        } catch (IOException e) {
            Alert alert = new Alert(AlertType.ERROR);
            alert.setHeaderText("Failed to connect to the server");
            alert.showAndWait();
        }
        catch (NumberFormatException e) {
            Alert alert = new Alert(AlertType.ERROR);
            alert.setHeaderText("Invalid port!");
            alert.showAndWait();
        }

    }

    void quitClicked(ActionEvent event) {
        Set<Thread> threadSet = Thread.getAllStackTraces().keySet();

        for(Thread t : threadSet) {
            if(((Object)t instanceof OpponentThread) || ((Object)t instanceof Client) || ((Object)t instanceof JoinGameController) || ((Object)t instanceof CreateGameController) || ((Object)t instanceof GameController)) {
                System.out.println("interrupting" + t.getClass().getName());
                t.interrupt();
            }
        }

        Platform.exit();
    }
}
