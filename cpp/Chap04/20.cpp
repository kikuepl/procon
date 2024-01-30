#include <bits/stdc++.h>
#include <algorithm>
using namespace std;
// #include <atcoder/dsu>
// using namespace atcoder;

#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define rep1(i, n) for (int i = 1; i <= (int)(n); i++)

string s,t;
int main() {
    cin >> s >> t;
    int dp[s.size()+1][t.size()+1];
    rep(i,s.size()+1) {
        rep(j,t.size()+1) {
            dp[i][j]=0;
        }
    }
    rep1(i,s.size()) {
        rep1(j,t.size()) {
            dp[i][j]=max(dp[i-1][j],dp[i][j-1]);
            if (s[i-1]==t[j-1]) {
                dp[i][j]=max(dp[i][j], dp[i-1][j-1]+1);
            }
        }
    }
    int ans =0;
    rep(i,t.size()+1) {
        ans = max(ans, dp[s.size()][i]);
    }
    cout << ans << endl;
}