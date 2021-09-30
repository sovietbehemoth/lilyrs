#include <stdio.h>
#include <stdlib.h>
#include <time.h>

#if defined(WIN_32) || defined(_WIN32)
    #error This benchmark utilizes a unix exclusive threading library.
#endif

#include <unistd.h>
#include <pthread.h>

#define C 0
#define CPP 1
#define RUST 2
#define PYTHON 3
#define JS 4

void *cthread(void *vargp) {
    system("./cbenchmark");
    return NULL;
}

void *cppthread(void *vargp) {
    system("./cppbenchmark");
    return NULL;
}

void *rsthread(void *vargp) {
    system("./benchmark");
    return NULL;
}

void *pythread(void *vargp) {
    system("python3 benchmark.py");
    return NULL;
}

void *jsthread(void *vargp) {
    system("node benchmark.js");
    return NULL;
}

int main() {

    system("gcc ./benchmark.c -lm -o cbenchmark");
    system("rustc ./benchmark.rs");
    system("g++ ./benchmark.cpp -o cppbenchmark");


    pthread_t CThread, CPPThread, RustThread, PythonThread, JSThread;

    pthread_create(&CThread, NULL, cthread, NULL);
    pthread_create(&CPPThread, NULL, cppthread, NULL);
    pthread_create(&RustThread, NULL, rsthread, NULL);
    pthread_create(&PythonThread, NULL, pythread, NULL);
    pthread_create(&JSThread, NULL, jsthread, NULL);
    

    pthread_join(RustThread, NULL);
    pthread_join(CPPThread, NULL);
    pthread_join(CThread, NULL);
    pthread_join(PythonThread, NULL);
    pthread_join(JSThread, NULL);

}