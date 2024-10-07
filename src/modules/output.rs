pub fn print_2d_array(arr:&[Vec<i32>]){
    for row in arr{
        println!("{:?}", row);
    }
}

pub fn print_1d_array(arr:&[i32]){
    println!("{:?}", arr)
}

pub fn print_3d_array(arr:&[Vec<Vec<i32>>]){
    for two in arr{
        for row in two{
            println!("{:?}", arr)
        }
    } 
}