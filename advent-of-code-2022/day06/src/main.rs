use itertools::Itertools;

fn main() {
    let stream = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";

    match marker_index(stream) {
        Some(idx) => print!("First marker appears after character {idx}."),
        None => println!("No marker appears in the given stream."),
    }
}

fn marker_index(stream: &str) -> Option<usize> {
    if stream.len() < 4 {
        return None;
    }

    let chars: Vec<char> = stream.chars().collect();
    for i in 0..chars.len() - 3 {
        let unique_chars: Vec<&char> = chars.iter().skip(i).take(4).unique().collect();

        if unique_chars.len() == 4 {
            return Some(i + 4);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use crate::marker_index;

    #[test]
    fn finding_marker() {
        assert_eq!(Some(7), marker_index("mjqjpqmgbljsphdztnvjfqwrcgsmlb"));
        assert_eq!(Some(5), marker_index("bvwbjplbgvbhsrlpgdmjqwftvncz"));
        assert_eq!(Some(6), marker_index("nppdvjthqldpwncqszvftbrmjlhg"));
        assert_eq!(Some(10), marker_index("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"));
        assert_eq!(Some(11), marker_index("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"));
    }

    #[test]
    fn not_finding() {
        assert_eq!(None, marker_index("aabbccaabbccaabbccaabbccaabbcc"));
        assert_eq!(None, marker_index("xyz"));
        assert_eq!(None, marker_index(""));
    }
}
