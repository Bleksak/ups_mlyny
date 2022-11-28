package mlyny.model;

import java.nio.ByteBuffer;
import java.util.Arrays;
import java.util.concurrent.ConcurrentLinkedQueue;

import javafx.application.Platform;
import mlyny.Main;
import mlyny.controller.GameController;

public class Receiver extends Thread {

    private final ConcurrentLinkedQueue<Message> m_queue = new ConcurrentLinkedQueue<>();
    // private final Client m_client;
    private MState m_state;

    public Receiver(Client client) {
        // m_client = client;
        m_state = MState.INIT;
        start();
    }

    public void pushMessage(Message message) {
        if(message.type() != MessageType.PING)
            System.out.println("adding: " + message.type().name());

        m_queue.add(message);
    }

    public void run() {
        while (true) {
            while (!m_queue.isEmpty()) {
                Message message = m_queue.remove();
                if(message.type() != MessageType.PONG)
                    System.out.println("received " + message.type().name());

                // Platform.runLater( () -> Main.getController().receivedMessage(message));
                if (message.type() == MessageType.PONG) {
                    PingSpammer.pong();
                    continue;
                }

                switch (m_state) {
                    case GAME_MOVE:
                        move(message);
                        break;
                    case GAME_OVER:
                        over(message);
                        break;
                    case GAME_PUT:
                        put(message);
                        break;
                    case GAME_TAKE:
                        take(message);
                        break;
                    case INIT:
                        init(message);
                        break;
                    case LOBBY:
                        init(message);
                        break;
                }
            }

            try {
                Thread.sleep(20);
            } catch (InterruptedException e) {
            }
        }

    }

    private void init(Message msg) {

        if(msg.type() == MessageType.PLAYER_JOIN_NOTIFY) {
            System.out.println("data length: "+msg.data().length);
            ByteBuffer intBuffer = ByteBuffer.wrap(msg.data());
            MState s = MState.fromInt(intBuffer.getInt());

            if(s != MState.BULLSHIT) {
                if(s == MState.LOBBY) {
                    Platform.runLater( () -> {
                        Main.setRoot("controller/CreateGameView");
                    });
                } else {
                    LColor color = LColor.valueOf(msg.data()[Integer.BYTES]);
                    Board  board = new Board(Arrays.copyOfRange(msg.data(), 1 + Integer.BYTES, msg.data().length));
                    Main.setRoot("controller/GameView");
                    while(!(Main.getController() instanceof GameController)) {}
                    GameController ctrl = (GameController) Main.getController();
                    ctrl.setBoard(board);
                    ctrl.setColor(color);
                }

                m_state = s;
            }

        }

        if(msg.type() == MessageType.NOK) {
            System.out.println("FUCK OFF");
        }
    }

    private void lobby(Message msg) {
        if(msg.type() == MessageType.PLAYER_JOIN_NOTIFY) {
            System.out.println("go to game?");

            LColor color = LColor.valueOf(msg.data()[0]);
            Board  board = new Board(Arrays.copyOfRange(msg.data(), 1, msg.data().length));

            Platform.runLater( () -> {
                Main.setRoot("controller/GameView");
                GameController ctrl = (GameController) Main.getController();
                ctrl.setBoard(board);
                ctrl.setColor(color);
            });

            m_state = MState.GAME_PUT;
        }

        if(msg.type() == MessageType.NOK) {

        }
    }

    private void move(Message msg) {
        if(msg.type() == MessageType.OK) {
            
        }

        if(msg.type() == MessageType.NOK) {

        }
    }

    private void take(Message msg) {
        if(msg.type() == MessageType.OK) {
            
        }

        if(msg.type() == MessageType.NOK) {

        }
    }

    private void put(Message msg) {
        if(msg.type() == MessageType.OK) {
            GameController ctrl = (GameController) Main.getController();
            ctrl.confirmLastOperation();
        }

        if(msg.type() == MessageType.NOK) {
            
        }
    }


    private void over(Message msg) {

    }
}
