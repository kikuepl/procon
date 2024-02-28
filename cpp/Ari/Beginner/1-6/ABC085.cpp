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
	int n,y;
	cin >> n >> y;
	rep(i,n+1) {
		rep(j,n-i+1) {
			int k = n-i-j;
			if (10000*i+5000*j+1000*k==y) {
				cout << i << " " << j << " " << k << endl;
				return 0;
			}
		}
	}
	cout << -1 << " " << -1 << " " << -1 << endl;
    return 0;
}