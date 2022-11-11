package mlyny;

import javafx.application.Application;
import javafx.fxml.FXMLLoader;
import javafx.scene.Parent;
import javafx.scene.Scene;
import javafx.stage.Stage;
import model.Client;

import java.io.IOException;

public class Main extends Application {

    private static Scene scene;

    public static void main(String... args) {
        launch(args);
    }

    private static Parent loadFXML(String fxml) throws IOException {
        return new FXMLLoader(Main.class.getResource(fxml + ".fxml")).load();
    }

    public static void setRoot(String fxml) throws IOException {
        scene.setRoot(loadFXML(fxml));
    }

    @Override
    public void start(Stage stage) throws Exception {
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
