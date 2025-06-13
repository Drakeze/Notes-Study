// Java Collections Framework and Exception Handling

import java.util.*;

/*
Topics covered:
1. Collections Framework
2. Exception Handling
3. Generic Types
*/

public class Collections_Exceptions {
    public static void main(String[] args) {
        // TODO: Practice these concepts:

        // 1. Collections Framework
        // Lists
        // - ArrayList
        // - LinkedList
        // - Vector
        
        // Sets
        // - HashSet
        // - TreeSet
        // - LinkedHashSet
        
        // Maps
        // - HashMap
        // - TreeMap
        // - LinkedHashMap
        
        // Queues
        // - PriorityQueue
        // - ArrayDeque

        // 2. Exception Handling
        // - try-catch blocks
        // - multiple catch blocks
        // - finally block
        // - throw statement
        // - throws clause
        // - custom exceptions
        // - try-with-resources

        // 3. Generic Types
        // - Generic classes
        // - Generic methods
        // - Bounded type parameters
        // - Wildcard types
    }

    // Collection examples
    public static void listExample() {
        List<String> list = new ArrayList<>();
        // Practice list operations
    }

    public static void setExample() {
        Set<Integer> set = new HashSet<>();
        // Practice set operations
    }

    public static void mapExample() {
        Map<String, Integer> map = new HashMap<>();
        // Practice map operations
    }

    // Exception handling examples
    public static void exceptionExample() {
        try {
            // Code that might throw an exception
        } catch (Exception e) {
            // Exception handling
        } finally {
            // Cleanup code
        }
    }

    // Custom exception example
    class CustomException extends Exception {
        public CustomException(String message) {
            super(message);
        }
    }

    // Generic class example
    class Box<T> {
        private T content;

        public void set(T content) {
            this.content = content;
        }

        public T get() {
            return content;
        }
    }

    // Generic method example
    public static <T> void printArray(T[] array) {
        // Practice generic method implementation
    }
}