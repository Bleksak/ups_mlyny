package mlyn.controller;

import java.nio.ByteBuffer;
import java.util.ArrayList;
import java.util.List;
import java.util.Set;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;

import javafx.concurrent.Task;
import javafx.event.ActionEvent;
import javafx.geometry.HPos;
import javafx.geometry.Insets;
import javafx.geometry.Pos;
import javafx.scene.Group;
import javafx.scene.control.Button;
import javafx.scene.layout.BorderPane;
import javafx.scene.layout.ColumnConstraints;
import javafx.scene.layout.GridPane;
import javafx.scene.layout.HBox;
import javafx.scene.layout.Priority;
import javafx.scene.layout.RowConstraints;
import javafx.scene.layout.VBox;
import javafx.scene.paint.Color;
import javafx.scene.shape.Circle;
import javafx.scene.shape.Line;
import javafx.stage.WindowEvent;
import mlyn.model.Client;
import mlyn.model.Message;
import mlyn.model.MessageType;

class OpponentThread extends Thread {

    private Client client;
    private GameController controller;

    OpponentThread(GameController controller, Client client) {
        this.client = client;
        this.controller = controller;
    }

    public void run() {
        try {
            while(true) {
                Message msg = client.getMessage(MessageType.PLAYER_PUT);
                int index = ByteBuffer.wrap(msg.data()).getInt();
                Color opponentColor = client.getColor() == Color.RED ? Color.BLUE : Color.RED;
                controller.getCircle(index).setFill(opponentColor);
            }
        } catch(InterruptedException ex) {
            System.out.println("interrupting");
            Thread.currentThread().interrupt();
        }
        System.out.println("end");
    }
}

public class GameController extends BorderPane {

    private static Set<Integer> stones = Set.of(0, 3, 6, 8, 10, 12, 16, 17, 18, 21, 22, 23, 25, 26, 27, 30, 31, 32, 36, 38, 40, 42, 45, 48);
    private static int rows = 7;
    private static int cols = 7;
    private Client client;
    private ExecutorService service = Executors.newFixedThreadPool(2);
    private List<Circle> circles = new ArrayList<>();

    private static Line lineMaker(GridPane grid, int indexFrom, int indexTo) {
        double xFrom = ((Circle)grid.getChildren().get(indexFrom)).getCenterX();
        double yFrom = ((Circle)grid.getChildren().get(indexFrom)).getCenterY();

        double xTo = ((Circle)grid.getChildren().get(indexTo)).getCenterX();
        double yTo = ((Circle)grid.getChildren().get(indexTo)).getCenterY();

        Line l = new Line(xFrom, yFrom, xTo, yTo);
        l.setStrokeWidth(7);
        return l;
    }

    private void makeGUI() {
        Button quitButton = new Button("Quit");
        quitButton.setOnAction(this::quitClicked);
        quitButton.setMinHeight(30);
        quitButton.setPrefHeight(30);
        quitButton.setMaxHeight(30);

        HBox topMenu = new HBox(quitButton);
        topMenu.setPadding(new Insets(15, 0, 0, 0));
        topMenu.setAlignment(Pos.CENTER);
        topMenu.setMinHeight(30);
        topMenu.setPrefHeight(30);
        topMenu.setMaxHeight(30);

        int gridSize = 500;

        GridPane grid = new GridPane();
        grid.setMinSize(gridSize, gridSize);
        grid.setPrefSize(gridSize, gridSize);
        grid.setMaxSize(gridSize, gridSize);

        int minHeight = gridSize / rows;
        int minWidth = gridSize / cols;

        int w = 0;
        int h = (int)0;
        System.out.println(h);

        for(int i = 0; i < cols; ++i) {
            ColumnConstraints colConstraint = new ColumnConstraints();
            colConstraint.setHgrow(Priority.SOMETIMES);
            colConstraint.setHalignment(HPos.CENTER);
            colConstraint.setMinWidth(minWidth);
            colConstraint.setPrefWidth(minWidth);
            colConstraint.setMaxWidth(minWidth);

            grid.getColumnConstraints().add(colConstraint);
        }

        for(int i = 0; i < rows; ++i) {
            RowConstraints rowConstraint = new RowConstraints();
            rowConstraint.setVgrow(Priority.SOMETIMES);
            rowConstraint.setMinHeight(minHeight);
            rowConstraint.setPrefHeight(minHeight);
            rowConstraint.setMaxHeight(minHeight);

            grid.getRowConstraints().add(rowConstraint);
        }

        for(int i = 0, j = 0; i < rows*cols; ++i) {
            if(!stones.contains(i)) {
                continue;
            }

            int y = i / cols;
            int x = i % cols;

            Circle c = new Circle();
            c.setFill(Color.BLACK);
            c.setRadius(15);
            c.setCenterX((x+0.5) * minWidth + w);
            c.setCenterY((y+0.5) * minHeight + h);

            final int index = j++;
            c.setOnMouseClicked(e -> circleClicked(index));

            circles.add(c);
            grid.add(c, x, y);
        }

        Line[] lines = {
            lineMaker(grid, 0, 2),
            lineMaker(grid, 0, 21),
            lineMaker(grid, 21, 23),
            lineMaker(grid, 2, 23),
            lineMaker(grid, 3, 5),
            lineMaker(grid, 6, 8),
            lineMaker(grid, 9, 11),
            lineMaker(grid, 12, 14),
            lineMaker(grid, 15, 17),
            lineMaker(grid, 18, 20),
            lineMaker(grid, 3, 18),
            lineMaker(grid, 6, 15),
            lineMaker(grid, 8, 17),
            lineMaker(grid, 5, 20),
        };

        Group group = new Group(lines);
        group.getChildren().add(grid);

        VBox vbox = new VBox(topMenu, group);
        vbox.setAlignment(Pos.CENTER);

        setMinSize(800, 600);
        setPrefSize(800, 600);
        setCenter(vbox);
    }

    public GameController(Client client, byte[] board) {
        this.client = client;
        this.service.execute(new OpponentThread(this, client));
        makeGUI();
    }

    public Circle getCircle(int index) { 
        return circles.get(index);
    }

    private void quitClicked(ActionEvent event) {
        this.service.shutdownNow();
        this.client.disconnect();
        this.getScene().getWindow().fireEvent(new WindowEvent(this.getScene().getWindow(), WindowEvent.WINDOW_CLOSE_REQUEST));
    }

    private void circleClicked(int index) {
        Task<Message> task = new Task<Message>() {
            @Override
            protected Message call() throws Exception {
                client.send(new Message(MessageType.PLAYER_PUT, ByteBuffer.allocate(Integer.BYTES).putInt(index).array()));
                return client.getMessage(MessageType.NOK, MessageType.OK);
            }
        };

        task.setOnSucceeded(e -> {
            Message msg = task.getValue();
            switch(msg.type()) {
                case NOK: {


                } break;
                case OK: {

                    System.out.println("setting color");
                    circles.get(index).setFill(client.getColor());

                } break;
            }
        });

        service.execute(task);
    }

}
