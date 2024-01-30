#include <bits/stdc++.h>
#include <algorithm>
using namespace std;
// #include <atcoder/dsu>
// using namespace atcoder;

#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define rep1(i, n) for (int i = 1; i <= (int)(n); i++)

int n;
int p[2001],a[2001];
int dp[2001][2001];
int main() {
    cin >> n;
    rep1(i,n) {
        cin >> p[i] >> a[i];
    }
    rep(l,n) {
        for (int r=n; 0<r;r--) {
            int scr1=0;
            int scr2=0;
            if (1<l && l <= p[l-1] && p[l-1] <= r) {
                scr1 = a[l-1];
            }
            if (r<n && l <= p[r+1] && p[r+1] <= r) {
                scr2 = a[r+1];
            }
            if (l==1) {
                dp[l][r] = dp[l][r+1]+scr2;
            } else if (r==n) {
                dp[l][r] = dp[l-1][r]+scr1;
            } else {
                dp[l][r]=max(dp[l-1][r]+scr1, dp[l][r+1]+scr2);
            }
        }
    }
    int ans = 0;
    rep1(i,n) {
        ans = max(ans, dp[i][i]);
    }
    cout << ans << endl;
}
