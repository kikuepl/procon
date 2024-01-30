#include <bits/stdc++.h>
#include <algorithm>
using namespace std;
// #include <atcoder/dsu>
// using namespace atcoder;

#define rep(i, n) for (int i = 0; i < (int)(n); i++)

int n,k;
int a[100001];

int main() {
    cin>>n>>k;
    rep(i,n) {
        cin>>a[i];
    }
    long long ans=0;
    rep(i,n-1) {
        ans += upper_bound(a,a+n,a[i]+k)-a-(i+1);
    }
    cout << ans << endl;
}