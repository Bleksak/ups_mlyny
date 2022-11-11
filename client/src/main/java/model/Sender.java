package model;

import java.util.concurrent.ConcurrentLinkedQueue;

public class Sender extends Thread {
    private final ConcurrentLinkedQueue<Message> m_queue = new ConcurrentLinkedQueue<>();
    private final Client m_client;
    
    public Sender(Client client) {
        m_client = client;
    }

    public void run() {
        while(true) {
            while(!m_queue.isEmpty()) {
                m_client.sendMessage(m_queue.remove());
            }

            try {
                Thread.sleep(20);
            } catch (InterruptedException e) {
                Thread.currentThread().interrupt();
            }
        }
    }

}
