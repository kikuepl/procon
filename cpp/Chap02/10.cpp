#include <atcoder/dsu>

#include <cstdio>


using namespace std;
using namespace atcoder;

#define rep(i, n) for (int i = 0; i < (int)(n); i++)

int main() {
    int n;
    scanf("%d", &n);
    int a[100001];
    rep(i,n) {
        scanf("%d", &a[i+1]);
    }
    int mae[100001],usiro[100001];
    rep(i,n) {
        mae[i+1]=a[i+1];
        usiro[i+1]=a[n-i];
    }
    rep(i,n) {
        mae[i+1]=max(mae[i+1],mae[i]);
        usiro[i+1]=max(usiro[i+1],usiro[i]);
    }
    int d;
    scanf("%d", &d);
    rep(i,d) {
        int l,r;
        scanf("%d %d", &l,&r);
        printf("%d \n", max(mae[l-1],usiro[n-r]));
    }
}