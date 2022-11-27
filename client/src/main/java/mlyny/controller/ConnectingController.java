package mlyny.controller;

import java.io.IOException;
import java.net.URL;
import java.util.ResourceBundle;

import javafx.application.Platform;
import javafx.event.ActionEvent;
import javafx.fxml.FXML;
import javafx.scene.control.Alert;
import javafx.scene.control.ButtonType;
import javafx.scene.control.ProgressIndicator;
import javafx.scene.control.Alert.AlertType;
import mlyny.Main;
import mlyny.model.Client;
import mlyny.model.PingSpammer;

public class ConnectingController extends Thread implements INotifiableController {

    @FXML
    private ResourceBundle resources;

    @FXML
    private URL location;

    @FXML
    private ProgressIndicator spinner;

    private static int connectionCounter = 0;

    private static final String FAILED_CONNECT_MESSAGE   = "Failed to connect, the application will exit now";
    private static final String FAILED_RECONNECT_MESSAGE = "Failed to reconnect, the application will exit now";

    private static final long connectionTimeout = 5000;

    @FXML
    void disconnect(ActionEvent event) {
        Alert alert = new Alert(AlertType.CONFIRMATION);
        alert.getButtonTypes().clear();
        alert.getButtonTypes().addAll(ButtonType.YES, ButtonType.NO);
        alert.setHeaderText("Are you sure you want to disconnect?");
        alert.setContentText("The appllication will exit");

        alert.showAndWait().ifPresent( type -> {
            if(type == ButtonType.YES) {
                Main.exit();
            }
        });
    }

    void showAlertAndExit() {
        Platform.runLater(() -> {
            Alert alert = new Alert(AlertType.ERROR);
            alert.setHeaderText(connectionCounter == 0 ? FAILED_CONNECT_MESSAGE : FAILED_RECONNECT_MESSAGE);
            alert.showAndWait();
            Main.exit();
        });
    }

    void showServerOffAndExit() {
        Platform.runLater(() -> {
            Alert alert = new Alert(AlertType.ERROR);
            alert.setHeaderText("Lost connection to server, server shut down");
            alert.showAndWait();
            Main.exit();
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

        client.stopThread();
        start = System.currentTimeMillis();

        try {
            while(!client.connect()) {
                long time = System.currentTimeMillis() - start;
                System.out.println(time);
                if(time >= connectionTimeout) {
                    showAlertAndExit();
                    return;
                }

                Thread.sleep(20);
            }
        } catch(IOException ex) {
            showServerOffAndExit();
            // showAlertAndExit();
        } catch(InterruptedException ex) {
            Thread.currentThread().interrupt();
        }

        connectionCounter += 1;

        PingSpammer ctrl = new PingSpammer(client);
        ctrl.setDaemon(true);
        ctrl.start();

        client.startThread();
        if(!client.isAlive()) {
            client.start();
        }

        Main.connectionDone();
    }

    @FXML
    void initialize() {
        start();
    }
}
