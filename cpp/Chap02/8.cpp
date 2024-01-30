#include <atcoder/dsu>
#include <cstdio>

using namespace std;
using namespace atcoder;

#define rep(i, n) for (int i = 0; i < (int)(n); i++)

int main() {
    int h,w;
    scanf("%d %d", &h,&w);
    int g[1502][1502],z[1502][1502];
    int a[100002],b[100002],c[100002],d[100002];
    rep(i,h) {
        rep(j,w) {
            scanf("%d", &g[i+1][j+1]);
        }
    }
    int q;
    scanf("%d", &q);
    rep(i,q) {
        scanf("%d %d %d %d",&a[i],&b[i],&c[i],&d[i]);
    }
    rep(i,h) {
        rep(j,w) {
            z[i+1][j+1]+=z[i+1][j]+g[i+1][j+1];
        }
    }
    rep(i,w) {
        rep(j,h) {
            z[j+1][i+1]+=z[j][i+1];
        }
    }
    rep(i,q) {
        printf("%d", z[c[i]][d[i]]-z[c[i]][b[i]-1]-z[a[i]-1][d[i]]+z[a[i]-1][b[i]-1]);
        printf("\n");
    }
}