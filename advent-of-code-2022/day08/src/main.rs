use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let (forest, mut is_visible) = parse_trees(&contents);

    let n_visible = num_of_visible(forest, &mut is_visible);
    println!("Number of visible trees is {}.", n_visible);
}

fn tree_house(contents: &str) -> usize {
    let (forest, mut is_visible) = parse_trees(&contents);
    let n_visible = num_of_visible(forest, &mut is_visible);

    n_visible
}

fn parse_trees(contents: &str) -> (Vec<Vec<u32>>, Vec<Vec<bool>>) {
    let forest: Vec<Vec<u32>> = contents.split("\r\n")
        .map(|line| line.chars()
            .map(|c| c.to_digit(10).unwrap_or(0))
            .collect::<Vec<u32>>()
        )
        .collect();

    let is_visible: Vec<Vec<bool>> = forest.iter()
        .map(|v| v.iter().map(|_| false)
            .collect())
        .collect();

    (forest, is_visible)
}

fn num_of_visible(forest: Vec<Vec<u32>>, is_visible: &mut Vec<Vec<bool>>) -> usize {
    let n_rows = forest.len();
    let n_cols = forest.get(0).unwrap().len();
     
    // Check each row from both sides.
    for i in 0..n_rows {
        
        let mut max_height = usize::try_from(*forest.get(i).unwrap().get(0).unwrap()).unwrap();
        is_visible[i][0] = true;
        for j in 1..n_cols {
            let height = usize::try_from(*forest.get(i).unwrap().get(j).unwrap()).unwrap();
            if max_height < height {
                max_height = height;
                is_visible[i][j] = true;
            }
        }

        let mut max_height = usize::try_from(*forest.get(i).unwrap().get(n_cols-1).unwrap()).unwrap();
        is_visible[i][n_cols-1] = true;
        for j in (1..n_cols-1).rev() {
            let height = usize::try_from(*forest.get(i).unwrap().get(j).unwrap()).unwrap();
            if max_height < height {
                max_height = height;
                is_visible[i][j] = true;
            }
        }
    }

    // Check each column from both sides.
    for i in 0..n_cols {
        
        let mut max_height = usize::try_from(*forest.get(0).unwrap().get(i).unwrap()).unwrap();
        is_visible[0][i] = true;
        for j in 1..n_rows {
            let height = usize::try_from(*forest.get(j).unwrap().get(i).unwrap()).unwrap();
            if max_height < height {
                max_height = height;
                is_visible[j][i] = true;
            }
        }

        let mut max_height = usize::try_from(*forest.get(n_rows-1).unwrap().get(i).unwrap()).unwrap();
        is_visible[n_rows-1][i] = true;
        for j in (1..n_rows-1).rev() {
            let height = usize::try_from(*forest.get(j).unwrap().get(i).unwrap()).unwrap();
            if max_height < height {
                max_height = height;
                is_visible[j][i] = true;
            }
        }
    }

    let n_visible = is_visible.iter()
        .map(|v| v.into_iter().filter(|b| **b).count())
        .sum::<usize>();

    n_visible
}

#[cfg(test)]
mod tests {
    use crate::tree_house;

    #[test]
    fn parse_input() {
        let tree_str = "30373\r
25512\r
65332\r
33549\r
35390";

        assert_eq!(21, tree_house(tree_str));
    }
}
