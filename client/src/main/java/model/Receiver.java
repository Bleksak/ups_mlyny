package model;

import java.util.concurrent.ConcurrentLinkedQueue;

public class Receiver extends Thread {

    private final ConcurrentLinkedQueue<Message> m_queue = new ConcurrentLinkedQueue<>();
    private final Client m_client;

    public Receiver(Client client) {
        m_client = client;
    }

    public void pushMessage(Message message) {
        m_queue.add(message);
    }

    public void run() {
        while(m_client.running()) {
            while(!m_queue.isEmpty()) {
                Message message = m_queue.remove();
            }

            try {
                Thread.sleep(20);
            } catch (InterruptedException e) {
                Thread.currentThread().interrupt();
            }
        }
    }
}
