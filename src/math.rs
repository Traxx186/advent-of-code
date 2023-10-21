pub fn median(numbers: &mut Vec<u32>) -> u32 {
    numbers.sort();

    let middle = numbers.len() / 2;
    if numbers.len() % 2 == 0 {
        ((numbers[middle - 1] + numbers[middle]) / 2) as u32
    } else {
        numbers[middle]
    }
}