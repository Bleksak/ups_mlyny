package mlyny.model;

import java.io.IOException;

import mlyny.Main;

public class PingSpammer extends Thread {
    private static final long TIMEOUT = 5000;
    private long m_start = System.currentTimeMillis();
    private static boolean m_response = false;

    public void run() {
        while(true) {
            m_response = false;
            
            try {
                Client.getInstance().ping();
            } catch (IOException e) {}

            while(!m_response) {

                if(System.currentTimeMillis() - m_start >= TIMEOUT) {
                    // connection lost, go to connecting view
                    Main.showConnectingView();
                    return;
                }

                try {
                    Thread.sleep(20);
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
}
