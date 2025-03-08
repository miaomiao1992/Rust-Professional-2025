use serde_json::Value;
use std::collections::{HashMap, HashSet};

struct UnionFind {
    parent: HashMap<String, String>,
}

impl UnionFind {
    fn new() -> Self {
        UnionFind {
            parent: HashMap::new(),
        }
    }

    fn find(&mut self, x: &str) -> String {
        if !self.parent.contains_key(x) {
            self.parent.insert(x.to_string(), x.to_string());
            return x.to_string();
        }

        let mut path = vec![];
        let mut current = x.to_string();
        while self.parent[&current] != current {
            path.push(current.clone());
            current = self.parent[&current].clone();
        }

        for node in path {
            self.parent.insert(node, current.clone());
        }

        current
    }

    fn union(&mut self, x: &str, y: &str) {
        let x_root = self.find(x);
        let y_root = self.find(y);
        if x_root != y_root {
            self.parent.insert(y_root, x_root);
        }
    }
}

pub fn count_provinces() -> String {
    let input_data = include_str!("../district.json");
    let json_value: Value = serde_json::from_str(input_data).unwrap();
    let batches = json_value.as_object().unwrap();

    let mut batch_numbers: Vec<u32> = batches.keys().filter_map(|k| k.parse().ok()).collect();
    batch_numbers.sort_unstable();

    let mut results = Vec::new();
    for num in batch_numbers {
        let batch_key = num.to_string();
        let batch = batches.get(&batch_key).unwrap().as_object().unwrap();

        // 关键修复：预处理时过滤自连接
        let mut merged: HashMap<&str, HashSet<&str>> = HashMap::new();
        for (city, neighbors) in batch {
            let city_str = city.as_str();
            let neighbors: HashSet<&str> = neighbors
                .as_array()
                .unwrap()
                .iter()
                .map(|v| v.as_str().unwrap())
                .filter(|&n| n != city_str) // 过滤自连接
                .collect();

            merged
                .entry(city_str)
                .and_modify(|set| set.extend(neighbors.iter().cloned()))
                .or_insert(neighbors);
        }

        // 构建双向邻接关系
        let mut graph: HashMap<&str, HashSet<&str>> = HashMap::new();
        for (&city, neighbors) in &merged {
            for &neighbor in neighbors {
                graph.entry(city).or_default().insert(neighbor);
                graph.entry(neighbor).or_default().insert(city); // 双向连接
            }
        }

        let mut uf = UnionFind::new();
        for (&city, neighbors) in &graph {
            for &neighbor in neighbors {
                uf.union(city, neighbor);
            }
        }

        // 统计所有节点的根
        let mut roots = HashSet::new();
        for node in graph.keys() {
            roots.insert(uf.find(node));
        }
        //println!("{:?}", roots);
        results.push(format!("{}", roots.len()));
    }

    results.join(",")
}
