

pub mod depth {
    pub fn count_increased_depth(input: &String) -> u32 {
        let mut values: Vec<u32> = input.clone().lines().map(|x| x.parse::<u32>().unwrap()).collect();
        let copy_of_values = values.clone();
        values.drain(1..values.len()).into_iter().zip(copy_of_values.into_iter()).filter(|(second,first)| second > first).count() as u32
    }

    pub fn count_increased_depth_intervals(input: &String) -> u32 {
        let mut values: Vec<u32> = input.clone().lines().map(|x| x.parse::<u32>().unwrap()).collect();
        let mut copy_of_values = values.clone();
        let pairs = values.drain(1..values.len()).into_iter().zip(copy_of_values.clone().into_iter());
        pairs.into_iter().zip(copy_of_values.drain(2..copy_of_values.len()).into_iter())//.for_each(|x| println!("{:?} {}", x, x.0.0 + x.0.1 + x.1));
             .fold((0,u32::MAX), |(count,last),((second,first),third)|{
                 let res = if first + second + third > last {
                     (count+1,first+second+third)
                 } else {
                    (count,first+second+third)
                 };
                 res
             } ).0
    }
}