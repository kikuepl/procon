#include <atcoder/dsu>
#include <cstdio>

using namespace std;
using namespace atcoder;

#define rep(i, n) for (int i = 0; i < (int)(n); i++)

int main() {
    int d,n;
    scanf("%d %d", &d,&n);
    int day[100001];
    rep(i,n) {
        int l,r;
        scanf("%d %d", &l,&r);
        day[l]++;
        day[r+1]--;
    }
    rep(i,d) {
        day[i+1]+=day[i];
    }
    rep(i,d) {
        printf("%d ", day[i+1]);
    }
}