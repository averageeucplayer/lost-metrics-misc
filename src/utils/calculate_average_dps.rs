pub fn calculate_average_dps(data: &[(i64, i64)], start_time: i64, end_time: i64) -> Vec<i64> {
    let step = 5;
    let mut results = vec![0; ((end_time - start_time) / step + 1) as usize];
    let mut current_sum = 0;
    let mut data_iter = data.iter();
    let mut current_data = data_iter.next();

    for t in (start_time..=end_time).step_by(step as usize) {
        while let Some((timestamp, value)) = current_data {
            if *timestamp / 1000 <= t {
                current_sum += value;
                current_data = data_iter.next();
            } else {
                break;
            }
        }

        results[((t - start_time) / step) as usize] = current_sum / (t - start_time + 1);
    }

    results
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use chrono::Utc;

    use super::*;
    
    #[test]
    fn should_return_average_dps() {
      
        let now = Utc::now();
        let start_time = now.timestamp();
        let data = vec![
            (start_time, 1e7 as i64),
            (start_time + 10000, 1e6 as i64)
        ];
        let end_time = (now + Duration::from_secs(300)).timestamp();
        let average_dps = calculate_average_dps(&data, start_time, end_time);
        assert_eq!(average_dps.first().unwrap(), &11000000);
    }
}