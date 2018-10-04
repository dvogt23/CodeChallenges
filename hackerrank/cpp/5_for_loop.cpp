#include <iostream>
#include <cstdio>
using namespace std;

/*
    Sample input:
    8
    11
    
    Sample output:
    eight
    nine
    even
    odd
*/
string getNumberAsText(int i) {
    if(i == 1)
        return "one";
    else if(i == 2)
        return "two";
    else if(i == 3)
        return "three";
    else if(i == 4)
        return "four";
    else if(i == 5)
        return "five";
    else if(i == 6)
        return "six";
    else if(i == 7)
        return "seven";
    else if(i == 8)
        return "eight";
    else if(i == 9)
        return "nine";
    
    return "";
}

int main() {
    // Complete the code.
    int a, b;
    scanf("%i\n%i", &a, &b);

    for(int i = a; i <= b; i++) {
        if(i > 9) {
            if(i % 2 == 0)
                printf("even\n");
             else
                 printf("odd\n");
            continue;
        }
        
        cout << getNumberAsText(i) << endl;
    }
    
    return 0;
}
