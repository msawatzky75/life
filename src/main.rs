use crossterm::{cursor, style::Print, terminal, QueueableCommand};
use indexmap::map::IndexMap;
use std::io::{stdout, Write};

struct SquareGrid<T> {
	cells: IndexMap<(i32, i32), T>,
}

trait Serialize {
	fn to_string(&self, size: (i32, i32), piece: char, empty: char) -> String;
}

impl Serialize for SquareGrid<bool> {
	fn to_string(&self, (width, height): (i32, i32), piece: char, empty: char) -> String {
		let mut output: String = "".to_owned();

		for y in 0..height {
			for x in 0..width {
				let alive = self.cells.get(&(x, y)).unwrap_or(&false);
				output.push(if *alive { piece } else { empty });
			}
			output.push('\n');
		}

		return output;
	}
}

trait Life {
	fn tick(&mut self);
	fn get_cells_to_consider(&self) -> Vec<(i32, i32)>;
	fn neighbors(&self, pos: (i32, i32)) -> Vec<(i32, i32)>;
	fn live_neighbors(&self, pos: (i32, i32)) -> i32;
}

impl Life for SquareGrid<bool> {
	fn tick(&mut self) {
		// the future state of the cells
		let mut next = self.cells.clone();

		// consider the state of each cell
		for coord in self.get_cells_to_consider() {
			let live_neighbors = Life::live_neighbors(self, coord);

			if live_neighbors < 2 || live_neighbors > 3 {
				// cell dies
				next.remove(&coord);
			} else if live_neighbors == 3 {
				next.insert(coord, true);
			}
		}

		// update the cells
		self.cells = next;
	}

	fn get_cells_to_consider(&self) -> Vec<(i32, i32)> {
		let mut cells_to_consider = Vec::new();

		for (coord, _alive) in &self.cells {
			let neighbors = Life::neighbors(self, *coord);

			if !cells_to_consider.contains(coord) {
				cells_to_consider.push(*coord);
			}

			for neighbor in neighbors {
				if cells_to_consider.contains(&neighbor) {
					continue;
				}
				cells_to_consider.push(neighbor);
			}
		}

		return cells_to_consider;
	}

	fn neighbors(&self, pos: (i32, i32)) -> Vec<(i32, i32)> {
		let mut neighbors = Vec::new();

		for dx in -1..2 {
			for dy in -1..2 {
				if dx == 0 && dy == 0 {
					continue;
				}

				neighbors.push((pos.0 + dx, pos.1 + dy));
			}
		}

		return neighbors;
	}

	fn live_neighbors(&self, pos: (i32, i32)) -> i32 {
		let mut count = 0;

		for (x, y) in [
			(pos.0 - 1, pos.1 - 1),
			(pos.0 - 1, pos.1),
			(pos.0 - 1, pos.1 + 1),
			(pos.0, pos.1 - 1),
			(pos.0, pos.1 + 1),
			(pos.0 + 1, pos.1 - 1),
			(pos.0 + 1, pos.1),
			(pos.0 + 1, pos.1 + 1),
		] {
			if self.cells.contains_key(&(x, y)) {
				count += 1;
			}
		}

		return count;
	}
}

fn main() -> Result<(), std::io::Error> {
	let mut stdout = stdout();
	let size = (
		terminal::size()?.0 as i32 - 1,
		terminal::size()?.1 as i32 - 2,
	);

	let mut grid = SquareGrid {
		cells: IndexMap::new(),
	};

	// insert a glider
	// grid.cells.insert((0, 0), true);
	// grid.cells.insert((2, 0), true);
	// grid.cells.insert((1, 1), true);
	// grid.cells.insert((2, 1), true);
	// grid.cells.insert((1, 2), true);

	insert_glider(&mut grid, (2, 5));

	let mut gen = 0;
	loop {
		stdout
			.queue(terminal::Clear(terminal::ClearType::All))?
			.queue(cursor::MoveTo(0, 0))?
			.queue(Print(grid.to_string(size, '0', ' ')))?
			.queue(Print(format!("generaton: {gen}")))?
			.flush()?;

		std::thread::sleep(std::time::Duration::from_millis(100));
		grid.tick();
		gen += 1;
	}

	Ok(())
}

fn insert_glider(grid: &mut SquareGrid<bool>, pos: (i32, i32)) {
	grid.cells.insert((pos.0 + 1, pos.1), true);
	grid.cells.insert((pos.0 + 2, pos.1), true);
	grid.cells.insert((pos.0 + 2, pos.1 + 1), true);
	grid.cells.insert((pos.0 + 1, pos.1 + 2), true);
	grid.cells.insert((pos.0, pos.1 + 2), true);
}
