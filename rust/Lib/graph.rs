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