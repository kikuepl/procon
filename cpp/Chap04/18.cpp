#include <bits/stdc++.h>
#include <algorithm>
using namespace std;
// #include <atcoder/dsu>
// using namespace atcoder;

#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define rep1(i, n) for (int i = 1; i <= (int)(n); i++)

int n,s;
int a[61];
int main() {
    cin >> n >> s;
    rep1(i,n) {
        cin >> a[i];
    }
    int dp[61][10001];
    dp[0][0]=true;
    rep1(i,s) {
        dp[0][i]=false;
    }
    rep1(i,n) {
        rep(j,s+1) {
            if (dp[i-1][j]==true) {
                dp[i][j]=true;
            }
            if (dp[i-1][j]==true && j+a[i]<=s) {
                dp[i][j+a[i]]=true;
            }
        }
    }
    if (dp[n][s]==true) {
        cout << "Yes" << endl;
    } else {
        cout << "No" << endl;
    }
}