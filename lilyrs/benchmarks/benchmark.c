#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>
#include <unistd.h>

int main() {
    char sample[] = \
    "000000000000000100000000000000000000000000000100000000000000000100000000000000000000000000010000000000000" \
    "000000000000000000010000000000000000000000000000000000000100000000000000000000000001000000000000000000010" \
    ;

    char repl[1000] = "";

    for (int i = 0; i < strlen(sample); i++) {
        if (sample[i] == '1') {
            strcat(repl, "ONE");
        } else {
            int len = strlen(repl);
            repl[len] = sample[i];
        }
    }

    int nums[] = {
        1.0, 4.0, 9.0, 16.0, 25.0, 36.0, 49.0, 
        64.0, 81.0, 100.0, 121.0, 144.0, 
        169.0, 196.0, 225.0, 256.0
    };

    for (int j = 0; j < 200; j++) {
        for (int i = 0; i < 16; i++) {
            sqrt(nums[i]);
        }
    }

    puts("C: Completed all iterations."); 
}