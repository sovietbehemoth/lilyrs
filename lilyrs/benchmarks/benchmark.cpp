#include <iostream>
#include <string>
#include <array>
#include <math.h>
#include <time.h>

using namespace std;

int main() {

    string sample = \
        "000000000000000100000000000000000000000000000100000000000000000100000000000000000000000000010000000000000" \
        "000000000000000000010000000000000000000000000000000000000100000000000000000000000001000000000000000000010" \
        ;

    string repl = "";

    for (int i = 0; i < repl.length(); i++) {
        if (sample[i] == '1') {
            repl += "ONE";
        } else {
            repl += sample[i];
        }
    }

    array<int,16> nums = {
        1, 4, 9, 16, 25, 36, 49, 
        64, 81, 100, 121, 144, 
        169, 196, 225, 256
    };


    for (int j = 0; j < 200; j++) {
        for (int i = 0; i < 16; i++) {
            sqrt(nums[i]);
        }
    }

    cout << "C++: Completed all iterations.\n";
}