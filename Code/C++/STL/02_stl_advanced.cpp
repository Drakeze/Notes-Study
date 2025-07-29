// Advanced STL Concepts in C++
#include <iostream>
#include <vector>
#include <deque>
#include <queue>
#include <stack>
#include <algorithm>
#include <functional>
#include <numeric>
using namespace std;

// Function prototypes
void dequeDemo();
void priorityQueueDemo();
void stackDemo();
void advancedAlgorithms();
void lambdaDemo();

int main() {
    cout << "C++ Advanced STL Concepts" << endl;

    dequeDemo();
    priorityQueueDemo();
    stackDemo();
    advancedAlgorithms();
    lambdaDemo();

    return 0;
}

void dequeDemo() {
    cout << "\nDeque (Double-ended Queue) Demo:" << endl;

    deque<int> numbers = {1, 2, 3, 4, 5};

    // Adding elements at both ends
    numbers.push_front(0);
    numbers.push_back(6);

    cout << "Deque elements: ";
    for(const auto& num : numbers) {
        cout << num << " ";
    }
    cout << endl;

    // Removing elements from both ends
    numbers.pop_front();
    numbers.pop_back();

    cout << "After pop operations: ";
    for(const auto& num : numbers) {
        cout << num << " ";
    }
    cout << endl;
}

void priorityQueueDemo() {
    cout << "\nPriority Queue Demo:" << endl;

    // Max heap (default)
    priority_queue<int> maxHeap;
    maxHeap.push(3);
    maxHeap.push(1);
    maxHeap.push(4);
    maxHeap.push(2);

    cout << "Max heap elements (in descending order): ";
    while(!maxHeap.empty()) {
        cout << maxHeap.top() << " ";
        maxHeap.pop();
    }
    cout << endl;

    // Min heap
    priority_queue<int, vector<int>, greater<int>> minHeap;
    minHeap.push(3);
    minHeap.push(1);
    minHeap.push(4);
    minHeap.push(2);

    cout << "Min heap elements (in ascending order): ";
    while(!minHeap.empty()) {
        cout << minHeap.top() << " ";
        minHeap.pop();
    }
    cout << endl;
}

void stackDemo() {
    cout << "\nStack Demo:" << endl;

    stack<string> books;

    // Adding elements
    books.push("Book 1");
    books.push("Book 2");
    books.push("Book 3");

    cout << "Top book: " << books.top() << endl;
    cout << "Stack size: " << books.size() << endl;

    cout << "\nRemoving books:" << endl;
    while(!books.empty()) {
        cout << "Removing: " << books.top() << endl;
        books.pop();
    }
}

void advancedAlgorithms() {
    cout << "\nAdvanced Algorithms Demo:" << endl;

    vector<int> numbers = {1, 2, 3, 4, 5, 6, 7, 8, 9, 10};

    // Partial sum
    vector<int> partialSums(numbers.size());
    partial_sum(numbers.begin(), numbers.end(), partialSums.begin());

    cout << "Partial sums: ";
    for(const auto& num : partialSums) {
        cout << num << " ";
    }
    cout << endl;

    // Adjacent difference
    vector<int> differences(numbers.size());
    adjacent_difference(numbers.begin(), numbers.end(), differences.begin());

    cout << "Adjacent differences: ";
    for(const auto& num : differences) {
        cout << num << " ";
    }
    cout << endl;

    // Transform example
    vector<int> squared(numbers.size());
    transform(numbers.begin(), numbers.end(), squared.begin(),
             [](int x) { return x * x; });

    cout << "Squared numbers: ";
    for(const auto& num : squared) {
        cout << num << " ";
    }
    cout << endl;
}

void lambdaDemo() {
    cout << "\nLambda Functions Demo:" << endl;

    vector<int> numbers = {1, 2, 3, 4, 5, 6, 7, 8, 9, 10};

    // Basic lambda
    auto isEven = [](int n) { return n % 2 == 0; };

    // Count even numbers
    int evenCount = count_if(numbers.begin(), numbers.end(), isEven);
    cout << "Number of even numbers: " << evenCount << endl;

    // Lambda with capture clause
    int threshold = 5;
    auto isGreaterThan = [threshold](int n) { return n > threshold; };

    // Count numbers greater than threshold
    int greaterCount = count_if(numbers.begin(), numbers.end(), isGreaterThan);
    cout << "Numbers greater than " << threshold << ": " << greaterCount << endl;

    // Lambda with mutable keyword
    int sum = 0;
    for_each(numbers.begin(), numbers.end(),
             [&sum](int n) { sum += n; });
    cout << "Sum of all numbers: " << sum << endl;
}