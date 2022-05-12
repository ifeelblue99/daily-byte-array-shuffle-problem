#[allow(unused)]

fn main() {
  use SortArray::sort_according_to_even_odd as sort;
  let data = [4, 2, 3, 1, 7, 0];
  println!("{:?}", sort(&data));
}

pub mod SortArray {
  pub fn sort_according_to_even_odd(arr: &[i32]) -> Vec<i32> {
    let mut new_arr: Vec<i32> = Vec::new();
    let mut odds_arr: Vec<i32> = Vec::new();
    let mut even_arr: Vec<i32> = Vec::new();
      
    for (indx, el) in arr.iter().enumerate() {
      if el%2==0 {
        even_arr.push(*el);
        continue
      }
      odds_arr.push(*el)
    }
    merge_arrays(odds_arr, even_arr)
  }
  pub fn merge_arrays(odds: Vec<i32>, evens: Vec<i32>) -> Vec<i32> {
    let mut merged_arr: Vec<i32> = Vec::new();
    for indx in 0..evens.len() {
      merged_arr.push(evens[indx]);
      merged_arr.push(odds[indx]);
    }
    merged_arr
  }  
}
