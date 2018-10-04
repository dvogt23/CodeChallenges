/* 
   Sample input:
   4        # number of integers 1 < N < 1000
   1 4 3 2  # integers 1 < A < 10000
   
   Sample output:
   2 3 4 1
*/
#include <cmath>
#include <cstdio>
#include <vector>
#include <iostream>
#include <algorithm>
using namespace std;

int main() {
    int n;
    scanf("%d", &n);

    vector<int> ints(n);
    string input;
    printf("size: %d", sizeof(ints));
    while(getline(cin, input, ' ')) {
        //printf("-%s", input.c_str());
        ints.insert(2);
    }

    for (it=ints.end(); it<ints.begin(); it--)
        std::cout << *it << ' ';


    return 0;
}
