struct Grid {
    rows: Vec<Row>
}

impl Grid {
    fn new(row_size: usize, col_size: usize) {
        let grid = Grid();
        for _ in 0..col_size {
            let row = Row::new(row_size);
            grid.rows.push(row);
        }
        grid
    }
}

struct Row {
    items: Vec<Chip>
}

impl Row {
    fn new(&self, size: usize) -> Row {
        let row = Row();
        row.items = Vec::new();
        for _ in 0..size {
            row.items.push(Chip::Empty);
        }
        row
    }
}

enum Chip {
    Empty,
    P1,
    P2
}