#include <bits/stdc++.h>
using namespace std;
// #include <atcoder/dsu>
// using namespace atcoder;

#define rep(i, n) for (int i = 0; i < (int)(n); i++)

int main() {
    int n,x;
    cin>>n>>x;
    int a[n];
    rep(i,n) {
        cin >> a[i];
    }
    int l=0;
    int r=n;
    while (l<r) {
        int mid = (l+r)/2;
        if (a[mid]<x) {
            l=mid+1;
        } else {
            r=mid;
        }
    }
    cout<<l+1<<endl;
}