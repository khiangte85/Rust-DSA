use std::time::Instant;

fn selection_sort(arr: &mut Vec<i32>) -> &Vec<i32> {
    for i in 0..arr.len() - 1 {
        let mut pos = i;

        for j in (i + 1)..arr.len() {
            if arr[j] < arr[pos] {
                pos = j;
            }
        }

        arr.swap(i, pos);
    }

    arr
}

fn main() {
    let time = Instant::now();
    let mut vec = vec![10, 3, 6, 99, 67, 20, 12, 120, 45, 32, 45];
    println!("{:?}", selection_sort(&mut vec));
    println!("Time {:?}", time.elapsed());
}
