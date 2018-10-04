#include <iostream>
#include <cstdio>
using namespace std;

/*
    Sample input:
    3 12345678912345 a 334.23 14049.30493
    
    Sample output:
    3
    12345678912345
    a
    334.230
    14049.304930000
*/
int main() {
    // Complete the code.
    int i;
    long l;
    char c;
    float f;
    double d;
    scanf("%i %li %c %f %lf", &i, &l, &c, &f, &d);
    printf("%i\n%li\n%c\n%.3f\n%.9lf", i, l, c, f, d);
    return 0;
}
