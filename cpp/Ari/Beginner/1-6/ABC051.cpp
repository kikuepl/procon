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
	int k,s;
	cin >> k >> s;
	int ans = 0;
	rep(x,k+1) {
		rep(y,k+1) {
			int z = s-x-y;
			if (0<=z&&z<=k) {
				ans++;
			} 
		}
	}
	cout << ans << endl;
    return 0;
}