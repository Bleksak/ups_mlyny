package mlyny;

import javafx.application.Application;
import javafx.application.Platform;
import javafx.fxml.FXMLLoader;
import javafx.scene.Parent;
import javafx.scene.Scene;
import javafx.scene.control.Alert;
import javafx.scene.control.Alert.AlertType;
import javafx.stage.Stage;
import model.Client;

import java.io.IOException;

public class Main extends Application {

    private static Scene scene;
    private static Parent lastRoot;

    public static void main(String... args) {
        launch(args);
    }

    public static void connectionDone() {
        scene.setRoot(lastRoot);
    }

    public static void showConnectingView() {
        lastRoot = scene.getRoot();

        Platform.runLater(() -> {
            try {
                scene.setRoot(loadFXML("controller/ConnectingView"));
            } catch(IOException ex) {
                Alert alert = new Alert(AlertType.ERROR);
                alert.setHeaderText("Application error, please reinstall your application");
                alert.showAndWait();
                Platform.exit();
            }
        });
    }

    private static Parent loadFXML(String fxml) throws IOException {
        return new FXMLLoader(Main.class.getResource(fxml + ".fxml")).load();
    }

    public static void setRoot(String fxml) {
        Platform.runLater(() -> {
            try {
                scene.setRoot(loadFXML(fxml));
            } catch(IOException ex) {
                    Alert alert = new Alert(AlertType.ERROR);
                    alert.setHeaderText("Application error, please reinstall your application");
                    alert.showAndWait();
                    Platform.exit();
            }
        });
    }

    @Override
    public void start(Stage stage) throws Exception {
        lastRoot = loadFXML("controller/MainMenuView");
        scene = new Scene(loadFXML("controller/ConnectingView"), 800, 600);
        stage.setScene(scene);

        stage.setOnCloseRequest( (e) -> {
            // try {
            //     Client.getInstance().close();
            // } catch (IOException ex) {}
        });

        stage.show();
    }
}
