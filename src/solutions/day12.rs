use std::collections::{HashSet, VecDeque};

type Point = (usize, usize);

struct Grid {
    heightmap: Vec<Vec<u32>>,
    start: Point,
    end: Point,
}

impl Grid {
    fn new() -> Grid {
        Grid {
            heightmap: Vec::new(),
            start: (0, 0),
            end: (0, 0),
        }
    }

    fn parse_grid(self: &mut Grid, data: &str) {
        for line in data.lines() {
            let mut row = Vec::new();
            for h in line.chars() {
                if h == 'E' {
                    self.end = (row.len(), self.heightmap.len());
                    row.push(26);
                } else if h == 'S' {
                    self.start = (row.len(), self.heightmap.len());
                    row.push(0);
                } else {
                    row.push((h as u32) - 'a' as u32);
                }
            }
            self.heightmap.push(row);
        }
    }

    fn get_height(self: &Grid, x: usize, y: usize) -> u32 {
        *self.heightmap.get(y).unwrap().get(x).unwrap()
    }

    fn draw_route(self: &Grid, route: &Vec<Point>) {
        let alpha = "abcdefghijklmnopqrstuvwxyz";
        for (y, row) in self.heightmap.iter().enumerate() {
            for (x, height) in row.iter().enumerate() {
                if *height == 26 {
                    print!("E");
                } else if route.contains(&(x, y)) {
                    print!("{}", alpha.chars().nth(*height as usize).unwrap());
                } else {
                    print!(".");
                }
            }
            println!("");
        }

        println!("");
    }

    fn search_route(self: &Grid) -> Vec<Point> {
        self.search_route_from(self.start)
    }

    fn search_route_from(self: &Grid, start: Point) -> Vec<Point> {
        let dirs: Vec<(isize, isize)> = vec![(0, 1), (1, 0), (-1, 0), (0, -1)];
        let mut q: VecDeque<Vec<Point>> = VecDeque::new();
        let mut seen: HashSet<Point> = HashSet::new();

        q.push_back(vec![start]);
        while q.len() > 0 {
            let current_routes = q.pop_front().unwrap();
            let row = (*current_routes.last().unwrap()).1;
            let col = (*current_routes.last().unwrap()).0;
            let cur_pos = (col, row);
            if !seen.contains(&cur_pos) {
                seen.insert(cur_pos);
                if cur_pos == self.end {
                    return current_routes;
                }
                let height1 = self.get_height(col, row);
                for d in &dirs {
                    let new_pos = (
                        (cur_pos.0 as isize + d.0) as usize,
                        (cur_pos.1 as isize + d.1) as usize,
                    );
                    if new_pos.0 < self.heightmap[0].len() && new_pos.1 < self.heightmap.len() {
                        let height2 = self.get_height(new_pos.0, new_pos.1);
                        if height2 <= height1 + 1 {
                            let mut route_copy = current_routes.clone();
                            route_copy.push(new_pos);
                            q.push_back(route_copy);
                        }
                    }
                }
            }
        }
        vec![]
    }
}

fn process(data: &str) -> isize {
    let mut grid = Grid::new();
    grid.parse_grid(data);

    let route = grid.search_route();
    grid.draw_route(&route);
    route.len() as isize - 1
}

fn process_best_start_pos(data: &str) -> isize {
    let mut grid = Grid::new();
    grid.parse_grid(data);

    // Get a route for each 'a' height point
    let mut shortest_route = Vec::new();
    let mut shortest = usize::MAX;
    for (y, row) in grid.heightmap.iter().enumerate() {
        for (x, height) in row.iter().enumerate() {
            if *height == 0 {
                let route = grid.search_route_from((x, y));
                if route.len() > 0 && route.len() < shortest {
                    shortest_route = route;
                    shortest = shortest_route.len();
                }
            }
        }
    }

    grid.draw_route(&shortest_route);
    shortest_route.len() as isize - 1
}

#[test]
fn test_example() {
    let example = r#"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi"#;

    assert_eq!(process(example), 31);
    assert_eq!(process_best_start_pos(example), 29);
}

pub fn run(data: &str) -> String {
    let part1 = process(data);
    let part2 = process_best_start_pos(data);
    format!("Part 1: {} - Part 2: {}", part1, part2)
}
