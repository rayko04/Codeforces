#include <iostream>
#include <string>
#include <vector>

std::string sol(const std::vector<std::string>& a) {
    std::string s{};

    for(const auto& str: a) {
        if (str + s < s + str)    s = str+s;
        else            s += str;
    }

    return s;
}

int main() {

    int t{};
    std::cin >> t;
    std::vector<std::string> res;
    res.reserve(t);

    for (int j{t}; j > 0; j--) {

        int n{};
        std::cin >> n;

        std::vector<std::string> a(n);
        for(int i{0}; i < n; i++) {
            std::cin >> a[i];
        }

        res.push_back(sol(a));
    }

    for(auto& str: res) {
        std::cout << str << std::endl;
    }


    return 0;
}
