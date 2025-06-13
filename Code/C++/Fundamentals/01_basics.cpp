// Basic C++ concepts
#include <iostream>
using namespace std;

// Function declarations
void variables();
void operators();
void controlFlow();

int main() {
    cout << "C++ Fundamentals - Basics" << endl;
    
    // Call the practice functions
    variables();
    operators();
    controlFlow();
    
    return 0;
}

void variables() {
    // Data types
    int number = 42;
    float decimal = 3.14f;
    double precise = 3.14159;
    char letter = 'A';
    bool isTrue = true;
    string text = "Hello C++";
    
    // Output variables
    cout << "\nVariables Demo:" << endl;
    cout << "Integer: " << number << endl;
    cout << "Float: " << decimal << endl;
    cout << "Double: " << precise << endl;
    cout << "Character: " << letter << endl;
    cout << "Boolean: " << isTrue << endl;
    cout << "String: " << text << endl;
}

void operators() {
    cout << "\nOperators Demo:" << endl;
    
    // Arithmetic operators
    int a = 10, b = 3;
    cout << "Addition: " << a + b << endl;
    cout << "Subtraction: " << a - b << endl;
    cout << "Multiplication: " << a * b << endl;
    cout << "Division: " << a / b << endl;
    cout << "Modulus: " << a % b << endl;
    
    // Comparison operators
    cout << "Is a > b? " << (a > b) << endl;
    cout << "Is a == b? " << (a == b) << endl;
}

void controlFlow() {
    cout << "\nControl Flow Demo:" << endl;
    
    // If-else statement
    int number = 15;
    if (number > 10) {
        cout << number << " is greater than 10" << endl;
    } else {
        cout << number << " is less than or equal to 10" << endl;
    }
    
    // For loop
    cout << "Counting from 1 to 5: ";
    for (int i = 1; i <= 5; i++) {
        cout << i << " ";
    }
    cout << endl;
    
    // While loop
    int count = 0;
    cout << "While loop demo: ";
    while (count < 3) {
        cout << "*";
        count++;
    }
    cout << endl;
}