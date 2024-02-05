use crate::grid::point::*;


pub type Neighbours = Vec<Point<i32>>;

pub enum NType {
    Moore,
    VonNeumann,
    ChebyshevDistance,
    ManhattanDistance,
}

pub enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl Direction {
    pub fn rel_cell(&self) -> Point<i32> {
        match self {
            Self::North => Point::new(-1, 0),
            Self::NorthEast => Point::new(-1, 1),
            Self::East => Point::new(0, 1),
            Self::SouthEast => Point::new(1, 1),
            Self::South => Point::new(1, 0),
            Self::SouthWest => Point::new(1, -1),
            Self::West => Point::new(0, -1),
            Self::NorthWest => Point::new(-1, -1),
        }
    }
}

pub struct MissingNeighbourhoodSize;

pub fn neighbourhood_coords(
    n_type: NType,
    n_size: Option<i32>,
    loc: Point<i32>,
) -> Result<Neighbours, MissingNeighbourhoodSize> {
    let mut neighbours = Vec::new();
    relative_neighbourhood_coords(n_type, n_size)?;
    convert_relative_point_to_absolute(&mut neighbours, &loc);
    remove_invalid(&mut neighbours, &Point::new(n_size.unwrap(), n_size.unwrap()));
    remove_self(&mut neighbours, &loc);
    Ok(neighbours)
}

pub fn relative_neighbourhood_coords(
    n_type: NType,
    n_size: Option<i32>,
) -> Result<Neighbours, MissingNeighbourhoodSize> {
    let mut neighbours: Neighbours = Vec::new();
    match n_type {
        NType::Moore => add_moore_neighbourhood_cells(&mut neighbours),
        NType::VonNeumann => add_von_neumann_neighbourhood_cells(&mut neighbours),
        NType::ChebyshevDistance => {
            if n_size.is_none() {
                return Err(MissingNeighbourhoodSize);
            }
            add_chebyshev_distance_cells(&mut neighbours, n_size.unwrap());
        }
        NType::ManhattanDistance => {
            if n_size.is_none() {
                return Err(MissingNeighbourhoodSize);
            }
            add_chebyshev_distance_cells(&mut neighbours, n_size.unwrap());
        }
    }
    Ok(neighbours)
}

fn add_moore_neighbourhood_cells(neighbours: &mut Neighbours) {
    add_chebyshev_distance_cells(neighbours, 1)
}

fn add_von_neumann_neighbourhood_cells(neighbours: &mut Neighbours) {
    add_manahattan_distance_cells(neighbours, 1)
}

fn add_chebyshev_distance_cells(neighbours: &mut Neighbours, neighbourhood_size: i32) {
    let mut count = 1;
    while count <= neighbourhood_size {
        add_corners_offset(neighbours, count);
        add_all_cells_across(neighbours, count);
        count += 1;
    }
}

fn add_manahattan_distance_cells(neighbours: &mut Neighbours, neighbourhood_size: i32) {
    add_adjacent_cells(neighbours, &Point::new(0, 0), 1);
    let mut count = 0;
    if neighbourhood_size == 1 {
        return;
    }
    for _ in 1..neighbourhood_size {
        for _ in count..neighbours.len() {
            let curr = Point::new(neighbours[count].x(), neighbours[count].y());
            add_adjacent_cells(neighbours, &curr, 1);
            count += 1;
        }
    }
}

fn add_all_cells_across(neighbours: &mut Neighbours, count: i32) {
    // TODO: cleanup with a macro
    add_cells_ptp(
        neighbours,
        Direction::East,
        &(Direction::NorthWest.rel_cell() * count),
        &(Direction::NorthEast.rel_cell() * count),
    )
    .ok();
    add_cells_ptp(
        neighbours,
        Direction::East,
        &(Direction::SouthWest.rel_cell() * count),
        &(Direction::SouthEast.rel_cell() * count),
    )
    .ok();
    add_cells_ptp(
        neighbours,
        Direction::South,
        &(Direction::NorthWest.rel_cell() * count),
        &(Direction::SouthWest.rel_cell() * count),
    )
    .ok();
    add_cells_ptp(
        neighbours,
        Direction::South,
        &(Direction::NorthEast.rel_cell() * count),
        &(Direction::SouthEast.rel_cell() * count),
    )
    .ok();
}

