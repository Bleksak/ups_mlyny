package mlyny.model;

import java.util.concurrent.ConcurrentLinkedQueue;

import mlyny.Main;

public class Receiver extends Thread {

    private final ConcurrentLinkedQueue<Message> m_queue = new ConcurrentLinkedQueue<>();
    private final Client m_client;

    public Receiver(Client client) {
        m_client = client;
        start();
    }

    public void pushMessage(Message message) {
        System.out.println("adding: " + message.type().name());
        m_queue.add(message);
    }

    public void run() {
        while(true) {
            while(!m_queue.isEmpty()) {
                System.out.println("received a message!");
                Message message = m_queue.remove();

                Main.getController().receivedMessage(message);

                switch(message.type()) {
                    case NOK:
                        break;
                    case OK:
                        break;
                    case PING:
                        break;
                    case PLAYER_INIT_CREATE:
                        break;
                    case PLAYER_INIT_JOIN:
                        break;
                    case PLAYER_MV:
                        break;
                    case PLAYER_PUT:
                        break;
                    case PLAYER_TAKE:
                        break;
                    case PONG:
                        System.out.println("pong!");
                        PingSpammer.pong();
                        break;
                    case PLAYER_INIT_USERNAME_INVALID:
                        break;
                    case PLAYER_INIT_USERNAME_USED:
                        break;
                    case OVER:
                        break;
                    case READY:
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
}
