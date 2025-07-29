// Java OOP Concepts

/*
Topics covered:
1. Classes and Objects
2. Inheritance
3. Polymorphism
4. Encapsulation
5. Abstraction
6. Interfaces
*/

// Base class example
public class OOP_Concepts {
    public static void main(String[] args) {
        // TODO: Practice these concepts:

        // 1. Classes and Objects
        // - Class definition
        // - Object creation
        // - Constructors
        // - Instance variables and methods
        // - Static variables and methods

        // 2. Inheritance
        // - Extending classes
        // - Method overriding
        // - super keyword
        // - final keyword

        // 3. Polymorphism
        // - Method overloading
        // - Method overriding
        // - Runtime polymorphism
        // - Type casting

        // 4. Encapsulation
        // - Access modifiers (public, private, protected)
        // - Getters and setters
        // - Package access

        // 5. Abstraction
        // - Abstract classes
        // - Abstract methods
        // - Implementation details hiding

        // 6. Interfaces
        // - Interface definition
        // - Implementation
        // - Multiple interfaces
        // - Default methods
    }
}

// Example class templates:
class Animal {
    // Base class implementation
    protected String name;
    
    public Animal(String name) {
        this.name = name;
    }
    
    public void makeSound() {
        // Method to be overridden
    }
}

class Dog extends Animal {
    // Inherited class implementation
    public Dog(String name) {
        super(name);
    }
    
    @Override
    public void makeSound() {
        // Override base class method
    }
}

interface Movable {
    // Interface methods
    void move();
    void stop();
}

abstract class Vehicle implements Movable {
    // Abstract class implementation
    protected String type;
    
    public abstract void startEngine();
}