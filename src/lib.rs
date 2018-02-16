extern crate rand;

use rand::Rng;
use rand::distributions::{IndependentSample, Range};

type Cells = Vec<Vec<bool>>;

#[derive(Debug)]
pub struct Game {
    cells: Cells,
    n_rows: usize,
    n_cols: usize,
}

impl Game {
    pub fn new(n_rows: usize, n_cols: usize) -> Game {
        let max = usize::max_value();
        assert!(n_rows < max && n_cols < max);

        let mut rands = RandCells::new(rand::thread_rng(), 10);
        let mut cells = Vec::with_capacity(n_rows);
        for _ in 0..n_rows {
            cells.push(rands.by_ref().take(n_cols).collect());
        }

        Game {
            cells,
            n_rows,
            n_cols,
        }
    }

    pub fn update(&mut self) {
        // XXX: We cannot use Iterator because we need to mutate the cells
        // during its loop. Rust's borrow checker does not allow it.
        for r in 0..self.n_rows {
            for c in 0..self.n_cols {
                let n_alive_nbs = neighbors(r, c, &self.cells);
                self.cells[r][c] = is_alive(self.cells[r][c], n_alive_nbs);
            }
        }
    }

    pub fn render(&self) {
        // https://stackoverflow.com/questions/34837011/how-to-clear-terminal-screen-in-rust-after-new-line-is-printing
        print!("{}[2J", 27 as char);
        for row in self.cells.iter() {
            for &cell in row.iter() {
                print!("{}", if cell { "█" } else { " " });
            }
            println!();
        }
    }
}

fn neighbors(r: usize, c: usize, cells: &Cells) -> usize {
    let nbs = vec![
        (r.wrapping_sub(1), c.wrapping_sub(1)),
        (r.wrapping_sub(1), c),
        (r.wrapping_sub(1), c.wrapping_add(1)),
        (r, c.wrapping_sub(1)),
        (r, c.wrapping_add(1)),
        (r.wrapping_add(1), c.wrapping_sub(1)),
        (r.wrapping_add(1), c),
        (r.wrapping_add(1), c.wrapping_add(1)),
    ];
    nbs.into_iter()
        .filter(|&(r, c)| *cells.get(r).and_then(|row| row.get(c)).unwrap_or(&false))
        // .filter(|&(r, c)| r < n_rows && c < n_cols && cells[r][c])
        .count()
}

fn is_alive(alive: bool, n_alive_nbs: usize) -> bool {
    if alive {
        n_alive_nbs == 2 || n_alive_nbs == 3
    } else {
        n_alive_nbs == 3
    }
}

struct RandCells<R: Rng> {
    rng: R,
    range: Range<u8>,
}

impl<R: Rng> RandCells<R> {
    fn new(rng: R, denom: u8) -> RandCells<R> {
        RandCells {
            rng,
            range: Range::new(0, denom),
        }
    }
}

impl<R: Rng> Iterator for RandCells<R> {
    type Item = bool;
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.range.ind_sample(&mut self.rng) == 0)
    }
}
