#include <atcoder/dsu>
#include <cstdio>

using namespace std;
using namespace atcoder;

#define rep(i, n) for (int i = 0; i < (int)(n); i++)

int main() {
    int n,q;
    int a[100001], s[100002];
    scanf("%d %d", &n,&q);
    rep(i,n) {
        scanf("%d", &a[i]);
    }
    rep(i,n+1) {
        s[i+1]=s[i]+a[i];
    }
    rep(i,q) {
        int l,r;
        scanf("%d %d", &l, &r);
        int ans = s[r]-s[l-1];
        printf("%d", ans);
        printf("\n");
    }
}