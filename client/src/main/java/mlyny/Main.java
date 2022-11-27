package mlyny;

import javafx.application.Application;
import javafx.application.Platform;
import javafx.fxml.FXMLLoader;
import javafx.scene.Parent;
import javafx.scene.Scene;
import javafx.scene.control.Alert;
import javafx.scene.control.Alert.AlertType;
import javafx.stage.Stage;
import mlyny.controller.INotifiableController;

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
                exit();
            }
        });
    }

    public static INotifiableController getController() {
        return (INotifiableController) scene.getRoot().getUserData();
    }

    private static Parent loadFXML(String fxml) throws IOException { 
        FXMLLoader loader = new FXMLLoader(Main.class.getResource(fxml + ".fxml"));
        Parent parent = loader.load();
        parent.setUserData(loader.getController());
        return parent;
    }

    public static void setRoot(String fxml) {
        Platform.runLater(() -> {
            try {
                scene.setRoot(loadFXML(fxml));
            } catch(IOException ex) {
                Alert alert = new Alert(AlertType.ERROR);
                alert.setHeaderText("Application error, please reinstall your application");
                alert.showAndWait();
                exit();
            }
        });
    }

    @Override
    public void start(Stage stage) throws Exception {
        scene = new Scene(loadFXML("controller/MainMenuView"), 800, 600);
        stage.setScene(scene);

        showConnectingView();

        stage.setOnCloseRequest( (e) -> {
            exit();
        });

        stage.show();
    }

    public static void exit() {
        Platform.exit();
        System.exit(0);
    }
}
