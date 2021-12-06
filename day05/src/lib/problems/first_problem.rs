use atoi::atoi;
use nom::*;

pub fn solve_first_problem(data: String) -> usize {
    let map = data
        .as_bytes()
        .split(|b| *b == b'\n')
        .map(|entry| {
            let ((x, y), (xx, yy)) = line(entry).unwrap().1;
            (x.min(xx), y.min(yy), x.max(xx), y.max(yy))
        })
        .filter(|(x, y, xx, yy)| x == xx || y == yy)
        .fold(vec![0u8; 1000 * 1000], |mut map, (x, y, xx, yy)| {
            if x == xx {
                (y..=yy).for_each(|y| map[x + y * 1000] += 1);
            } else {
                (x..=xx).for_each(|x| map[x + y * 1000] += 1);
            }
            map
        });

    map.into_iter().filter(|c| *c >= 2).count()
}

named!(usize<&[u8], usize>, map_opt!(nom::character::complete::digit1, atoi));
named!(coord<&[u8], (usize, usize)>, separated_pair!(usize, char!(','), usize));
named!(line<&[u8], ((usize, usize), (usize, usize))>, separated_pair!(coord, tag!(" -> "), coord));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_first_problem_works() {
        let input = r##"456,846 -> 221,846
980,926 -> 73,19
682,930 -> 562,930
766,592 -> 274,100
247,685 -> 247,21
106,800 -> 635,800
953,340 -> 135,340
293,223 -> 293,12
454,196 -> 454,463
886,766 -> 164,766"##
            .to_string();
        let result = solve_first_problem(input);

        assert_eq!(result, 2);
    }
}
