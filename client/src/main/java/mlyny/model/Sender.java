package mlyny.model;

import java.io.IOException;
import java.util.concurrent.ConcurrentLinkedQueue;

public class Sender extends Thread {
    private final ConcurrentLinkedQueue<Message> m_queue = new ConcurrentLinkedQueue<>();
    private final Client m_client;
    
    public Sender(Client client) {
        m_client = client;
        start();
    }

    public void pushMessage(Message msg) {
        m_queue.add(msg);
    }

    public void run() {
        while(true) {
            while(!m_queue.isEmpty() && m_client.running()) {
                System.out.println("sending message");
                try {
                    m_client.sendMessage(m_queue.remove());
                } catch (IOException e) {
                    System.out.println("failed to send a messsage");
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
