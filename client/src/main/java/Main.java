import controller.MainMenuController;
import javafx.application.Application;
import javafx.fxml.FXMLLoader;
import javafx.scene.Scene;
import javafx.stage.Stage;
import model.Client;

import java.io.IOException;
import java.util.Objects;

public class Main extends Application {

    public static void main(String... args) {
        launch(args);
    }

    @Override
    public void start(Stage stage) throws Exception {
        try {
            Client.getInstance();
        } catch(Exception ignored) {}

        stage.setWidth(800);
        stage.setHeight(600);

        Scene scene = FXMLLoader.load(Objects.requireNonNull(MainMenuController.class.getResource("MainMenuView.fxml")));
        stage.setScene(scene);

        stage.setOnCloseRequest( (e) -> {
            try {
                Client.getInstance().close();
            } catch (IOException ex) {}
        });

        stage.show();
    }
}
