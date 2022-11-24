package mlyny.model;

import java.util.concurrent.ConcurrentLinkedQueue;

import javafx.application.Platform;
import mlyny.Main;

public class Receiver extends Thread {

    private final ConcurrentLinkedQueue<Message> m_queue = new ConcurrentLinkedQueue<>();
    private final Client m_client;
    private MState m_state;

    public Receiver(Client client) {
        m_client = client;
        m_state = MState.INIT;
        start();
    }

    public void pushMessage(Message message) {
        System.out.println("adding: " + message.type().name());
        m_queue.add(message);
    }

    public void run() {
        while (true) {
            while (!m_queue.isEmpty()) {
                System.out.println("received a message!");
                Message message = m_queue.remove();

                // Platform.runLater( () -> Main.getController().receivedMessage(message));
                if (message.type() == MessageType.PING) {
                    System.out.println("pong!");
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
                        lobby(message);
                        break;
                }
            }

            try {
                Thread.sleep(20);
            } catch (InterruptedException e) {
                Thread.currentThread().interrupt();
            }
        }

    }

    private void init(Message msg) {
        if(msg.type() == MessageType.PLAYER_JOIN_NOTIFY) {
            m_state = MState.LOBBY;
        }

        if(msg.type() == MessageType.NOK) {
            System.out.println("FUCK OFF");
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
            
        }

        if(msg.type() == MessageType.NOK) {

        }
    }

    private void lobby(Message msg) {
        if(msg.type() == MessageType.PLAYER_JOIN_NOTIFY) {
            m_state = MState.GAME_PUT;
        }

        if(msg.type() == MessageType.NOK) {

        }
    }

    private void over(Message msg) {}
}
