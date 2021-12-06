use atoi::atoi;
use nom::*;
use std::iter;

pub fn solve_second_problem(data: String) -> usize {
    let map =
        data.as_bytes()
            .split(|b| *b == b'\n')
            .fold(vec![0u8; 1000 * 1000], |mut map, entry| {
                let ((x, y), (xx, yy)) = line(entry).unwrap().1;
                let range = |a: isize, b: isize| {
                    iter::successors(Some(a), move |n| Some(n + (b - a).signum()))
                };
                range(x, xx)
                    .cycle()
                    .zip(range(y, yy).cycle())
                    .take((x - xx).abs().max((y - yy).abs()) as usize + 1)
                    .for_each(|(x, y)| map[(x + y * 1000) as usize] += 1);
                map
            });

    map.into_iter().filter(|c| *c >= 2).count()
}

named!(isize<&[u8], isize>, map_opt!(nom::character::complete::digit1, atoi));
named!(coord<&[u8], (isize, isize)>, separated_pair!(isize, char!(','), isize));
named!(line<&[u8], ((isize, isize), (isize, isize))>, separated_pair!(coord, tag!(" -> "), coord));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_second_problem_works() {
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
        let result = solve_second_problem(input);

        assert_eq!(result, 9);
    }
}
