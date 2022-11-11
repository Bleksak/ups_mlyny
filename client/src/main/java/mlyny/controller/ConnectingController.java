package mlyny.controller;

import java.io.IOException;
import java.net.URL;
import java.util.Optional;
import java.util.ResourceBundle;

import javafx.application.Platform;
import javafx.event.ActionEvent;
import javafx.fxml.FXML;
import javafx.scene.control.Alert;
import javafx.scene.control.ButtonType;
import javafx.scene.control.ProgressIndicator;
import javafx.scene.control.Alert.AlertType;
import mlyny.Main;
import model.Client;

public class ConnectingController extends Thread {

    @FXML
    private ResourceBundle resources;

    @FXML
    private URL location;

    @FXML
    private ProgressIndicator spinner;

    private static final long connectionTimeout = 1000;

    @FXML
    void disconnect(ActionEvent event) {
        Alert alert = new Alert(AlertType.CONFIRMATION);
        alert.getButtonTypes().clear();
        alert.getButtonTypes().addAll(ButtonType.YES, ButtonType.NO);
        alert.setHeaderText("Are you sure you want to disconnect?");
        alert.setContentText("The appllication will exit");

        alert.showAndWait().ifPresent( type -> {
            if(type == ButtonType.YES) {
                Platform.exit();
            }
        });
    }

    void showAlertAndExit() {
        Platform.runLater(() -> {
            Alert alert = new Alert(AlertType.ERROR);
            alert.setHeaderText("Failed to connect to the server");
            alert.showAndWait();
            Platform.exit();
        });
    }

    public void run() {
        long start = System.currentTimeMillis();
        Client client = null;

        while(client == null) {
            try {
                long time = System.currentTimeMillis() - start;
                System.out.println(time);
            
                if(time >= connectionTimeout) {
                    showAlertAndExit();
                    return;
                }

                client = Client.getInstance();

                Thread.sleep(20);
            } catch (IOException e) {
            } catch(InterruptedException ex) {
                Thread.currentThread().interrupt();
            }
        }

        start = System.currentTimeMillis();

        try {
            while(!Client.getInstance().connect()) {
                if(System.currentTimeMillis() - start >= connectionTimeout) {
                    showAlertAndExit();
                    return;
                }

                Thread.sleep(20);
            }
        } catch(IOException ex) {
            showAlertAndExit();
        } catch(InterruptedException ex) {
            Thread.currentThread().interrupt();
        }

        Main.connectionDone();
    }

    @FXML
    void initialize() {
        start();
    }
}
