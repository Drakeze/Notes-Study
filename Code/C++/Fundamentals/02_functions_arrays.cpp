// Functions and Arrays in C++
#include <iostream>
using namespace std;

// Function prototypes
int add(int a, int b);
int subtract(int a, int b);
void printArray(int arr[], int size);
void arrayOperations();
void functionOverloading();
double add(double a, double b);  // Overloaded function

int main() {
    cout << "C++ Functions and Arrays" << endl;

    // Function calls
    int result1 = add(5, 3);
    cout << "\nFunction Demo:" << endl;
    cout << "5 + 3 = " << result1 << endl;
    cout << "5 - 3 = " << subtract(5, 3) << endl;

    // Function overloading demo
    functionOverloading();

    // Array operations
    arrayOperations();

    return 0;
}

// Function definitions
int add(int a, int b) {
    return a + b;
}

int subtract(int a, int b) {
    return a - b;
}

double add(double a, double b) {
    return a + b;
}

void printArray(int arr[], int size) {
    for(int i = 0; i < size; i++) {
        cout << arr[i] << " ";
    }
    cout << endl;
}

void arrayOperations() {
    cout << "\nArray Operations Demo:" << endl;

    // Array declaration and initialization
    int numbers[5] = {1, 2, 3, 4, 5};
    
    // Print original array
    cout << "Original array: ";
    printArray(numbers, 5);

    // Modifying array elements
    numbers[2] = 10;
    cout << "After modifying element at index 2: ";
    printArray(numbers, 5);

    // 2D array example
    int matrix[2][3] = {
        {1, 2, 3},
        {4, 5, 6}
    };

    cout << "\n2D Array (Matrix):" << endl;
    for(int i = 0; i < 2; i++) {
        for(int j = 0; j < 3; j++) {
            cout << matrix[i][j] << " ";
        }
        cout << endl;
    }
}

void functionOverloading() {
    cout << "\nFunction Overloading Demo:" << endl;
    
    // Using overloaded functions
    int num1 = 5, num2 = 3;
    double d1 = 3.14, d2 = 2.0;

    cout << "Integer addition: " << add(num1, num2) << endl;
    cout << "Double addition: " << add(d1, d2) << endl;
}