pub struct InvalidDirection;
fn add_cells_ptp(
    neighbours: &mut Neighbours,
    direction: Direction,
    start: &Point<i32>,
    finish: &Point<i32>,
) -> Result<(), InvalidDirection> {
    match direction {
        Direction::East | Direction::West => Ok(add_cells_horizontally(neighbours, start, finish)),
        Direction::North | Direction::South => Ok(add_cells_vertically(neighbours, start, finish)),
        _ => Err(InvalidDirection),
    }
}

fn add_cells_horizontally(neighbours: &mut Neighbours, start: &Point<i32>, finish: &Point<i32>) {
    for cell in start.y() + 1..finish.y() {
        neighbours.push(Point::new(start.x(), cell));
    }
}

fn add_cells_vertically(neighbours: &mut Neighbours, start: &Point<i32>, finish: &Point<i32>) {
    for cell in start.x() + 1..finish.x() {
        neighbours.push(Point::new(cell, start.y()));
    }
}

fn add_corners_offset(neighbours: &mut Neighbours, count: i32) {
    neighbours.push(Direction::NorthWest.rel_cell() * count);
    neighbours.push(Direction::NorthEast.rel_cell() * count);
    neighbours.push(Direction::SouthWest.rel_cell() * count);
    neighbours.push(Direction::SouthEast.rel_cell() * count);
}

fn add_adjacent_cells(n: &mut Neighbours, rel_cell: &Point<i32>, distance: i32) {
    for i in 1..distance + 1 {
        n.push(Point::new(i, 0) + *rel_cell);
        n.push(Point::new(0, i) + *rel_cell);
        n.push(Point::new(-i, 0) + *rel_cell);
        n.push(Point::new(0, -i) + *rel_cell);
    }
}
pub fn convert_relative_point_to_absolute(neighbours: &mut Neighbours, cell: &Point<i32>) {
    // neighbours.iter_mut().map(|n| *n + *cell).for_each(drop);
    for i in 0..neighbours.len() {
        neighbours[i] = neighbours[i] + *cell;
    }
}

fn remove_invalid(neighbours: &mut Neighbours, max_dims: &Point<i32>) {
    let mut j = neighbours.len();
    let mut i = 0;
    while i < j {
        while i < j && !is_valid_point(&neighbours[i], max_dims) {
            neighbours.swap_remove(i);
            j -= 1;
        }
        i += 1;
    }
}

pub fn remove_self(neighbours: &mut Neighbours, loc: &Point<i32>) {
    let mut j = neighbours.len();
    let mut i = 0;
    while i < j {
        while i < j && neighbours[i] == *loc {
            neighbours.swap_remove(i);
            j -= 1;
        }
        i += 1;
    }
}
pub fn is_valid_point(point: &Point<i32>, max_dims: &Point<i32>) -> bool {
    point.x() < max_dims.x() && point.y() < max_dims.y() && point.x() >= 0 && point.y() >= 0
}


#[cfg(test)]
mod tests {
    use super::*;

    fn print_neighbourhood(loc: Point<i32>, neighbours: Vec<Point<i32>>, size: i32) {
        let mut grid = Vec::new();

        for _ in 0..size {
            let mut cur = Vec::new();
            for _ in 0..size {
                cur.push(" . ");
            }
            grid.push(cur);
        }

        grid[loc.y() as usize][loc.x() as usize] = "\x1b[0;33m X \x1b[0m";

        for n in neighbours.iter() {
            grid[n.y() as usize][n.x() as usize] = "\x1b[0;91m X \x1b[0m";
        }

        for i in 0..size as usize {
            for j in 0..size as usize {
                print!("{}", grid[i][j]);
            }
            println!();
        }
    }

    #[test]
    fn add_chebyshev_distance_cells_test() {
        let mut neighbours = Vec::new();
        let loc = Point::new(0, 9);
        let size = 9;
        add_chebyshev_distance_cells(&mut neighbours, 2);
        convert_relative_point_to_absolute(&mut neighbours, &loc);
        remove_invalid(&mut neighbours, &Point::new(size, size));
        remove_self(&mut neighbours, &loc);
        // print_neighbourhood(loc, neighbours, size as i32);
    }

    #[test]
    fn add_manahattan_distance_cells_test() {
        let mut neighbours = Vec::new();
        let loc = Point::new(4, 4);
        let size = 9;
        add_manahattan_distance_cells(&mut neighbours, 3);
        // println!("{:?}", neighbours);
        convert_relative_point_to_absolute(&mut neighbours, &loc);
        remove_invalid(&mut neighbours, &Point::new(size, size));
        remove_self(&mut neighbours, &loc);
        print_neighbourhood(loc, neighbours, size as i32);
    }
}
