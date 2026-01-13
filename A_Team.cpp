#include <iostream>
#include <vector>


int main() {

    int n;
    std::cin >> n;

    int count{0};

    for(int i{0}; i < n; i++) {
        int sum{0};
        for(int j{0}; j < 3; j++) {
            int abc{};
            std::cin >> abc;
            sum += abc;
        }
        if(sum > 1) count++;
    }

    std::cout << count;

    return 0;
}
