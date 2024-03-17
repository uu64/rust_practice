use std::collections::HashMap;


#[allow(dead_code)]
#[derive(Debug,PartialEq)]
struct Stats {
    mean: f64,
    med: i64,
    mode: i64,
}

#[allow(dead_code)]
fn get_stats(numbers: &[i64]) -> Stats {
    if numbers.len() == 0 {
        return Stats {mean: 0.0, med: 0, mode: 0};
    }

    let mut numbers_vec = numbers.to_vec();
    numbers_vec.sort();

    let mut counter = HashMap::new();
    let mut sum = 0;
    for n in numbers_vec.iter() {
        counter.entry(*n)
            .and_modify(|e| { *e += 1})
            .or_insert(1);
        sum += n;
    }

    let mut counter_entries = counter.iter()
        .collect::<Vec<(&i64, &i64)>>();
    counter_entries.sort_by(|a, b| if a.1 == b.1 {
        a.0.cmp(b.0)
    } else {
        a.1.cmp(b.1).reverse()
    });

    return Stats {
        mean: (sum / numbers.len() as i64) as f64,
        med: numbers_vec[numbers_vec.len()/2],
        mode: *counter_entries[0].0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pass_one_number() {
        let numbers = vec![1];
        let result = get_stats(&numbers);
        assert_eq!(result, Stats {mean: 1.0, med: 1, mode: 1});
    }

    #[test]
    fn pass_two_numbers() {
        let numbers = vec![1, 3];
        let result = get_stats(&numbers);
        assert_eq!(result, Stats {mean: 2.0, med: 3, mode: 1});
    }


    #[test]
    fn pass_multiple_numbers() {
        let numbers = vec![1, 7, 7, 7, 4, 5, 4];
        let result = get_stats(&numbers);
        assert_eq!(result, Stats {mean: 5.0, med: 5, mode: 7});
    }

    #[test]
    fn pass_empty_list() {
        let numbers: Vec<i64> = Vec::new();
        let result = get_stats(&numbers);
        assert_eq!(result, Stats {mean: 0.0, med: 0, mode: 0});
    }
}
