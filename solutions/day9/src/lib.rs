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

pub fn get_extrapolated_value(history: &Vec<i64>) -> i64 {
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

    let last_num = sequences[idx][0];

    sequences[idx].push(last_num);

    for rev_idx in (0..idx).rev() {
        let prev_num = sequences[rev_idx + 1].last().unwrap();

        let new_num = sequences[rev_idx].last().unwrap() + prev_num;

        sequences[rev_idx].push(new_num);
    }

    *sequences[0].last().unwrap()
}
