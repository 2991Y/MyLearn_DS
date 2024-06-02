use les2::Other::*;

fn main() {
    println!("Hello, dynamic plan !");

    let caches = [1,5,10,20,50];
    let amount = 9u32;
    // let caches_num = rec_mc1(&caches, amount);
    let mut min_cashes = [0;82];
    // let caches_num = rec_mc2(&caches, amount,&mut min_cashes);
    // let caches_num = dp_rec_mc(&caches, amount,&mut min_cashes);
    let mut caches_used = [0;82];
    let caches_num = dp_rec_mc_show(&caches, amount,&mut min_cashes,&mut caches_used);


    println!("需要{}张零钱",caches_num);
    print_caches(&caches_used, amount);

    println!("fib 10:{}", fibnacci_dp(10));
    println!("fib 10:{}", fibnacci_rec(10));
}

fn sum_loop(nums:&[i32])->i32{
    let mut sum = 0;
    for num in nums{
        sum += num;
    }
    sum
}

fn sum_not_loop1(nums:&[i32])->i32{
    if 1 == nums.len(){
        nums[0]
    }
    else{
        let first = nums[0];
        sum_not_loop1(&nums[1..]) + first
    }
}

fn sum_not_loop2(nums:&[i32])->i32{
    if 1 == nums.len(){
        nums[0]
    }
    else{
        let last = nums[nums.len() - 1];
        sum_not_loop2(&nums[..nums.len()-1]) + last
    }
}

fn sum_not_loop_tail(sum:i32, nums:&[i32])->i32{
    if 1 == nums.len(){
        sum + nums[0]
    }else{
        sum_not_loop_tail(sum + nums[nums.len()-1], &nums[..nums.len()-1])
    }
}

mod tests{
    use crate::*;
    use les2::Other::*;

    #[test]
    pub fn test_sum_loop(){
        let nums = [2,1,7,4,5];

        // let sum0 = sum_loop(&nums);
        let sum0 = sum_not_loop_tail(0, &nums);
        let sum1 = sum_not_loop1(&nums);
        let sum2 = sum_not_loop2(&nums);
        println!(" sum is {}, sum1 is {}, sum2 is {}", sum0,sum1,sum2);
        assert_eq!(sum0,sum1);
        assert_eq!(sum0,sum2);
    }

    #[test]
    pub fn test_num2str(){
        let num = 100;
        let sb =num2str_rec(num,2);
        let so = num2str_rec(num,8);
        let sh = num2str_rec(num,16);

        println!(" {num} is b{sb}, o{so}, x{sh}");
        assert_eq!(true,true);
    }
    #[test]
    pub fn test_move_tower(){
        move2tower(1, "A","B", "C");
        move2tower(2, "A","B", "C");
        move2tower(3, "A","B", "C");
        move2tower(4, "A","B", "C");
    }
}