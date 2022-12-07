package mlyn.controller;

import java.nio.ByteBuffer;
import java.util.ArrayList;
import java.util.List;
import java.util.Set;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;

import javafx.application.Platform;
import javafx.concurrent.Task;
import javafx.event.ActionEvent;
import javafx.geometry.HPos;
import javafx.geometry.Insets;
import javafx.geometry.Pos;
import javafx.scene.Group;
import javafx.scene.Scene;
import javafx.scene.control.Alert;
import javafx.scene.control.Button;
import javafx.scene.control.Alert.AlertType;
import javafx.scene.input.MouseButton;
import javafx.scene.input.MouseEvent;
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
import javafx.scene.text.Font;
import javafx.scene.text.Text;
import javafx.stage.Stage;
import javafx.stage.WindowEvent;
import mlyn.model.Client;
import mlyn.model.Machine;
import mlyn.model.Message;
import mlyn.model.MessageType;
import mlyn.model.Machine.State;

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
                Message msg = client.getMessage(MessageType.SERVER_CRASH, MessageType.PLAYER_PUT, MessageType.PLAYER_TAKE, MessageType.PLAYER_MV, MessageType.GAME_STATE, MessageType.DISCONNECT, MessageType.OVER);
                switch(msg.type()) {
                    case SERVER_CRASH: {
                        controller.serverCrash();
                        return;
                    }
                    case PLAYER_PUT: {
                        int index = ByteBuffer.wrap(msg.data()).getInt();
                        Color opponentColor = client.getColor() == Color.RED ? Color.BLUE : Color.RED;
                        controller.getCircle(index).setFill(opponentColor);
                    } break;

                    case PLAYER_TAKE: {
                        int index = ByteBuffer.wrap(msg.data()).getInt();
                        controller.getCircle(index).setFill(Color.BLACK);
                    } break;

                    case PLAYER_MV: {
                        ByteBuffer buffer = ByteBuffer.wrap(msg.data());
                        int oldIndex = buffer.getInt();
                        int newIndex = buffer.getInt();

                        Color opponentColor = client.getColor() == Color.RED ? Color.BLUE : Color.RED;
                        controller.getCircle(oldIndex).setFill(Color.BLACK);
                        controller.getCircle(newIndex).setFill(opponentColor);
                    } break;

                    case GAME_STATE: {
                        if(msg.data().length > 0) {
                            ByteBuffer buffer = ByteBuffer.wrap(msg.data());
                            Machine.State newState = Machine.State.valueOf(buffer.getInt());
                            client.getMachine().setState(newState);
                            System.out.println(newState.name());
                            controller.updateTurn();
                        }
                    } break;

                    case DISCONNECT: {
                        controller.opponentDisconnected();
                    } break;

                    case OVER: {
                        controller.gameOver();
                        return;
                    }
                }
            }
        } catch(InterruptedException ex) {
            Thread.currentThread().interrupt();
        }
    }
}

public class GameController extends BorderPane {

    private static Set<Integer> stones = Set.of(0, 3, 6, 8, 10, 12, 16, 17, 18, 21, 22, 23, 25, 26, 27, 30, 31, 32, 36, 38, 40, 42, 45, 48);
    private static int rows = 7;
    private static int cols = 7;
    private Client client;
    private ExecutorService service = Executors.newFixedThreadPool(2);
    private List<Circle> circles = new ArrayList<>();
    private int prevIndex = -1;

    private Text turnText = new Text();
    private Text errorText = new Text();

    private static Line lineMaker(GridPane grid, int indexFrom, int indexTo) {
        double xFrom = ((Circle)grid.getChildren().get(indexFrom)).getCenterX();
        double yFrom = ((Circle)grid.getChildren().get(indexFrom)).getCenterY();

        double xTo = ((Circle)grid.getChildren().get(indexTo)).getCenterX();
        double yTo = ((Circle)grid.getChildren().get(indexTo)).getCenterY();

        Line l = new Line(xFrom, yFrom, xTo, yTo);
        l.setStrokeWidth(7);
        return l;
    }

    public void updateTurn() {
        switch(client.getMachine().getState()) {
            case GAME_MOVE:
            case GAME_PUT:
            case GAME_TAKE: {
                turnText.setText("Your turn!");
                break;
            }

            case GAME_OVER: {
                turnText.setText("Game over!");
            } break;

            default: turnText.setText("Opponent's turn!");
        }
    }

    public void serverCrash() {
        Platform.runLater(() -> {
            Alert alert = new Alert(AlertType.ERROR);
            alert.setHeaderText("Server crashed, game terminated!");
            alert.showAndWait().ifPresent(e -> quitClicked(null));
        });
    }

