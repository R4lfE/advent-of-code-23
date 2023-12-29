use std::{error::Error, fs, ops::{Mul, Add, Sub, SubAssign, Neg, AddAssign, DivAssign, Div}};

use num::Integer;

#[derive(Clone, Copy, Debug, PartialEq)]
struct Vec3(i128, i128, i128);

impl From<&str> for Vec3 {
    fn from(str: &str) -> Self {
        let vec: Vec<i128> = str
            .replace(' ', "")
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();
        Self(vec[0], vec[1], vec[2])
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        *self = *self + rhs;
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        *self = *self - rhs;
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3(-self.0, -self.1, -self.2)
    }
}

impl Mul<i128> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: i128) -> Self::Output {
        Vec3(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl Div<i128> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: i128) -> Self::Output {
        Vec3(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

impl DivAssign<i128> for Vec3 {
    fn div_assign(&mut self, rhs: i128) {
        self.0 /= rhs;
        self.1 /= rhs;
        self.2 /= rhs;
    }
}

impl Vec3 {
    fn cross(a: Self, b: Self) -> Self {
        Vec3(
            a.1 * b.2 - a.2 * b.1,
            a.2 * b.0 - a.0 * b.2,
            a.0 * b.1 - a.1 * b.0
        )
    }

    fn dot(a: Self, b: Self) -> i128 {
        a.0 * b.0 + a.1 * b.1 + a.2 * b.2
    }

    fn reduce(&mut self) {
        let gcd = self.0.gcd(&self.1.gcd(&self.2));
        *self /= gcd;
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Hailstone {
    position: Vec3,
    velocity: Vec3
}

impl From<&str> for Hailstone {
    fn from(str: &str) -> Self {
        let split: Vec<&str> = str.split('@').collect();
        Self {
            position: Vec3::from(split[0]),
            velocity: Vec3::from(split[1]),
        }
    }
}

impl Hailstone {
    fn new(position: Vec3, velocity: Vec3) -> Self {
        Self {
            position,
            velocity
        }
    }

    fn intersects_at(&self, other: &Self) -> Option<(i128, i128)> {
        // check if parallel
        if self.velocity.0 * other.velocity.1 == self.velocity.1 * other.velocity.0 {
            return None;
        }

        let t_other = (self.position.1 as f64 - other.position.1 as f64 + self.velocity.1 as f64 * (other.position.0 - self.position.0) as f64 / self.velocity.0 as f64)
                    / (other.velocity.1 as f64 - (self.velocity.1 * other.velocity.0) as f64 / self.velocity.0 as f64);
        let t_self = (other.position.0 as f64 - self.position.0 as f64 + other.velocity.0 as f64 * t_other) / self.velocity.0 as f64;
        Some((t_self as i128, t_other as i128))
    }

    fn at_time(&self, t: i128) -> Vec3 {
        self.position + self.velocity * t
    }
}

fn observe_hailstorm(input: &str) -> Vec<Hailstone> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.trim())
        .map(Hailstone::from)
        .collect()
}

fn part1(input: &str, bounds: (i128, i128)) -> usize {
    let hailstones = observe_hailstorm(input);

    let mut count = 0;
    for (i, hailstone) in hailstones.iter().enumerate() {
        for other in hailstones.iter().skip(i + 1) {

            if let Some(times) = hailstone.intersects_at(other) {
                if times.0 > 0_i128 && times.1 > 0_i128 {
                    
                    let intersection = hailstone.at_time(times.0);
                    if bounds.0 < intersection.0 && intersection.0 < bounds.1
                    && bounds.0 < intersection.1 && intersection.1 < bounds.1 {
                        count += 1;
                    }
                }
            }
        }
    }

    count
}

fn part2(input: &str) -> i128 {
    let hailstones: Vec<Hailstone> = observe_hailstorm(input);
    
    // take first three hailstones
    let p1 = hailstones[0].position;
    let v1 = hailstones[0].velocity;

    let p2 = hailstones[1].position;
    let v2 = hailstones[1].velocity;

    let p3 = hailstones[2].position;
    let v3 = hailstones[2].velocity;

    // vectors from p1 to the second hailstone's trajectory
    let q20 = p2 - p1;
    let q21 = q20 + (v2 - v1);

    // compute normal of the plane from p1 to the second hailstone's trajectory
    let mut n2 = Vec3::cross(q21, q20);
    n2.reduce();

    // vectors from p1 to the third hailstone's trajectory
    let q30 = p3 - p1;
    let q31 = q30 + (v3 - v1);

    // compute normal of the plane from p1 to the third hailstone's trajectory
    let mut n3 = Vec3::cross(q31, q30);
    n3.reduce();

    // compute the intersection of the planes
    let mut v = Vec3::cross(n2, n3);
    v.reduce();

    // compute the normal of the plane from the intersection and the velocity of p
    let mut np = Vec3::cross(v, v1);
    np.reduce();

    // compute the intersection of the second hailstone's trajectory and the plane
    let t2 = Vec3::dot(-q20, np) / Vec3::dot(v2, np);
    let i2 = p2 + v2 * t2;

    // compute the intersection of the third hailstone's trajectory and the plane
    let t3 = Vec3::dot(-q30, np) / Vec3::dot(v3, np);
    let i3 = p3 + v3 * t3;

    // compute the trajectory and start position of the rock
    let mut rock = Hailstone::new(if t2 > t3 { i3 } else { i2 }, (i3 - i2) / (t3 - t2));
    rock.position -= rock.velocity * t2.min(t3);

    rock.position.0 + rock.position.1 + rock.position.2
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input.txt")?;
    
    dbg!(part1(&input, (200000000000000_i128, 400000000000000_i128)));
    dbg!(part2(&input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = r#"19, 13, 30 @ -2,  1, -2
        18, 19, 22 @ -1, -1, -2
        20, 25, 34 @ -2, -2, -4
        12, 31, 28 @ -1, -2, -1
        20, 19, 15 @  1, -5, -3"#;
        assert_eq!(part1(input, (7_i128, 27_i128)), 2);
    }

    #[test]
    fn part_2() {
        let input = r#"19, 13, 30 @ -2,  1, -2
        18, 19, 22 @ -1, -1, -2
        20, 25, 34 @ -2, -2, -4
        12, 31, 28 @ -1, -2, -1
        20, 19, 15 @  1, -5, -3"#;
        assert_eq!(part2(input), 47);
    }
}