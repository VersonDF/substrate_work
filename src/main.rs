fn bubble_sort(arr: &mut [i32]) {
    let mut swapped = true;
  
    while swapped {
        swapped = false;
        for i in 1..arr.len() {
            if arr[i - 1] > arr[i] {
                arr.swap(i - 1, i);
                swapped = true;
            }
        }
    }
  }
  
  fn main() {
    let mut arr = [5, 3, 8, 2, 1];
    bubble_sort(&mut arr);
    println!("{:?}", arr); 
  }