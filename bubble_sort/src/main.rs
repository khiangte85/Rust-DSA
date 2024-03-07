fn bubble_sort(arr: &mut Vec<i32>) -> &Vec<i32> {
    for i in 0..arr.len() {
        for j in 0..arr.len() - (i + 1) {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }

    arr
}

fn main() {
    let mut vec = vec![10, 3, 6, 99, 67, 20, 12, 120, 45, 32, 45];
    println!("{:?}", bubble_sort(&mut vec));
}
