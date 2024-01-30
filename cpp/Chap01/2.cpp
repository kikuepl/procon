#include <atcoder/dsu>
#include <cstdio>

using namespace std;
using namespace atcoder;

#define rep(i, n) for (int i = 0; i < (int)(n); i++)

int main() {
    int n,x;
    scanf("%d %d", &n, &x);
    rep(i,n) {
        int a;
        scanf("%d", &a);
        if (a==x) {
            printf("Yes");
            return 0;
        } 
    }
    printf("No");
    return 0;
}