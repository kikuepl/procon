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
	cin >> n;
	vector<P> p;
	rep(_,n) {
		int x,y;
		cin >> x >> y;
		p.push_back(make_pair(x,y));
	}
	double ans = 0;
	rep(i,n-1) {
		for (int j=i+1; j<n;j++) {
			int x1 = p[i].first, y1 = p[i].second;
			int x2 = p[j].first, y2 = p[j].second;
			double dis = (x1 - x2)*(x1-x2) + (y1-y2)*(y1-y2);
			dis = sqrt(dis);
			ans = max(ans, dis);
		}
	}
	cout << ans << endl;
    return 0;
}