// Modern C++ Features (C++11 and beyond)
#include <iostream>
#include <memory>
#include <vector>
#include <thread>
#include <mutex>
using namespace std;

// Smart Pointers demo
void smartPointersDemo() {
    cout << "\nSmart Pointers Demo:" << endl;

    // unique_ptr - exclusive ownership
    unique_ptr<int> uniqPtr = make_unique<int>(42);
    cout << "unique_ptr value: " << *uniqPtr << endl;

    // shared_ptr - shared ownership
    shared_ptr<string> sharedPtr1 = make_shared<string>("Hello");
    shared_ptr<string> sharedPtr2 = sharedPtr1; // Both point to same object
    cout << "shared_ptr value: " << *sharedPtr1 << endl;
    cout << "Reference count: " << sharedPtr1.use_count() << endl;

    // weak_ptr - weak reference
    weak_ptr<string> weakPtr = sharedPtr1;
    if (auto temp = weakPtr.lock()) {
        cout << "weak_ptr value: " << *temp << endl;
    }
}

// Move semantics and rvalue references
class Resource {
public:
    Resource() { cout << "Resource constructed" << endl; }
    ~Resource() { cout << "Resource destroyed" << endl; }
    
    // Move constructor
    Resource(Resource&& other) noexcept {
        cout << "Resource moved" << endl;
    }
};

// Lambda expressions with capture modes
void lambdaFeaturesDemo() {
    cout << "\nModern Lambda Features Demo:" << endl;

    int multiplier = 10;
    vector<int> numbers = {1, 2, 3, 4, 5};

    // Lambda with capture by value
    auto multiplyByVal = [multiplier](int n) { return n * multiplier; };

    // Lambda with capture by reference
    auto multiplyByRef = [&multiplier](int n) { return n * multiplier; };

    // Generic lambda (C++14)
    auto genericLambda = [](auto x, auto y) { return x + y; };

    cout << "Generic lambda result: " << genericLambda(5, 3.14) << endl;
}

// Threading example
mutex mtx;
void threadFunction(int id) {
    lock_guard<mutex> lock(mtx);
    cout << "Thread " << id << " executing" << endl;
}

void threadingDemo() {
    cout << "\nThreading Demo:" << endl;

    vector<thread> threads;
    for(int i = 0; i < 3; ++i) {
        threads.emplace_back(threadFunction, i);
    }

    for(auto& t : threads) {
        t.join();
    }
}

// Variadic templates
template<typename T>
void print(T t) {
    cout << t << endl;
}

template<typename T, typename... Args>
void print(T t, Args... args) {
    cout << t << " ";
    print(args...);
}

// constexpr functions
constexpr int factorial(int n) {
    return n <= 1 ? 1 : n * factorial(n - 1);
}

int main() {
    cout << "Modern C++ Features" << endl;

    smartPointersDemo();

    cout << "\nMove Semantics Demo:" << endl;
    Resource r1;
    Resource r2 = move(r1); // Move constructor called

    lambdaFeaturesDemo();
    threadingDemo();

    cout << "\nVariadic Templates Demo:" << endl;
    print(1, 2.5, "Hello", 'A');

    cout << "\nconstexpr Demo:" << endl;
    constexpr int fact5 = factorial(5);
    cout << "Factorial of 5 (computed at compile time): " << fact5 << endl;

    return 0;
}