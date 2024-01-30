#include <atcoder/dsu>
#include <cstdio>

using namespace std;
using namespace atcoder;

#define rep(i, n) for (int i = 0; i < (int)(n); i++)

int main() {
    int n;
    scanf("%d", &n);
    for (int i = 9; 0<=i; i--) {
        int wari = 1<<i;
        printf("%d", (n/wari)%2);
    }
}