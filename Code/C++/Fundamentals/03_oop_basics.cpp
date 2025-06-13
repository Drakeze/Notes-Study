// Object-Oriented Programming in C++
#include <iostream>
#include <string>
using namespace std;

// Basic class definition
class Car {
private:
    string brand;
    string model;
    int year;

public:
    // Constructor
    Car(string b, string m, int y) {
        brand = b;
        model = m;
        year = y;
    }

    // Member functions
    void displayInfo() {
        cout << year << " " << brand << " " << model << endl;
    }

    // Getters and setters
    string getBrand() { return brand; }
    void setBrand(string b) { brand = b; }
    string getModel() { return model; }
    void setModel(string m) { model = m; }
    int getYear() { return year; }
    void setYear(int y) { year = y; }
};

// Inheritance example
class ElectricCar : public Car {
private:
    int batteryCapacity;

public:
    ElectricCar(string b, string m, int y, int bc) : Car(b, m, y) {
        batteryCapacity = bc;
    }

    void displayInfo() {
        Car::displayInfo();
        cout << "Battery Capacity: " << batteryCapacity << " kWh" << endl;
    }
};

// Class with static member
class Counter {
private:
    static int count;
    int id;

public:
    Counter() {
        count++;
        id = count;
    }

    static int getCount() {
        return count;
    }

    int getId() {
        return id;
    }
};

// Initialize static member
int Counter::count = 0;

int main() {
    cout << "C++ OOP Basics" << endl;

    // Creating objects
    cout << "\nBasic Class Demo:" << endl;
    Car car1("Toyota", "Camry", 2022);
    car1.displayInfo();

    // Using getters and setters
    car1.setYear(2023);
    cout << "Updated year: " << car1.getYear() << endl;

    // Inheritance demo
    cout << "\nInheritance Demo:" << endl;
    ElectricCar tesla("Tesla", "Model 3", 2023, 75);
    tesla.displayInfo();

    // Static members demo
    cout << "\nStatic Members Demo:" << endl;
    Counter c1, c2, c3;
    cout << "Total objects created: " << Counter::getCount() << endl;
    cout << "Object IDs: " << c1.getId() << ", " << c2.getId() << ", " << c3.getId() << endl;

    return 0;
}