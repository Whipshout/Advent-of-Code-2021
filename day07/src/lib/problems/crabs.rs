pub struct Crabs {
    pub positions: Vec<i32>,
    pub lowest_fuel: i32,
}

impl Crabs {
    pub fn new(positions: Vec<i32>) -> Self {
        Self {
            positions,
            lowest_fuel: 0,
        }
    }

    pub fn max_position(&self) -> i32 {
        *self.positions.iter().max().unwrap()
    }

    pub fn calculate_fuel(&self, index: i32) -> i32 {
        self.positions
            .iter()
            .fold(0, |sum, val| sum + ((val - index).abs()))
    }

    pub fn calculate_fuel_exponential(&self, index: i32) -> i32 {
        self.positions.iter().fold(0, |sum, val| {
            let steps = (val - index).abs();
            sum + (0..=steps).sum::<i32>()
        })
    }

    pub fn calculate_lowest_fuel(&mut self, max: i32, is_exponential: bool) {
        self.lowest_fuel = (0..=max)
            .map(|n| {
                if is_exponential {
                    Crabs::calculate_fuel_exponential(self, n)
                } else {
                    Crabs::calculate_fuel(self, n)
                }
            })
            .min()
            .unwrap();
    }
}
