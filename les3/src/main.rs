use les3::finder::*;
fn main() {
    println!("Hello, Search!");
    // seq_search_run();
    // seq_search2_run();
    // ordered_sequential_search_run();
    // ordered_binary_search_run();
    // ordered_binary_search_rev_run();
    // interpolant_search_run();
    exponential_search_run();
}

#[allow(dead_code)]
fn seq_search_run(){
    let num = 8;
    let nums = [9,3,7,3,10,5,6,2,8];

    let found = sequential_search(&nums,num);
    println!("集合中存在{num}? {found}");
}
#[allow(dead_code)]
fn seq_search2_run(){
    let num = 8;
    let nums = [9,3,7,3,10,5,6,2,8];

    match sequential_search_pos(&nums, num){
        Some(pos)=>{
            println!("{num} 是 {:?}的第{pos}项", nums);
        }
        None=>println!("{num} 不在集合中!"),
    }
}

#[allow(dead_code)]
fn ordered_sequential_search_run(){
    let nums = [1,3,8,10,15,24,32,44,56,63,79];
    println!("{:?}",nums);
    let num = 32;
    let found = ordered_sequential_search(&nums, num);
    println!(" {num} 在？ {found} ");
    let num = 33;
    let found = ordered_sequential_search(&nums, num);
    println!(" {num} 在？ {found} ");

}

#[allow(dead_code)]
fn ordered_binary_search_run(){
    let nums = [1,3,8,10,15,24,32,44,56,63,79];
    println!("{:?}",nums);
    let num = 3;
    let found =ordered_binary_search1(&nums, num);
    println!(" {num} 在？ {found} ");
    let num = 66;
    let found = ordered_binary_search1(&nums, num);
    println!(" {num} 在？ {found} ");
}

#[allow(dead_code)]
fn ordered_binary_search_rev_run(){
    let nums = [1,3,8,10,15,24,32,44,56,63,79];
    println!("{:?}",nums);
    let num = 3;
    let found =ordered_binary_search2(&nums, num);
    println!(" {num} 在？ {found} ");
    let num = 66;
    let found = ordered_binary_search2(&nums, num);
    println!(" {num} 在？ {found} ");
}

#[allow(dead_code)]
fn interpolant_search_run(){
    let nums = [1,3,8,10,15,24,32,44,56,63,79];
    println!("{:?}",nums);
    let num = 3;
    let found =ordered_binary_search3(&nums, num);
    println!(" {num} 在？ {found} ");
    let num = 66;
    let found = ordered_binary_search3(&nums, num);
    println!(" {num} 在？ {found} ");
}

#[allow(dead_code)]
fn exponential_search_run(){
    let nums = [1,3,8,10,15,24,32,44,56,63,79];
    println!("{:?}",nums);
    let num = 3;
    let found =exponential_search(&nums, num);
    println!(" {num} 在？ {found} ");
    let num = 66;
    let found = ordered_binary_search3(&nums, num);
    println!(" {num} 在？ {found} ");
}