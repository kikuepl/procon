#include <bits/stdc++.h>
#include <algorithm>
using namespace std;
// #include <atcoder/dsu>
// using namespace atcoder;

#define rep(i, n) for (int i = 0; i < (int)(n); i++)

int n,k;
int a[1001],b[1001],c[1001],d[1001];
int p[1000001],q[1000001];

int main() {
    cin >> n >> k;
    rep(i,n) {
        cin >> a[i];
    }
    rep(i,n) {
        cin >> b[i];
    }
    rep(i,n) {
        cin >> c[i];
    }
    rep(i,n) {
        cin >> d[i];
    }
    rep(i,n) {
        rep(j,n) {
            p[i*n+j]=a[i]+b[j];
            q[i*n+j]=c[i]+d[j];
        }
    }
    sort(q, q+n*n);
    rep(i,n*n) {
        int t = k-p[i];
        int pos = lower_bound(q, q+n*n, t)-q;
        if (pos <= n*n && q[pos]==t) {
            cout << "Yes" << endl;
            return 0;
        }
    }
    cout << "No" << endl;
}