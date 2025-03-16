fn main() { 
    let mut nums:[i32; 10] = [3,1,5,12,7,16,3,2,27,1]; 
    //let mut tmp:usize; 
    for i in 1..nums.len(){
        for j in (0..i).rev(){
            if nums[j] > nums[j+1] {
                nums.swap(j,j+1);
            }else{
                break; 
            } 
        } 
    }
    for i in nums.iter(){
        print!("{} ",i);
    }
    println!("");
}
