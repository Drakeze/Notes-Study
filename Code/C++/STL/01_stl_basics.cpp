// STL Basics in C++
#include <iostream>
#include <vector>
#include <list>
#include <map>
#include <set>
#include <algorithm>
#include <string>
using namespace std;

// Function prototypes
void vectorDemo();
void listDemo();
void mapDemo();
void setDemo();
void algorithmDemo();

int main() {
    cout << "C++ STL Basics" << endl;

    vectorDemo();
    listDemo();
    mapDemo();
    setDemo();
    algorithmDemo();

    return 0;
}

void vectorDemo() {
    cout << "\nVector Demo:" << endl;

    // Creating and initializing vector
    vector<int> numbers = {1, 2, 3, 4, 5};

    // Adding elements
    numbers.push_back(6);
    numbers.push_back(7);

    // Accessing elements
    cout << "First element: " << numbers[0] << endl;
    cout << "Last element: " << numbers.back() << endl;

    // Iterating through vector
    cout << "All elements: ";
    for(const auto& num : numbers) {
        cout << num << " ";
    }
    cout << endl;

    // Vector size and capacity
    cout << "Size: " << numbers.size() << endl;
    cout << "Capacity: " << numbers.capacity() << endl;
}

void listDemo() {
    cout << "\nList Demo:" << endl;

    // Creating and initializing list
    list<string> fruits = {"apple", "banana", "orange"};

    // Adding elements
    fruits.push_back("grape");
    fruits.push_front("mango");

    // Iterating through list
    cout << "All fruits: ";
    for(const auto& fruit : fruits) {
        cout << fruit << " ";
    }
    cout << endl;

    // Remove an element
    fruits.remove("banana");
    cout << "After removing 'banana': ";
    for(const auto& fruit : fruits) {
        cout << fruit << " ";
    }
    cout << endl;
}

void mapDemo() {
    cout << "\nMap Demo:" << endl;

    // Creating and initializing map
    map<string, int> ages = {
        {"John", 25},
        {"Jane", 22},
        {"Bob", 30}
    };

    // Adding elements
    ages["Alice"] = 28;

    // Accessing elements
    cout << "John's age: " << ages["John"] << endl;

    // Checking if key exists
    string name = "Alice";
    if(ages.find(name) != ages.end()) {
        cout << name << "'s age: " << ages[name] << endl;
    }

    // Iterating through map
    cout << "All entries:" << endl;
    for(const auto& pair : ages) {
        cout << pair.first << ": " << pair.second << endl;
    }
}

void setDemo() {
    cout << "\nSet Demo:" << endl;

    // Creating and initializing set
    set<int> numbers = {3, 1, 4, 1, 5, 9, 2, 6, 5};

    // Set automatically sorts and removes duplicates
    cout << "Unique sorted numbers: ";
    for(const auto& num : numbers) {
        cout << num << " ";
    }
    cout << endl;

    // Adding elements
    numbers.insert(7);

    // Checking if element exists
    int searchNum = 4;
    if(numbers.find(searchNum) != numbers.end()) {
        cout << searchNum << " exists in the set" << endl;
    }
}

void algorithmDemo() {
    cout << "\nAlgorithm Demo:" << endl;

    vector<int> numbers = {64, 34, 25, 12, 22, 11, 90};

    // Sorting
    sort(numbers.begin(), numbers.end());
    cout << "Sorted numbers: ";
    for(const auto& num : numbers) {
        cout << num << " ";
    }
    cout << endl;

    // Finding maximum and minimum
    auto minMax = minmax_element(numbers.begin(), numbers.end());
    cout << "Minimum: " << *minMax.first << endl;
    cout << "Maximum: " << *minMax.second << endl;

    // Binary search
    int searchNum = 25;
    bool found = binary_search(numbers.begin(), numbers.end(), searchNum);
    cout << searchNum << (found ? " found" : " not found") << endl;

    // Count occurrences
    vector<int> nums = {1, 2, 3, 2, 4, 2, 5};
    int count = std::count(nums.begin(), nums.end(), 2);
    cout << "Number 2 appears " << count << " times" << endl;
}