#include <bits/stdc++.h>
#include <algorithm>
#include <atcoder/all>

using namespace atcoder;
using namespace std;
typedef long long ll;
typedef pair<int, int> P;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define rep1(i, n) for (int i = 1; i <= (int)(n); i++)

const vector<vector<int>> d = {{0,1}, {0,-1}, {1,0}, {-1, 0}};
const int MOD = 1e9 + 7;

int main() {
	int n;
	cin>>n;
	vector<int> l(n);
	rep(i,n) {
		cin >> l[i];
	}
	int ans  =0;
	sort(l.begin(), l.end());
	rep(i,n-2) {
		for (int j=i+1; j<n-1;j++) {
			for (int k=j+1; k<n;k++) {
				if (l[k]<l[i]+l[j] && l[i]!=l[j] && l[j]!=l[k] && l[k]!=l[i]) {
					ans++;
				}
			}
		}
	}
	cout << ans << endl;
    return 0;
}