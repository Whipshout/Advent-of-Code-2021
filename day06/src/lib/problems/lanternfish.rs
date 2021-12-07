#[derive(Debug)]
pub struct Pool(pub [isize; 9]);

impl Pool {
    pub fn create_pool(data: &str) -> Self {
        let map = data.split(',').fold([0; 9], |mut map, n| {
            map[n.parse::<usize>().unwrap()] += 1;
            map
        });

        Pool(map)
    }

    pub fn complete_cycles(&mut self, times: usize) {
        let pool = self;
        (1..times).for_each(|day| pool.0[(day + 7) % 9] += pool.0[day % 9]);
    }

    pub fn total_count(&self) -> isize {
        self.0.into_iter().sum::<isize>()
    }
}
