use std::collections::HashSet;

fn scc_bfs<T>(g: &Graph<T>, order: &mut Vec<usize>, visited: &mut Vec<bool>, n: usize) {
    visited[n] = true;
    for e in g[n].iter().rev() {
        if !visited[e.dst] {
            scc_bfs(g, order, visited, e.dst);
        }
    }
    order.push(n);
}

//強連結成分分解
//上流側の強連結成分の番号が小さくなるよう番号付する
//強連結成分の数と各頂点の強連結成分の番号を返す
fn strongly_connected_components<T>(g: &Graph<T>) -> (usize, Vec<usize>)
where
    T: Copy,
{
    //行く先がなくなった頂点から順に追加される
    let mut order = Vec::<usize>::new();
    let mut visited = vec![false; g.len()];
    let mut bfs = VecDeque::<usize>::new();
    //未訪の頂点から順にBFS
    for n in 0..g.len() {
        if !visited[n] {
            scc_bfs(&g, &mut order, &mut visited, n);
        }
    }
    //辺の向きを逆にしたグラフ
    let mut g_rev = Graph::<T>::new(g.len());
    for n in 0..g.len() {
        for e in g[n].iter() {
            g_rev.add_directed_edge(e.dst, e.src, e.cost);
        }
    }
    //gの頂点がどの強連結成分に属するか
    let mut scv = vec![0; g.len()];
    let mut scv_num = 0;
    order.reverse();
    visited.fill(false);
    for n in order {
        if visited[n] {
            continue;
        }
        bfs.push_front(n);
        scv[n] = scv_num;
        while let Some(i) = bfs.pop_front() {
            visited[i] = true;
            for e in g_rev[i].iter() {
                if !visited[e.dst] {
                    bfs.push_front(e.dst);
                    scv[e.dst] = scv_num;
                }
            }
        }
        scv_num += 1;
    }
    return (scv_num, scv);
}