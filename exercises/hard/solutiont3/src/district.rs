use crate::graph_parser::{GraphParser, GraphValue};
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::{Display, Formatter};
use std::fs::OpenOptions;
use std::io::Read;

struct Graph {
    dic_map: HashMap<String, usize>,
    data: Vec<Vec<usize>>,
}

impl Display for Graph {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (i, row) in self.data.iter().enumerate() {
            f.write_str(&format!("({}, {:?}) ", i, row))?;
            f.write_str("\n")?;
        }
        Ok(())
    }
}

impl Graph {
    fn new() -> Self {
        let dic_map = HashMap::new();
        let data = Vec::new();
        Self { dic_map, data }
    }
    fn generate_edges(&mut self, edges: &[(String, Vec<String>)]) {
        let mut dic_map = HashMap::new(); // 节点名称到索引的映射
        let mut next_index = 0; // 下一个可用的索引
        let mut data = Vec::new(); // 邻接表
                                   // 遍历所有节点, 生成节点名称到索引的映射
        for (src, dsts) in edges {
            // 生成节点清单
            dic_map.entry(src.to_string()).or_insert_with(|| {
                let index = next_index;
                next_index += 1;
                data.push(Vec::new()); // 为每个新节点初始化邻接表
                index
            });
            // 遍历目标节点
            for dst in dsts {
                // 获取或分配目标节点的索引
                dic_map.entry(dst.to_string()).or_insert_with(|| {
                    let index = next_index;
                    next_index += 1;
                    data.push(Vec::new()); // 为每个新节点初始化邻接表
                    index
                });
            }
        }

        // 遍历所有边
        for (src, dsts) in edges {
            let src_index = *dic_map.get(src).unwrap();
            // 遍历目标节点
            for dst in dsts {
                // 获取或分配目标节点的索引
                let dst_index = *dic_map.get(dst).unwrap();

                // 添加边到邻接表，插入双向边
                data[src_index].push(dst_index);
                data[dst_index].push(src_index);
            }
        }
        self.dic_map = dic_map;
        self.data = data;
    }
}
fn count_connected_components(graph: &Graph) -> usize {
    let mut visited = HashSet::new();
    let mut count = 0;

    for node in 0..graph.data.len() {
        if !visited.contains(&node) {
            bfs(graph, node, &mut visited);
            count += 1;
        }
    }

    count
}

fn bfs(graph: &Graph, start: usize, visited: &mut HashSet<usize>) {
    let mut queue = VecDeque::new();
    queue.push_back(start);
    visited.insert(start);

    while let Some(node) = queue.pop_front() {
        for &neighbor in &graph.data[node] {
            if !visited.contains(&neighbor) {
                visited.insert(neighbor);
                queue.push_back(neighbor);
            }
        }
    }
}

pub fn count_provinces() -> String {
    let graphs = get_graphs();
    let mut counts_list = [0; 5];
    for (k, v) in graphs.into_iter() {
        let mut graph = Graph::new();
        graph.generate_edges(&v);
        let count = count_connected_components(&graph);
        counts_list[k.parse::<usize>().unwrap() - 1] = count;
        counts_list[k.parse::<usize>().unwrap() - 1] = count;
    }
    counts_list
        .iter()
        .map(|v| v.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

fn get_graphs() -> HashMap<String, Vec<(String, Vec<String>)>> {
    let mut input_file = OpenOptions::new().read(true).open("district.json").unwrap();
    let mut input = String::new();
    input_file.read_to_string(&mut input).unwrap();
    let parse_map = GraphParser::new(&input).parse().unwrap();
    if let GraphValue::Graph(map) = parse_map {
        map.iter()
            .map(|(k, v)| {
                let k = if let GraphValue::String(s) = k {
                    s.to_string()
                } else {
                    "".to_string()
                };
                let value = if let GraphValue::Graph(map) = v {
                    map.iter()
                        .map(|(k, v)| {
                            let k = if let GraphValue::String(s) = k {
                                s.to_string()
                            } else {
                                "".to_string()
                            };
                            let value = if let GraphValue::Array(array) = v {
                                array
                                    .iter()
                                    .map(|v| {
                                        if let GraphValue::String(s) = v {
                                            s.to_string()
                                        } else {
                                            "".to_string()
                                        }
                                    })
                                    .collect()
                            } else {
                                vec![]
                            };
                            (k.to_string(), value)
                        })
                        .collect::<Vec<(String, Vec<String>)>>()
                } else {
                    Vec::new()
                };
                (k.to_string(), value)
            })
            .collect()
    } else {
        HashMap::new()
    }
}
