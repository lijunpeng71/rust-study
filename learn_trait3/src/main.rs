fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut larger = list[0];
    for &item in list.iter() {
        if item > larger {
            larger = item;
        }
    }
    larger
}

fn main() {
    let number_list = vec![1, 2, 23, 34, 9, 100];
    let larger = largest(&number_list);
    println!("number_list larger={}", larger);
}
