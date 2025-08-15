use rand::Rng;

pub fn find_min_adjacent_and_print() {
    let mut rng = rand::thread_rng();
    let data: Vec<u32> = (0..20).map(|_| rng.gen_range(0..100)).collect();

    let idx_str = (0..data.len())
        .map(|i| i.to_string())
        .collect::<Vec<_>>()
        .join(" ");
    println!("indexes: {}", idx_str);

    let data_str = data
        .iter()
        .map(u32::to_string)
        .collect::<Vec<_>>()
        .join(" ");
    println!("data:    {}", data_str);

    let (min_idx, min_sum) = data
        .windows(2)
        .enumerate()
        .map(|(i, w)| (i, w[0] + w[1]))
        .min_by_key(|&(_, sum)| sum)
        .unwrap();

    println!(
        "min adjacent sum={}+{}={} at indexes:{},{}",
        data[min_idx],
        data[min_idx + 1],
        min_sum,
        min_idx,
        min_idx + 1
    );
}
