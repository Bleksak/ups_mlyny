package mlyny.model;

import mlyny.Main;

public class PingSpammer extends Thread {
    private static final long TIMEOUT = 5000;
    private long m_start = System.currentTimeMillis();
    private static boolean m_response = false;
    private static boolean m_shutdown = false;

    private final Client m_client;

    public PingSpammer(Client client) {
        m_client = client;
    }

    public void run() {
        while(true) {
            m_response = false;

            while(!m_response) {
                if(m_shutdown) {
                    return;
                }

                m_client.ping();

                if(System.currentTimeMillis() - m_start >= TIMEOUT) {
                    // connection lost, go to connecting view
                    Main.showConnectingView();
                    return;
                }

                try {
                    Thread.sleep(500);
                } catch(InterruptedException e) {
                    Thread.currentThread().interrupt();
                }
            }
            
            try {
                // sleep for 5s then repeat ping
                Thread.sleep(TIMEOUT);
            } catch(InterruptedException e) {

            }

            m_start = System.currentTimeMillis();
        }
    }

    public static void pong() {
        m_response = true;
    }

    public static void shutdown() {
        m_shutdown = true;
    }
}
