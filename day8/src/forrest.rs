use std::fmt::{Display, Formatter};

struct Forrest(Vec<Vec<char>>);

enum Direction {
    WEST,
    NORTH,
    EAST,
    SOUTH,
}

impl Forrest {
    fn from_str(input: &str) -> Self {
        let mut forrest = Forrest(vec![]);
        for line in input.split('\n') {
            if line.trim().is_empty() {
                continue;
            }
            let trees = line.chars().collect::<Vec<char>>();
            forrest.0.push(trees);
        }
        forrest
    }

    fn height(&self) -> usize {
        self.0.len()
    }

    fn width(&self) -> usize {
        if self.0.len() == 0 {
            return 0;
        }
        self.0[0].len()
    }

    fn tree_visible(&self, row: usize, column: usize) -> bool {
        let max_width_index = self.width() - 1;
        let max_height_index = self.height() - 1;
        match (row, column) {
            (_, column) if column == max_height_index || column == 0 => true,
            (row, _) if row == max_width_index || row == 0 => true,
            (row, column) => {}
            _ => false,
        }
    }
}

impl Display for Forrest {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for tree_row in self.0.iter() {
            writeln!(f, "{}", tree_row.iter().collect::<String>())?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::forrest::Direction::{EAST, NORTH, SOUTH, WEST};
    use crate::forrest::Forrest;
    const INPUT: &str = r#"123456
654321
123456
"#;

    #[test]
    fn test_parsing() {
        let forrest = Forrest::from_str(INPUT);
        assert_eq!(format!("{forrest}"), INPUT)
    }

    #[test]
    fn visible_from() {
        let forrest = Forrest::from_str(INPUT);
        assert!(forrest.tree_visible_from(0, 0, NORTH));
        assert!(forrest.tree_visible_from(0, 0, WEST));
        assert!(!forrest.tree_visible_from(0, 0, EAST));
        assert!(!forrest.tree_visible_from(0, 0, SOUTH));
    }
}
