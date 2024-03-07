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
    let mut vec: Vec<i32> = vec![5,10,1,4,11];
    println!("{:?}", selection_sort(&mut vec));
}
