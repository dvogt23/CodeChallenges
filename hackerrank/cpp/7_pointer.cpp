/* 
   Sample input:
   4
   5  
   
   Sample output:
   9    # sum
   1    # diff
*/
#include <stdio.h>

void update(int *a,int *b) {
    int sum = *a + *b;
    int diff = 0;
    if(a > b) {
       diff = *a % *b; 
    } else {
        diff = *b % *a;
    }

    *a = sum;
    *b = diff;
}

int main() {
    int a, b;
    int *pa = &a, *pb = &b;
    
    scanf("%d %d", &a, &b);
    update(pa, pb);
    printf("%d\n%d", a, b);

    return 0;
}
