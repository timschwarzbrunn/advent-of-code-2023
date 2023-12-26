use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Default)]
enum Task {
    #[default]
    First,
    Second,
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    // Two vectors are independent if they are not in the same direction.
    // We can test this by calculating the cross product. If it is a zero-vector,
    // they are not independent.
    fn independent(&self, other: &Self) -> bool {
        let cross_product = self.cross(&other);
        [
            cross_product.x >= f64::EPSILON,
            cross_product.y >= f64::EPSILON,
            cross_product.z >= f64::EPSILON,
        ]
        .iter()
        .any(|&v| v == true)
    }

    // Cross product of two vectors.
    fn cross(&self, rhs: &Self) -> Self {
        Self::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }

    // Dot product of two vectors.
    fn dot(&self, rhs: &Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    // Adding two vectors.
    fn add(&self, rhs: &Self) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }

    // Subtracting one vector from another.
    fn sub(&self, rhs: &Self) -> Self {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }

    // Multiplicating one vector by a scalar.
    fn mul(&self, f: f64) -> Self {
        Self::new(self.x * f, self.y * f, self.z * f)
    }

    // Divide one vector by a scalar.
    fn div(&self, d: f64) -> Self {
        Self::new(self.x / d, self.y / d, self.z / d)
    }

    // Create a linear combination from multiple vectors with multiple weights.
    // The result is another vector.
    fn linear_combination(vectors: Vec<Vec3>, factors: Vec<f64>) -> Vec3 {
        vectors.iter().zip(factors.iter()).fold(
            Vec3::new(0_f64, 0_f64, 0_f64),
            |acc_vector, (&vector, &factor)| acc_vector.add(&vector.mul(factor)),
        )
    }
}

// A hailstone has a position and velocity. We also calculate the 2D line in XY
// because we need it to check if the paths intersect.
#[derive(Debug)]
struct HailStone {
    p: Vec3,
    v: Vec3,
    m: f64, // y = m * x + n
    n: f64,
}

impl HailStone {
    fn from_line(line: String) -> Self {
        // Example line: 19, 13, 30 @ -2,  1, -2
        let (position, velocity) = line.split_once('@').unwrap();
        let mut position = position.split(',');
        let p = Vec3::new(
            position.next().unwrap().trim().parse::<f64>().unwrap(),
            position.next().unwrap().trim().parse::<f64>().unwrap(),
            position.next().unwrap().trim().parse::<f64>().unwrap(),
        );
        let mut velocity = velocity.split(',');
        let v = Vec3::new(
            velocity.next().unwrap().trim().parse::<f64>().unwrap(),
            velocity.next().unwrap().trim().parse::<f64>().unwrap(),
            velocity.next().unwrap().trim().parse::<f64>().unwrap(),
        );
        // y = m * x + n
        let m: f64 = v.y / v.x;
        let n: f64 = p.y - (m * p.x);
        Self { p, v, m, n }
    }

    fn will_cross_2d(&self, other: &Self, pos_min: f64, pos_max: f64) -> bool {
        if self.m == other.m {
            // Parallel.
            return false;
        }
        let x = (other.n - self.n) / (self.m - other.m);
        let y = self.m * x + self.n;
        if (x < self.p.x as f64 && self.v.x as f64 > 0_f64)
            || (x > self.p.x as f64 && (self.v.x as f64) < 0_f64)
            || (x < other.p.x as f64 && other.v.x as f64 > 0_f64)
            || (x > other.p.x as f64 && (other.v.x as f64) < 0_f64)
        {
            // Back in time.
            return false;
        }
        if x >= pos_min && x <= pos_max && y >= pos_min && y <= pos_max {
            // Inside test area.
            return true;
        } else {
            // Outside test area.
            return false;
        }
    }

    // Creates a plane that is created by two hailstone paths.
    // It is described by its normal and distance from the center of origin.
    fn create_plane_between_two_hailstone_paths(&self, other: &Self) -> (Vec3, f64) {
        let pos_diff = self.p.sub(&other.p);
        let vel_diff = self.v.sub(&other.v);
        let vel_normal = self.v.cross(&other.v);
        let plane_normal = pos_diff.cross(&vel_diff);
        (plane_normal, pos_diff.dot(&vel_normal))
    }
}

fn solve_first_task<B: BufRead>(reader: B, pos_min: f64, pos_max: f64) -> usize {
    let hailstones: Vec<HailStone> = reader
        .lines()
        .map(Result::unwrap)
        .map(HailStone::from_line)
        .collect();
    let mut result = 0;
    for (i, hailstone) in hailstones.iter().enumerate() {
        for other in hailstones.iter().skip(i + 1) {
            if hailstone.will_cross_2d(other, pos_min, pos_max) {
                result += 1;
            }
        }
    }
    result
}

