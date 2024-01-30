#include <atcoder/dsu>
#include <cstdio>

using namespace std;
using namespace atcoder;

#define rep(i, n) for (int i = 0; i < (int)(n); i++)

int main() {
    int h,w,n;
    scanf("%d %d %d", &h,&w,&n);
    int g[1502][1502];
    rep(i,n) {
        int a,b,c,d;
        scanf("%d %d %d %d", &a,&b,&c,&d);
        g[a][b]++;
        g[c+1][d+1]++;
        g[c+1][b]--;
        g[a][d+1]--;
    } 
    rep(i,h) {
        rep(j,w) {
            g[i+1][j+1]+=g[i+1][j];
        }
    }
    rep(j,w) {
        rep(i,h) {
            g[i+1][j+1]+=g[i][j+1];
        }
    }
    rep(i,h) {
        rep(j,w) {
            printf("%d ",g[i+1][j+1]);
        }
        printf("\n");
    }
}