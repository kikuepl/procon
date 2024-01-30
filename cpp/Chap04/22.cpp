#include <bits/stdc++.h>
#include <algorithm>
using namespace std;
// #include <atcoder/dsu>
// using namespace atcoder;

#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define rep1(i, n) for (int i = 1; i <= (int)(n); i++)

int n;
int a[100001],b[100001];
long long dp[1000010];
int main() {
    cin >> n;
    rep1(i,n-1) {
        cin >> a[i];
    }
    rep1(i,n-1) {
        cin >> b[i];
    }
    rep1(i,n) {
        dp[i]=-1000000000;
    }
    dp[1]=0;
    for (int i=1; i<=n-1; i++) {
        dp[a[i]]=max(dp[a[i]], dp[i]+100);
        dp[b[i]]=max(dp[b[i]], dp[i]+150);
    }
    cout << dp[n] << endl;
}
