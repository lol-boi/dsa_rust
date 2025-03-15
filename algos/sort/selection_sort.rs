fn main() {
    let mut nums:[i32; 10] = [3,1,5,12,7,16,3,2,27,1];
    let mut min:usize; 
    for i in 0..nums.len()-1 {
        min = i;
        for j in i..nums.len(){
            if nums[min] > nums[j] {min = j}
        }
        nums.swap(i,min);
    } 
    for i in nums.iter(){
        print!("{} ",i);
    }
    println!("");
}
