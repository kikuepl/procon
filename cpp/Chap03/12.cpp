#include <bits/stdc++.h>
using namespace std;
// #include <atcoder/dsu>
// using namespace atcoder;

#define rep(i, n) for (int i = 0; i < (int)(n); i++)

int n,k;
int a[100001];

bool check(int x) {
    long long ans = 0;
    rep(i,n) {
        ans += x/a[i];
    }
    if (k<=ans) {
        return true;
    } else {
        return false;
    }
}

int main() {
    cin>>n>>k;
    rep(i,n) {
        cin >> a[i];
    }
    int l=0,r=1'000'000'000;
    while (l<r) {
        int mid =(l+r)/2;
        bool chk = check(mid);
        if (chk ==false) {
            l=mid+1;
        } else {
            r=mid;
        }
    }
    cout << l << endl;
}