fn solve_second_task<B: BufRead>(reader: B) -> usize {
    let hailstones: Vec<HailStone> = reader
        .lines()
        .map(Result::unwrap)
        .map(HailStone::from_line)
        .collect();
    // Find three independent hailstones.
    let hailstone_1 = hailstones.get(0).unwrap();
    let mut hailstone_2 = hailstones.get(1).unwrap();
    let mut hailstone_3 = hailstones.get(2).unwrap();
    for idx2 in 1..hailstones.len() {
        hailstone_2 = hailstones.get(idx2).unwrap();
        if hailstone_1.v.independent(&hailstone_2.v) {
            for idx3 in (idx2 + 1)..hailstones.len() {
                hailstone_3 = hailstones.get(idx3).unwrap();
                if hailstone_1.v.independent(&hailstone_3.v)
                    && hailstone_2.v.independent(&hailstone_3.v)
                {
                    break;
                }
            }
            break;
        }
    }
    // Create the three planes that are created by these three hailstones.
    let (plane_1_normal, plane_1_distance) =
        hailstone_1.create_plane_between_two_hailstone_paths(hailstone_2);
    let (plane_2_normal, plane_2_distance) =
        hailstone_1.create_plane_between_two_hailstone_paths(hailstone_3);
    let (plane_3_normal, plane_3_distance) =
        hailstone_2.create_plane_between_two_hailstone_paths(hailstone_3);
    // Now calculate the three vectors that descibe the direction of the intersecting
    // line between the planes.
    let plane_intersection_line_1_2 = plane_1_normal.cross(&plane_2_normal);
    let plane_intersection_line_3_1 = plane_3_normal.cross(&plane_1_normal);
    let plane_intersection_line_2_3 = plane_2_normal.cross(&plane_3_normal);
    // Now calculate the vector in between these intersecting lines.
    // They are weighted by the distance to the plane that is not involved
    // in the interseciton.
    // This is the path, the stone will fly.
    let stone_path = Vec3::linear_combination(
        vec![
            plane_intersection_line_2_3,
            plane_intersection_line_3_1,
            plane_intersection_line_1_2,
        ],
        vec![plane_1_distance, plane_2_distance, plane_3_distance],
    );
    // Calculate the velocity of the stone by calculating how fast the stone has to fly.
    let t = plane_1_normal.dot(&plane_2_normal.cross(&plane_3_normal));
    let stone_velocity = Vec3::new(
        f64::round(stone_path.x / t),
        f64::round(stone_path.y / t),
        f64::round(stone_path.z / t),
    );
    // We can now subtract the velocity of the stone from the velocities of the
    // hailstones. This allows us to determine where these two lines meet. This is
    // the initial position of the stone we throw.
    let h1_velocity_sub_stone_velocity = hailstone_1.v.sub(&stone_velocity);
    let h2_velocity_sub_stone_velocity = hailstone_2.v.sub(&stone_velocity);
    // Calculate the cross product of these two new velocity vectors. This is orthogonal.
    let new_velocity_vectors_orthogonal =
        h1_velocity_sub_stone_velocity.cross(&h2_velocity_sub_stone_velocity);
    let e =
        new_velocity_vectors_orthogonal.dot(&hailstone_2.p.cross(&h2_velocity_sub_stone_velocity));
    let f =
        new_velocity_vectors_orthogonal.dot(&hailstone_1.p.cross(&h1_velocity_sub_stone_velocity));
    let g = hailstone_1.p.dot(&new_velocity_vectors_orthogonal);
    let s = new_velocity_vectors_orthogonal.dot(&new_velocity_vectors_orthogonal);
    let rock = Vec3::linear_combination(
        vec![
            h1_velocity_sub_stone_velocity,
            h2_velocity_sub_stone_velocity,
            new_velocity_vectors_orthogonal,
        ],
        vec![e, -f, g],
    );
    ((rock.x + rock.y + rock.z) / s) as usize
}

fn main() {
    let mut args = std::env::args().skip(1);
    let filename = args.next().unwrap_or_else(|| String::from("./input"));
    let task = args
        .next()
        .map(|arg| match arg.as_str() {
            "first" => Task::First,
            "second" => Task::Second,
            _ => unreachable!(),
        })
        .unwrap_or_default();
    let reader = BufReader::new(File::open(filename).expect("Input file not found."));

    println!(
        "{:?} task solution: {:?}",
        task,
        match task {
            Task::First => solve_first_task(reader, 200000000000000_f64, 400000000000000_f64),
            Task::Second => solve_second_task(reader),
        }
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_combination() {
        let v1 = Vec3::new(1.0, 0.0, 0.0);
        let v2 = Vec3::new(0.0, 1.0, 0.0);
        let v3 = Vec3::new(2.0, 2.0, 2.0);
        assert_eq!(
            Vec3::linear_combination(vec![v1, v2, v3], vec![1_f64, -2_f64, 4_f64]),
            Vec3::new(9.0, 6.0, 8.0)
        );
    }

    #[test]
    fn test_first_task() {
        let reader = BufReader::new(File::open("./input.test").expect("Input file not found."));
        assert_eq!(solve_first_task(reader, 7_f64, 27_f64), 2);
    }

    #[test]
    fn test_second_task() {
        let reader = BufReader::new(File::open("./input.test").expect("Input file not found."));
        assert_eq!(solve_second_task(reader), 47);
    }
}
