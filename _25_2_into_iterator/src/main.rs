struct Grid {
    x_coords: Vec<u32>,
    y_coords: Vec<u32>,
}

struct GridIter<'a> {
    grid: &'a Grid,
    i: usize,
    j: usize,
}

impl<'a> IntoIterator for &'a Grid {
    type Item = (u32, u32);
    type IntoIter = GridIter<'a>;

    fn into_iter(self) -> GridIter<'a> {
        GridIter {
            grid: self,
            i: 0,
            j: 0,
        }
    }
}

impl Iterator for GridIter<'_> {
    type Item = (u32, u32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= self.grid.x_coords.len() {
            self.i = 0;
            self.j += 1;
            if self.j >= self.grid.y_coords.len() {
                return None;
            }
        }

        let res = Some((self.grid.x_coords[self.i], self.grid.y_coords[self.j]));
        self.i += 1;
        res
    }
}

fn main() {
    let grid = Grid {
        x_coords: vec![3, 5, 7, 9],
        y_coords: vec![10, 20, 30, 40],
    };

    for (x, y) in &grid {
        println!("point: ({x}, {y})");
    }
    for (x, y) in &grid {
        println!("point: ({x}, {y})");
    }
}
