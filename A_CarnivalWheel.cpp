#include <iostream>
#include <vector>


int main() {
    
    int t{};
    std::cin >> t;

    std::vector<int> vec{};

    int l{}, a{}, b{};

    for(int i{0}; i < t; i++) {
        std::cin >> l >> a >> b;
        
        int j{1}, initial{a}, max{a};
        while(true) {
            a = (a+b)%l;
            if(max < a) max = a;
            if(a == initial) break;
        }
        vec.push_back(max);
    }

    for(int val: vec) {
        std::cout << val << '\n';
    }

    return 0;
}
