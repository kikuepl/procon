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
    rep(i,n) {
        cin >> a[i];
    }
    vector<int> t;
    rep(i,n) {
        t.push_back(a[i]);
    }
    sort(t.begin(), t.end());
    t.erase(unique(t.begin(), t.end()), t.end());
    rep(i,n) {
        int pos = lower_bound(t.begin(), t.end(), a[i])-t.begin();
        b[i]=pos+1;
    }
    rep(i,n) {
        cout << b[i] << ' ';
    }
}