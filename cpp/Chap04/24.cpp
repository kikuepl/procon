#include <bits/stdc++.h>
#include <algorithm>
using namespace std;
// #include <atcoder/dsu>
// using namespace atcoder;

#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define rep1(i, n) for (int i = 1; i <= (int)(n); i++)


int n;
int a[100001];
int main() {
    cin >> n;
    rep1(i,n) {
        cin >> a[i];
    }
    int dp[100001];
    rep1(i,n) {
        dp[i] = INT_MAX;
    }
    vector <int> t;
    int length = 0;
    t.push_back(0);
    rep1(i,n) {
        int pos = lower_bound(t.begin(), t.end(), a[i])-t.begin();
        if (length < pos) {
            t.push_back(a[i]);
            length ++;
        } else {
            t[pos] = a[i];
            dp[pos] = a[i];
        }
        dp[pos]=min(dp[pos], a[i]);
    }
    cout << t.size()-1 << endl;
}
