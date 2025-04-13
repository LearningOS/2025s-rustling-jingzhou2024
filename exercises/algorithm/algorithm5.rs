/*
	bfs
	This problem requires you to implement a basic BFS algorithm
*/


use std::collections::VecDeque;

// 定义图结构
struct Graph {
    adj: Vec<Vec<usize>>, // 邻接表，表示图的边
}

impl Graph {
    // 创建一个有n个顶点的新图
    fn new(n: usize) -> Self {
        Graph {
            adj: vec![vec![]; n], // 初始化n个空的邻接表
        }
    }

    // 添加边到图中（无向图）
    fn add_edge(&mut self, src: usize, dest: usize) {
        self.adj[src].push(dest); // 从src到dest的边
        self.adj[dest].push(src); // 从dest到src的边（无向图）
    }

    // 执行广度优先搜索，返回访问节点的顺序
    fn bfs_with_return(&self, start: usize) -> Vec<usize> {
        let mut visit_order = vec![]; // 存储访问节点的顺序
        let mut visited = vec![false; self.adj.len()]; // 标记节点是否被访问
        let mut queue = VecDeque::new(); // BFS使用的队列

        // 初始化：将起始节点标记为已访问并加入队列
        visited[start] = true;
        queue.push_back(start);
        visit_order.push(start);

        // 当队列不为空时，继续处理
        while let Some(current) = queue.pop_front() {
            // 遍历当前节点的所有邻居
            for &neighbor in &self.adj[current] {
                // 如果邻居未被访问
                if !visited[neighbor] {
                    visited[neighbor] = true; // 标记为已访问
                    queue.push_back(neighbor); // 将邻居加入队列
                    visit_order.push(neighbor); // 记录访问顺序
                }
            }
        }

        visit_order // 返回访问顺序
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bfs_all_nodes_visited() {
        let mut graph = Graph::new(5);
        graph.add_edge(0, 1);
        graph.add_edge(0, 4);
        graph.add_edge(1, 2);
        graph.add_edge(1, 3);
        graph.add_edge(1, 4);
        graph.add_edge(2, 3);
        graph.add_edge(3, 4);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0, 1, 4, 2, 3]);
    }

    #[test]
    fn test_bfs_different_start() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);

        let visited_order = graph.bfs_with_return(2);
        assert_eq!(visited_order, vec![2, 1, 0]);
    }

    #[test]
    fn test_bfs_with_cycle() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 0);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0, 1, 2]);
    }

    #[test]
    fn test_bfs_single_node() {
        let mut graph = Graph::new(1);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0]);
    }
}