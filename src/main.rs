use std::{collections::VecDeque, fs};

#[derive(Debug, Clone)]
struct Graph {
    pub shortcuts: Vec<usize>,
}

impl Graph {
    fn new(in_shortcuts: &[usize]) -> Self {
        Self {
            shortcuts: {
                in_shortcuts
                    .iter()
                    .enumerate()
                    .map(|(index, shortcut_index)| {
                        if index != shortcut_index - 1 {
                            shortcut_index - 1
                        } else {
                            0
                        }
                    })
                    .collect::<Vec<usize>>()
            },
        }
    }

    fn find_shortest_paths(&mut self) -> String {
        let mut distances = vec![usize::MAX; self.shortcuts.len()];
        distances[0] = 0;
        let mut queue = VecDeque::from([1usize]);

        while !queue.is_empty() {
            let popped = queue.pop_front().unwrap();
            if popped >= distances.len() {
                break;
            }

            // If distance is MAX, we know that we have to at least assign the last node + 1
            if distances[popped] == usize::MAX {
                distances[popped] = distances[popped - 1] + 1;
                queue.push_front(popped + 1);
            }
        }

        let str_vec = distances
            .iter()
            .map(|d| d.to_string())
            .collect::<Vec<String>>();
        str_vec.join(" ")
    }
}

fn convert_input(input: &str) -> (usize, Vec<usize>) {
    let mut iter = input.lines();
    let num_intersections = iter.next().unwrap().parse::<usize>().unwrap();
    let shortcuts = iter
        .next()
        .unwrap()
        .split(' ')
        .filter_map(|c| c.parse::<usize>().ok())
        .collect::<Vec<usize>>();
    (num_intersections, shortcuts)
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let (num_intersections, shortcuts) = convert_input(&input);
    println!("{} - {:?}", num_intersections, shortcuts);

    let mut graph = Graph::new(&shortcuts);
    println!("{:?}", graph);
    println!("{:?}", graph.find_shortest_paths());
}

#[cfg(test)]
mod tests {
    use crate::convert_input;

    #[test]
    fn test_input() {
        let (num_intersections, shortcuts) = convert_input("3\n2 2 3");
        assert_eq!(num_intersections, 3);
        assert_eq!(&[2, 2, 3], &shortcuts[..]);

        let (num_intersections, shortcuts) = convert_input("5\n1 2 3 4 5");
        assert_eq!(num_intersections, 5);
        assert_eq!(&[1, 2, 3, 4, 5], &shortcuts[..]);

        let (num_intersections, shortcuts) = convert_input("7\n4 4 4 4 7 7 7");
        assert_eq!(num_intersections, 7);
        assert_eq!(&[4, 4, 4, 4, 7, 7, 7], &shortcuts[..]);
    }
}
