// safe-insert-sort.rs
// author: Seong Yong-ju <sei40kr@gmail.com>

fn safe_insert_sort<T: Ord>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let mut j = i;
        while 0 < j && arr[j] < arr[j - 1] {
            arr.swap(j, j - 1);
            j -= 1;
        }
    }
}

fn main() {
    {
        let mut array = [1, 2, 3, 4];
        safe_insert_sort(&mut array);
        println!("{:?}", array);
    }
    {
        let mut array = [4, 3, 2, 1];
        safe_insert_sort(&mut array);
        println!("{:?}", array);
    }
    {
        let mut array = [3, 2, 3, 0];
        safe_insert_sort(&mut array);
        println!("{:?}", array);
    }
    {
        let mut array = [3, 3, 6, 2, 1, 5, 7, 3, 1, 2];
        safe_insert_sort(&mut array);
        println!("{:?}", array);
    }
}
