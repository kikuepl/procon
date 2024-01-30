#include <bits/stdc++.h>
#include <algorithm>
using namespace std;
// #include <atcoder/dsu>
// using namespace atcoder;

#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define rep1(i, n) for (int i = 1; i <= (int)(n); i++)

int n,w;
long long dp[101][100001];
int main() {
    cin >> n >> w;
    rep(i,n+1) {
        rep(j,w+1) {
            dp[i][j]=-1;
        }
    }
    dp[0][0]=0;
    rep1(i,n) {
        int u,v;
        cin >> u >> v;
        rep(j,w+1) {
            if (dp[i-1][j]==-1) {
                continue;
            }
            dp[i][j]=max(dp[i][j],dp[i-1][j]);
            if (j+u<=w) {
                dp[i][j+u]=max(dp[i][j], dp[i-1][j]+v);
            }
        }
    }
    long long ans = 0;
    rep(j,w+1) {
        ans = max(ans,dp[n][j]);
    }
    cout << ans << endl;
}