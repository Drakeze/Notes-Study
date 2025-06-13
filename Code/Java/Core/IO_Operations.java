// Java I/O Operations and File Handling

import java.io.*;
import java.nio.file.*;

/*
Topics covered:
1. File I/O
2. Streams
3. NIO.2 API
4. Serialization
*/

public class IO_Operations {
    public static void main(String[] args) {
        // TODO: Practice these concepts:

        // 1. File I/O
        // - File class operations
        // - FileReader and FileWriter
        // - BufferedReader and BufferedWriter
        // - Scanner class
        // - PrintWriter

        // 2. Streams
        // - InputStream and OutputStream
        // - FileInputStream and FileOutputStream
        // - DataInputStream and DataOutputStream
        // - ObjectInputStream and ObjectOutputStream

        // 3. NIO.2 API
        // - Path interface
        // - Files class
        // - DirectoryStream
        // - File attributes
        // - File walking

        // 4. Serialization
        // - Serializable interface
        // - transient keyword
        // - Custom serialization
        // - Version control
    }

    // File operations examples
    public static void fileOperationsExample() {
        try {
            // Basic file operations
            File file = new File("example.txt");
            
            // Create new file
            if (file.createNewFile()) {
                System.out.println("File created");
            }
            
            // Write to file
            try (FileWriter writer = new FileWriter(file)) {
                // Practice file writing
            }
            
            // Read from file
            try (FileReader reader = new FileReader(file)) {
                // Practice file reading
            }
        } catch (IOException e) {
            e.printStackTrace();
        }
    }

    // Stream examples
    public static void streamExample() {
        try (FileInputStream fis = new FileInputStream("input.txt");
             FileOutputStream fos = new FileOutputStream("output.txt")) {
            // Practice stream operations
        } catch (IOException e) {
            e.printStackTrace();
        }
    }

    // NIO.2 examples
    public static void nio2Example() {
        try {
            Path path = Paths.get("example.txt");
            
            // Create directory
            Files.createDirectory(path.getParent());
            
            // Write file
            Files.write(path, "Hello, World!".getBytes());
            
            // Read file
            List<String> lines = Files.readAllLines(path);
            
            // Walk directory
            try (DirectoryStream<Path> stream = Files.newDirectoryStream(path.getParent())) {
                // Practice directory operations
            }
        } catch (IOException e) {
            e.printStackTrace();
        }
    }

    // Serialization example
    static class SerializableClass implements Serializable {
        private static final long serialVersionUID = 1L;
        private String data;
        private transient String tempData; // Won't be serialized

        // Practice serialization methods
        private void writeObject(ObjectOutputStream out) throws IOException {
            out.defaultWriteObject();
            // Custom serialization
        }

        private void readObject(ObjectInputStream in) throws IOException, ClassNotFoundException {
            in.defaultReadObject();
            // Custom deserialization
        }
    }
}