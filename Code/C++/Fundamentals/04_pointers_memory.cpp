// Pointers and Memory Management in C++
#include <iostream>
using namespace std;

// Function prototypes
void basicPointers();
void pointerArithmetic();
void dynamicMemory();
void referenceDemo();
void smartPointerDemo();

int main() {
    cout << "C++ Pointers and Memory Management" << endl;

    basicPointers();
    pointerArithmetic();
    dynamicMemory();
    referenceDemo();
    
    return 0;
}

void basicPointers() {
    cout << "\nBasic Pointers Demo:" << endl;

    // Basic pointer declaration and initialization
    int number = 42;
    int* ptr = &number;

    cout << "Value of number: " << number << endl;
    cout << "Address of number: " << ptr << endl;
    cout << "Value through pointer: " << *ptr << endl;

    // Modifying value through pointer
    *ptr = 100;
    cout << "Modified value: " << number << endl;

    // Null pointer
    int* nullPtr = nullptr;
    cout << "Null pointer value: " << nullPtr << endl;
}

void pointerArithmetic() {
    cout << "\nPointer Arithmetic Demo:" << endl;

    // Array and pointer arithmetic
    int numbers[] = {1, 2, 3, 4, 5};
    int* ptr = numbers;

    cout << "Array values using pointer:" << endl;
    for(int i = 0; i < 5; i++) {
        cout << "Element " << i << ": " << *ptr << endl;
        ptr++; // Move to next element
    }

    // Reset pointer to start of array
    ptr = numbers;
    cout << "\nAccessing elements using pointer arithmetic:" << endl;
    cout << "First element: " << *ptr << endl;
    cout << "Second element: " << *(ptr + 1) << endl;
    cout << "Third element: " << *(ptr + 2) << endl;
}

void dynamicMemory() {
    cout << "\nDynamic Memory Allocation Demo:" << endl;

    // Single integer allocation
    int* dynamicInt = new int;
    *dynamicInt = 42;
    cout << "Dynamically allocated integer: " << *dynamicInt << endl;
    delete dynamicInt; // Free memory

    // Dynamic array allocation
    int size = 5;
    int* dynamicArray = new int[size];

    // Initialize array
    for(int i = 0; i < size; i++) {
        dynamicArray[i] = i + 1;
    }

    cout << "Dynamic array elements: ";
    for(int i = 0; i < size; i++) {
        cout << dynamicArray[i] << " ";
    }
    cout << endl;

    delete[] dynamicArray; // Free array memory
}

void referenceDemo() {
    cout << "\nReferences Demo:" << endl;

    int number = 42;
    int& ref = number; // Reference to number

    cout << "Original value: " << number << endl;
    
    // Modify through reference
    ref = 100;
    cout << "Value after modifying reference: " << number << endl;

    // References in function parameters
    auto modifyValue = [](int& value) { value *= 2; };
    modifyValue(number);
    cout << "Value after function call: " << number << endl;
}