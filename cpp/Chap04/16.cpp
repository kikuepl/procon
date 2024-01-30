#include <bits/stdc++.h>
#include <algorithm>
using namespace std;
// #include <atcoder/dsu>
// using namespace atcoder;

#define rep(i, n) for (int i = 0; i < (int)(n); i++)

int n;
int a[100001],b[100001];

int main() {
    cin >> n;
    rep(i,n-1) {
        cin>>a[i];
    }
    rep(i,n-2) {
        cin>>b[i];
    }
    int dp[100001];
    rep(i,n+1) {
        dp[i]=INT_MAX;
    }
    dp[0]=0;
    dp[1]=a[0];
    for (int i=2; i<n; i++) {
        dp[i]=min(dp[i-1]+a[i-1],dp[i-2]+b[i-2]);
    }
    cout << dp[n-1] << endl;
}