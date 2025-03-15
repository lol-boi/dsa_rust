fn main() {
    let mut nums:[i32; 10] = [3,1,5,12,7,16,3,2,27,1];
    for i in 0..nums.len(){
        for j in 0..nums.len() - i-1 {
            if nums[j] > nums[j+1] { nums.swap(j,j+1);} 
        } 
    } 
    for i in nums.iter(){
        print!("{} ",i);
    }
    println!("");
}


