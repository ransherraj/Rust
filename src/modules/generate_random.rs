use rand::Rng;

pub fn gen_random(left:i32, right:i32)-> i32{
    let rand_num = rand::thread_rng().gen_range(left..=right+1);
    return rand_num;
}