//DFS(深さ優先探索)の基本のやつ,パスが連結かどうかを調べる。
//https://atcoder.jp/contests/tessoku-book/submissions/49128345
fn dfs(position: usize, graph: &[Vec<usize>], visited: &mut Vec<bool>) {
    visited[position] = true;
    for &vv in &graph[position] {
        if visited[vv]==false {
            dfs(vv, graph, visited);
        }
    }
}

//ansに'goal'までの経路を入れる
//最短経路とは違う
//https://atcoder.jp/contests/tessoku-book/submissions/49193703
fn dfs(graph: &Vec<Vec<usize>>, visited: &mut Vec<bool>, position: usize, goal: usize, ans: &mut Vec<usize>) -> bool {
    if position == goal {
        ans.push(position+1);
        return true;
    }
    visited[position] = true;
    for v in &graph[position] {
        if visited[*v] == false {
            if dfs(graph, visited, *v, goal, ans) == true {
                ans.push(position+1);
                return true;
            }
        }
    }
    false
}

//BFS(幅優先探索), 最短経路などを求めるときに使う。
//https://atcoder.jp/contests/tessoku-book/submissions/49199189
fn bfs(que: &mut VecDeque<usize>, dist: &mut Vec<isize>, graph: Vec<Vec<usize>>) {
    while !que.is_empty() {
        let pos = que.pop_front().unwrap();
        for &v in &graph[pos] {
            if dist[v] == -1 {
                que.push_back(v);
                dist[v] = dist[pos]+1;
            }
        }
    }
}
//Dijkstra
//https://atcoder.jp/contests/tessoku-book/submissions/49200707
fn dijkstra(n: usize, g: &Vec<Vec<(usize, isize)>>) -> Vec<isize> {
    let mut cur = vec![INF; n];
    cur[0] = 0;
    let mut kakutei = vec![false; n];
    let mut que = BinaryHeap::new();
    que.push((0, 0));
    while !que.is_empty() {
        let (_, pos) = que.pop().unwrap();
        if kakutei[pos] == true {
            continue;
        }
        kakutei[pos] = true;
        for v in 0..g[pos].len() {
            let (nex, scr) = g[pos][v];
            if cur[pos]+scr < cur[nex] {
                cur[nex] = cur[pos]+scr;
                que.push((-cur[nex], nex));
            }
        }
    }
    cur
}
//Dijkstraの復元(0からendに行くまでの最短経路)
fn dijkstra_re(end: usize,n: usize, g: &Vec<Vec<(usize, isize)>>) -> Vec<usize> {
    let cur = dijkstra(n, &g);
    let mut v = end-1;
    let mut ans  = vec![];
    loop {
        let mut curscr = cur[v];
        ans.push(v+1);
        if v == 0 {
            break;
        }
        for (pre, scr) in &g[v] {
            if cur[*pre]+scr == curscr {
                v = *pre;
                curscr = cur[*pre];
                break;
            }
        }
    }
    ans.reverse();
    ans
}


//union-find
//https://atcoder.jp/contests/tessoku-book/submissions/49209137
//root -> 再帰による経路圧縮
struct UnionFind {
    par: Vec<usize>,
    siz: Vec<usize>
}

impl UnionFind {
    fn init(n: usize) -> UnionFind {
        let mut par = vec![INF; n];
        let mut siz = vec![1; n];
        UnionFind {
            par,
            siz,
        }
    }

    fn root(&mut self, mut x: usize) -> usize {
        if self.par[x] == INF {
            x
        } else {
            self.par[x] = self.root(self.par[x]);
            self.par[x]
        }
    }

    fn unite(&mut self, u: usize, v: usize) {
        let rootU = self.root(u);
        let rootV = self.root(v);
        if rootU != rootV {
            if self.siz[rootU] < self.siz[rootV] {
                self.par[rootU] = rootV;
                self.siz[rootV] += self.siz[rootU]
            } else {
                self.par[rootV] = rootU;
                self.siz[rootU] += self.siz[rootV];
            }
        }
    }

    fn same(&mut self, u: usize, v: usize) -> bool {
    if self.root(u) == self.root(v) {
            return true;
        } else {
            return false;
        };
    }
}

//.size(i)で連結成分を答える
struct UnionFind {
    par: Vec<usize>,
    siz: Vec<usize>
}

impl UnionFind {
    fn init(n: usize) -> UnionFind {
        let mut par = vec![INF; n];
        let mut siz = vec![1; n];
        UnionFind {
            par,
            siz,
        }
    }

    fn root(&mut self, mut x: usize) -> usize {
        if self.par[x] == INF {
            x
        } else {
            self.par[x] = self.root(self.par[x]);
            self.par[x]
        }
    }

    fn unite(&mut self, u: usize, v: usize) {
        let rootU = self.root(u);
        let rootV = self.root(v);
        if rootU != rootV {
            if self.siz[rootU] < self.siz[rootV] {
                self.par[rootU] = rootV;
                self.siz[rootV] += self.siz[rootU]
            } else {
                self.par[rootV] = rootU;
                self.siz[rootU] += self.siz[rootV];
            }
        }
    }

    fn size(&mut self, x: usize) -> usize {
        let root = self.root(x);
        self.siz[root]
    }

    fn same(&mut self, u: usize, v: usize) -> bool {
    if self.root(u) == self.root(v) {
            return true;
        } else {
            return false;
        };
    }
}