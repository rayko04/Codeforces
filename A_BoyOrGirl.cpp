#include <iostream>
#include <string>
#include <unordered_map>



int main() {

    std::string input{};
    std::cin >> input;

    std::unordered_map<char, int> freq{};

    for(char ch: input) {
        freq[ch]++;
    }

    if(freq.size() % 2 == 0)
        std::cout << "CHAT WITH HER!";
    else
        std::cout << "IGNORE HIM!";

    return 0;
}
