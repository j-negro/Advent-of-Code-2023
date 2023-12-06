pub fn multiply_ways_to_win(races: Vec<PreviousRace>) -> u64 {
    return races
        .iter()
        .map(|race| race.possible_charge_times())
        .fold(1, |acc, x| acc * x);
}


#[derive(Debug)]
pub struct PreviousRace {
    time: u64,
    distance: u64,
}

impl PreviousRace {
    pub fn new(time: u64, distance: u64) -> Self {
        Self {
            time: time,
            distance: distance,
        }
    }

    fn possible_charge_times(&self) -> u64 {
        let discriminant: f64 = (self.time.pow(2) as f64) - 4.0 * (self.distance as f64);

        if discriminant < 1e-8 {
            return 0;
        }

        let delta = (discriminant as f64).sqrt();

        let range_start = ((self.time as f64) - delta) / 2.0;
        let range_end = ((self.time as f64) + delta) / 2.0;

        let range_start = if range_start.fract() == 0.0 {
            (range_start.ceil() as u64) + 1
        } else {
            range_start.ceil() as u64
        };

        let range_end = if range_end.fract() == 0.0 {
            (range_end.floor() as u64) - 1
        } else {
            range_end.floor() as u64
        };

        return range_end - range_start + 1;
    }
}