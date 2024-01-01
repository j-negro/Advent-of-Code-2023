pub fn parse_file(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|line| {
            line.trim()
                .split_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

pub fn construct_extrapolation_sequences(history: &Vec<i64>) -> Vec<Vec<i64>> {
    let mut sequences: Vec<Vec<i64>> = vec![history.clone()];

    let mut idx = 0;

    loop {
        let mut new_sequence = Vec::new();

        for i in 1..sequences[idx].len() {
            new_sequence.push(sequences[idx][i] - sequences[idx][i - 1]);
        }

        if new_sequence.iter().all(|v| *v == 0) {
            break;
        }

        sequences.push(new_sequence);
        idx += 1;
    }
    sequences
}
