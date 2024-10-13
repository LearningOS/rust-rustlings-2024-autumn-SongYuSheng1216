/*
	bfs
	This problem requires you to implement a basic BFS algorithm
*/

use std::collections::VecDeque;

// Define a graph
struct Graph {
    adj: Vec<Vec<usize>>, 
}

impl Graph {
    // Create a new graph with n vertices
    fn new(n: usize) -> Self {
        Graph {
            adj: vec![vec![]; n],
        }
    }

    // Add an edge to the graph
    fn add_edge(&mut self, src: usize, dest: usize) {
        self.adj[src].push(dest); 
        self.adj[dest].push(src); 
    }

    // Perform a breadth-first search on the graph, return the order of visited nodes
    fn bfs_with_return(&self, start: usize) -> Vec<usize> {
		//TODO
        /*
        先将start压入visit_order
        根据start，访问self.adj[start]的值
        先访问最先压入self.adj[start]的值，然后对比访问的值和visit_order数组里的值
        若为新值，则将访问到的值压入visit_order数组中，直到self.adj[start]为空
        然后访问visit_order[1]的值，然后重复以上步骤
        */
        // 了解一下HashSet是什么？
        // 了解一下VecDeque是什么？
        // contains是什么方法
        let mut visit_order = vec![];
        visit_order.push(start);
        for search_index in 0..self.adj.len(){          // 不会动态变化,len只会在进入for之前算一次
            for i in self.adj[search_index].iter() {    // for循环默认时into_iter()
                // for j in visit_order.iter() {           // 以不可变引用的形式遍历visit_order
                //     if *i == *j {
                //         flag_new_value = false;
                //         break;
                //     }
                // }                                       // 不可变引用的作用域结束
                if visit_order.contains(i) == false {
                    visit_order.push(*i);               // 以可变引用的形式遍历visit_order
                }
            }
        }
        visit_order
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

