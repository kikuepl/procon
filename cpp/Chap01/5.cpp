#include <atcoder/dsu>
#include <cstdio>

using namespace std;
using namespace atcoder;

#define rep(i, n) for (int i = 0; i < (int)(n); i++)

int main() {
    int n,k;
    scanf("%d %d", &n,&k);
    int ans=0;
    for (int i=1; i<=n; i++) {
        for (int j=1; j<=n; j++) {
            int l=k-i-j;
            if (1<=l && l<=n) {
                ans++;
            }
        }
    }
    printf("%d", ans);
}