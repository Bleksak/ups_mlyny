package mlyn;

import javafx.application.Application;
import javafx.scene.Scene;
import javafx.stage.Stage;
import mlyn.controller.MainMenuController;

public class Main extends Application {

    private static int WIDTH = 800;
    private static int HEIGHT = 600;

    public static void main(String... args) {
        Application.launch(Main.class, args);
    }

    @Override
    public void start(Stage primaryStage) throws Exception {
        Scene scene = new Scene(new MainMenuController(), WIDTH, HEIGHT);
        
        primaryStage.setScene(scene);
        primaryStage.setResizable(false);
        primaryStage.centerOnScreen();
        primaryStage.setTitle("Mill");

        primaryStage.show();
    }
}