    private void makeGUI() {
        this.setOnMouseClicked(e -> {
            errorText.setText("");
            if(e.getButton() == MouseButton.SECONDARY) {
                prevIndex = -1;
            }
        });

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
            c.setOnMouseClicked(e -> circleClicked(e, index));

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
            lineMaker(grid, 1, 7),
            lineMaker(grid, 16, 22),
        };

        Group group = new Group(lines);
        group.getChildren().add(grid);

        turnText.setFont(Font.font(15.0));
        errorText.setFont(Font.font(15.0));
        errorText.setFill(Color.RED);

        VBox vbox = new VBox(topMenu, group, turnText, errorText);
        vbox.setAlignment(Pos.CENTER);

        setMinSize(800, 800);
        setPrefSize(800, 800);
        setCenter(vbox);

        updateTurn();
    }

    private int countColor(Color color) {
        int count = 0;
        for(Circle c : circles) {
            if(c.getFill() == color) {
                count += 1;
            }
        }

        return count;
    }

    public void gameOver() {
        int redCount = countColor(Color.RED);
        int blueCount = countColor(Color.BLUE);

        String redWin = "Red player wins!";
        String bluWin = "Blue player wins!";

        Platform.runLater(() -> {
            Alert alert = new Alert(AlertType.INFORMATION);
            if(redCount > blueCount) {
                alert.setHeaderText(redWin);
            } else {
                alert.setHeaderText(bluWin);
            }

            alert.showAndWait().ifPresent(e -> quitClicked(null));
        });
    }

    public GameController(Client client, ByteBuffer board) {
        this.client = client;
        this.service.execute(new OpponentThread(this, client));
        makeGUI();

        int size = board.remaining();

        for(int i = 0; i < size; ++i) {
            byte b = board.get();
            if(b == 0) {
                continue;
            }

            Color c = b == 1 ? Color.RED : Color.BLUE;
            circles.get(i).setFill(c);
        }
    }

    public Circle getCircle(int index) { 
        return circles.get(index);
    }

    public void quitClicked(ActionEvent event) {
        Platform.runLater(() -> {
            this.client.disconnect();
            this.service.shutdownNow();
            this.getScene().getWindow().fireEvent(new WindowEvent(this.getScene().getWindow(), WindowEvent.WINDOW_CLOSE_REQUEST));
        });
    }

    public void opponentDisconnected() {
        this.service.shutdownNow();
        Platform.runLater(() -> {
            Scene sc = new Scene(new JoinGameController(client));
            Stage stage = (Stage) this.getScene().getWindow();
            stage.setScene(sc);
            stage.show();
        });
    }

    private void circleClicked(MouseEvent e, int index) {
        errorText.setText("");
        if(e.getButton() == MouseButton.SECONDARY) {
            prevIndex = -1;
            return;
        }

        Machine.State state = client.getMachine().getState();

        if(state == State.GAME_MOVE_OPP || state == State.GAME_PUT_OPP || state == State.GAME_TAKE_OPP || state == State.GAME_OVER) {
            errorText.setText("It's not your turn!");
        }

        if(prevIndex == -1 && client.getMachine().getState() == State.GAME_MOVE) {
            prevIndex = index;
            return;
        }

        Task<Message> task = new Task<Message>() {
            @Override
            protected Message call() throws Exception {
                switch(client.getMachine().getState()) {
                    case GAME_MOVE:
                    case GAME_MOVE_OPP: {
                        System.out.println("move");
                        client.send(new Message(MessageType.PLAYER_MV, ByteBuffer.allocate(2*Integer.BYTES).putInt(prevIndex).putInt(index).array()));
                        return client.getMessage(MessageType.NOK, MessageType.OK);
                    }

                    case GAME_PUT:
                    case GAME_PUT_OPP: {
                        System.out.println("put");
                        client.send(new Message(MessageType.PLAYER_PUT, ByteBuffer.allocate(Integer.BYTES).putInt(index).array()));
                        return client.getMessage(MessageType.NOK, MessageType.OK);
                    }

                    case GAME_TAKE:
                    case GAME_TAKE_OPP: {
                        System.out.println("take");
                        client.send(new Message(MessageType.PLAYER_TAKE, ByteBuffer.allocate(Integer.BYTES).putInt(index).array()));
                        return client.getMessage(MessageType.NOK, MessageType.OK);
                    }
                }

                return null;
            }
        };

        task.setOnSucceeded(r -> {
            Message msg = task.getValue();
            switch(msg.type()) {
                case NOK: {
                    // TODO: handle rejection
                } break;
                case OK: {
                    if(state == State.GAME_TAKE) {
                        circles.get(index).setFill(Color.BLACK);
                    } else if(state == State.GAME_PUT) {
                        circles.get(index).setFill(client.getColor());
                    } else {
                        circles.get(prevIndex).setFill(Color.BLACK);
                        circles.get(index).setFill(client.getColor());
                        prevIndex = -1;
                    }

                } break;
            }
        });

        service.execute(task);
    }

}
