#include <bits/stdc++.h>
#include <algorithm>
using namespace std;
// #include <atcoder/dsu>
// using namespace atcoder;

#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define rep1(i, n) for (int i = 1; i <= (int)(n); i++)

int n,m;
int a[101][11];
int dp[101][1024];
int main() {
    cin >> n >> m;
    rep1(i,m) {
        rep1(j,n) {
            cin >> a[i][j];
        }
    }
    for (int i=0; i<=m; i++) {
        rep(j,1<<n) {
            dp[i][j] = 1'000'000'000;
        }
    }
    dp[0][0]=0;
    rep1(i,m) {
        rep(j,1<<n) {
            int already[11];
            rep1(k,n) {
                if ((j/(1<<(k-1)))%2==1) {
                    already[k]=1;
                } else {
                    already[k]=0;
                }
            }
            int v=0;
            rep1(k,n) {
                if (already[k]==1 || a[i][k]==1) {
                    v += (1<<(k-1));
                }
            }
            dp[i][j]=min(dp[i][j],dp[i-1][j]);
            dp[i][v]=min(dp[i][v],dp[i-1][j]+1);
        }
    }
    if (dp[m][(1<<n)-1] == 1'000'000'000) {
        cout << -1 << endl;
    } else  {
        cout << dp[m][(1<<n)-1] << endl;
    }
}
