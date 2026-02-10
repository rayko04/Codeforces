#include <iostream>
#include <vector>

void sol(long long a, long long b, long long n, std::vector<int>& res) {
    if(b*n > a && b < a)
        res.push_back(2);
    else
        res.push_back(1);
}

int main() {

    int t{};
    std::cin >> t;

    std::vector<int> res{};
    res.reserve(t);

    for(int i{0}; i < t; i++) {
        long long a{}, b{}, n{};
        std::cin >> a >> b >> n;

        sol(a, b, n, res);
    }

    for(int ans: res) {
        std::cout << ans << std::endl;
    }
}
