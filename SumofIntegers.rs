fn main() {
   let numbers = &[0u32, 1u32, 2u32];
   println!("result is {:?}", sum(numbers));
   let numbers = &[0u32, 1u32, u32::MAX];
   println!("result is {:?}", sum(numbers));

}

pub fn sum(list: &[u32]) -> Option<u32> {
    // list.iter().sum::<u32>();
    let mut result = 0u32;
    for item in list {
        if  result.checked_add(*item) == None {
            return None;
        }
        else
        {
           result += item;
        }
    }
    return Some(result);
}
