use crate::forrest::Direction::{Down, Left, Right, Up};
use std::str::FromStr;

#[derive(Default, Debug)]
pub struct Forrest {
    tree_matrix: Vec<u8>,
    width: usize,
    height: usize,
}

#[derive(Copy, Clone, Debug, Default)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl From<(usize, usize)> for Coordinate {
    fn from(value: (usize, usize)) -> Self {
        Coordinate {
            x: value.0,
            y: value.1,
        }
    }
}

struct CoordinateIterator<'a> {
    forrest: &'a Forrest,
    current_coordinate: Option<Coordinate>,
}

impl Iterator for CoordinateIterator<'_> {
    type Item = Coordinate;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ref mut current_coordinate) = self.current_coordinate {
            current_coordinate.x += 1;

            if current_coordinate.x >= self.forrest.width {
                current_coordinate.x = 0;
                current_coordinate.y += 1;
            }

            if current_coordinate.y >= self.forrest.height {
                self.current_coordinate = None
            }
        } else {
            self.current_coordinate = Some(Coordinate { x: 0, y: 0 })
        }

        self.current_coordinate
    }
}

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

struct ForrestIterator<'a> {
    forrest: &'a Forrest,
    current_coordinate: Coordinate,
    direction: Direction,
}

impl Iterator for ForrestIterator<'_> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let Coordinate { x, y } = self.current_coordinate;
        let Forrest {
            width: max_x,
            height: max_y,
            ..
        } = self.forrest;

        match self.direction {
            Up if y == 0 => {
                return None;
            }
            Up => {
                self.current_coordinate.y -= 1;
            }

            Right if x + 1 >= *max_x => {
                return None;
            }
            Right => {
                self.current_coordinate.x += 1;
            }

            Down if y + 1 >= *max_y => {
                return None;
            }
            Down => {
                self.current_coordinate.y += 1;
            }

            Left if x == 0 => {
                return None;
            }
            Left => {
                self.current_coordinate.x -= 1;
            }
        }

        self.forrest
            .get_tree_height(self.current_coordinate)
            .copied()
    }
}

impl FromStr for Forrest {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut forrest = Forrest::default();
        let rows = s.split('\n').filter(|row| !(*row).is_empty());
        forrest.height = rows.clone().count();
        forrest.width = rows.clone().next().unwrap_or_default().len();

        forrest.tree_matrix = rows.collect::<Vec<&str>>().as_slice().join("").into();

        Ok(forrest)
    }
}

impl Forrest {
    fn get_tree_height(&self, coord: Coordinate) -> Option<&u8> {
        let position = coord.y * self.width + coord.x;

        self.tree_matrix.get(position)
    }

    fn scenic_score(&self, coord: Coordinate) -> usize {
        let mut scores = vec![];
        let current_tree_height = self.get_tree_height(coord).unwrap();

        for direction in vec![Up, Right, Down, Left].into_iter() {
            if let Some(distance) = self
                .trees(coord, direction)
                .position(|height| height >= *current_tree_height)
            {
                scores.push(distance + 1);
            } else {
                scores.push(self.trees(coord, direction).count());
            }
        }

        scores.iter().product()
    }

    pub fn highest_scenic_score(&self) -> usize {
        self.coordinate_iterator()
            .map(|coord| self.scenic_score(coord))
            .max()
            .unwrap()
    }

    fn is_visible(&self, coord: Coordinate) -> bool {
        let current_tree_height = self.get_tree_height(coord).copied().unwrap();
        for direction in vec![Up, Right, Down, Left].into_iter() {
            if self.trees(coord, direction).max().unwrap_or(0) < current_tree_height {
                return true;
            }
        }

        false
    }

    fn coordinate_iterator(&self) -> CoordinateIterator {
        CoordinateIterator {
            forrest: self,
            current_coordinate: None,
        }
    }

    fn trees(&self, start_coordinate: Coordinate, direction: Direction) -> ForrestIterator {
        ForrestIterator {
            forrest: self,
            current_coordinate: start_coordinate,
            direction,
        }
    }

    pub fn num_of_visible_trees(&self) -> usize {
        // bool as usize -> false: 0, true: 1
        self.coordinate_iterator()
            .fold(0, |acc, coord| acc + self.is_visible(coord) as usize)
    }
}

#[cfg(test)]
mod tests {
    use crate::forrest::Direction::{Down, Left, Right, Up};
    use crate::forrest::Forrest;

    #[test]
    fn forrest_iterator() {
        const INPUT: &str = r#"
abcdef
ghXjkl
mnopqr
"#;
        let start_coordinate = (2, 1).into();
        let forrest: Forrest = INPUT.parse().unwrap();
        assert_eq!(
            *forrest.get_tree_height(start_coordinate).unwrap(),
            'X' as u8
        );
        assert_eq!(
            forrest
                .trees(start_coordinate, Up)
                .map(|c| c as char)
                .collect::<Vec<char>>(),
            vec!['c']
        );
        assert_eq!(
            forrest
                .trees(start_coordinate, Right)
                .map(|c| c as char)
                .collect::<Vec<char>>(),
            vec!['j', 'k', 'l']
        );
        assert_eq!(
            forrest
                .trees(start_coordinate, Down)
                .map(|c| c as char)
                .collect::<Vec<char>>(),
            vec!['o']
        );
        assert_eq!(
            forrest
                .trees(start_coordinate, Left)
                .map(|c| c as char)
                .collect::<Vec<char>>(),
            vec!['h', 'g']
        );
    }

    const INPUT: &str = r#"
30373
25512
65332
33549
35390       
"#;
    #[test]
    fn tree_visible() {
        let forrest: Forrest = INPUT.parse().unwrap();

        assert!(forrest.is_visible((1, 1).into()));
        assert!(!forrest.is_visible((3, 1).into()));
        assert!(!forrest.is_visible((1, 3).into()));

        assert_eq!(forrest.num_of_visible_trees(), 21);
    }

    #[test]
    fn scenic_score() {
        let forrest: Forrest = INPUT.parse().unwrap();

        assert_eq!(forrest.scenic_score((2, 1).into()), 4);
        assert_eq!(forrest.scenic_score((2, 3).into()), 8);
        assert_eq!(forrest.highest_scenic_score(), 8);
    }
}
