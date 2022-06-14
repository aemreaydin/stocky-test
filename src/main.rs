// fn solution(input: &str) -> Result<String> {
//     let mut iter = input.lines();
//     let num_intersections = iter.next()?.parse::<usize>()?;
//     let shortcuts = iter
//         .next()?
//         .split(" ")
//         .filter_map(|c| c.parse::<usize>().ok())

// }

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

fn main() {}

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
