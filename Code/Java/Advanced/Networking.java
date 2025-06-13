// Java Networking and Web Services

import java.net.*;
import java.io.*;
import javax.net.ssl.*;

/*
Topics covered:
1. Basic Networking
2. Socket Programming
3. URL Processing
4. HTTP Client
5. SSL/TLS
*/

public class Networking {
    public static void main(String[] args) {
        // TODO: Practice these concepts:

        // 1. Basic Networking
        // - InetAddress
        // - Network interfaces
        // - IP addresses
        // - Host name resolution

        // 2. Socket Programming
        // - TCP Sockets
        // - UDP Sockets
        // - Server sockets
        // - Client sockets

        // 3. URL Processing
        // - URL class
        // - URLConnection
        // - Reading from URLs
        // - Parsing URLs

        // 4. HTTP Client
        // - HttpURLConnection
        // - HTTP methods
        // - Headers
        // - Request/Response handling

        // 5. SSL/TLS
        // - SSLSocket
        // - SSLServerSocket
        // - Certificates
        // - Trust managers
    }

    // Basic networking examples
    public static void networkingBasics() {
        try {
            // Get local host information
            InetAddress localhost = InetAddress.getLocalHost();
            
            // Get IP addresses for a host
            InetAddress[] addresses = InetAddress.getAllByName("example.com");
            
            // List network interfaces
            NetworkInterface.getNetworkInterfaces().asIterator().forEachRemaining(ni -> {
                // Process network interface
            });
        } catch (UnknownHostException | SocketException e) {
            e.printStackTrace();
        }
    }

    // TCP Socket server example
    static class TCPServer {
        public void start(int port) {
            try (ServerSocket serverSocket = new ServerSocket(port)) {
                while (true) {
                    Socket clientSocket = serverSocket.accept();
                    handleClient(clientSocket);
                }
            } catch (IOException e) {
                e.printStackTrace();
            }
        }

        private void handleClient(Socket clientSocket) {
            // Handle client connection
        }
    }

    // TCP Socket client example
    static class TCPClient {
        public void connect(String host, int port) {
            try (Socket socket = new Socket(host, port)) {
                // Communicate with server
            } catch (IOException e) {
                e.printStackTrace();
            }
        }
    }

    // URL processing example
    public static void urlProcessing() {
        try {
            URL url = new URL("https://example.com");
            URLConnection connection = url.openConnection();
            
            // Read from URL
            try (BufferedReader reader = new BufferedReader(
                    new InputStreamReader(connection.getInputStream()))) {
                // Process URL content
            }
        } catch (IOException e) {
            e.printStackTrace();
        }
    }

    // HTTP client example
    public static void httpClient() {
        try {
            URL url = new URL("https://api.example.com");
            HttpURLConnection conn = (HttpURLConnection) url.openConnection();
            
            // Configure request
            conn.setRequestMethod("GET");
            conn.setRequestProperty("Accept", "application/json");
            
            // Handle response
            try (BufferedReader reader = new BufferedReader(
                    new InputStreamReader(conn.getInputStream()))) {
                // Process response
            }
        } catch (IOException e) {
            e.printStackTrace();
        }
    }

    // SSL/TLS example
    static class SSLExample {
        public void secureServer() {
            try {
                // Initialize SSL context
                SSLContext sslContext = SSLContext.getInstance("TLS");
                // Configure context with key managers and trust managers
                
                // Create SSL server socket
                SSLServerSocketFactory factory = sslContext.getServerSocketFactory();
                try (SSLServerSocket serverSocket = 
                        (SSLServerSocket) factory.createServerSocket(8443)) {
                    // Handle secure connections
                }
            } catch (Exception e) {
                e.printStackTrace();
            }
        }
    }
}