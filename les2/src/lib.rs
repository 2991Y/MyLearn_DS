pub mod Other{

    const BASESTR:[&str;16] = [
        "0","1","2","3","4",
        "5","6","7","8","9",
        "A","B","C","D","E","F"
    ];

    /// 进制输出
    pub fn num2str_rec(num:i32, base:i32)->String{
        if num < base {
            BASESTR[num as usize].to_string()
        }else{
            num2str_rec(num / base, base) + 
                BASESTR[(num % base) as usize]
        }
    }

    /// 汉诺塔
    pub fn move2tower(height:u32, str_p:&str, des_p:&str, mid_p:&str){
        if height >=1 {
            move2tower(height - 1, str_p, mid_p, des_p);
            println!("moveing disk from {str_p} to {des_p}");
            move2tower(height-1, mid_p, des_p, str_p);
        }
    }

    /// 人民币找零,
    pub fn rec_mc1(caches:&[u32], amount:u32)->u32{
        let mut min_caches = amount;

        if caches.contains(&amount){
            return  1;
        }else{
            for c in caches.iter()
             .filter(|&c|*c<=amount)
             .collect::<Vec<&u32>>() {

                let num_cashes = 1 + rec_mc1(&caches, amount - c);

                if num_cashes < min_caches {
                    min_caches = num_cashes;
                }
            }
        }
        min_caches
    }
    // 人民币找零 ， 缓存
    pub fn rec_mc2(caches:&[u32], amount:u32, min_caches:&mut [u32])->u32{
        let mut min_caches_num = amount;

        if caches.contains(&amount){
            min_caches[amount as usize] = 1;
            return 1;
        }else if min_caches[amount as usize] > 0 {
            return  min_caches[amount as usize];
        }
        else {
            for c in caches.iter()
                .filter(|&c|*c <= amount)
                .collect::<Vec<&u32>>(){
                    let cache_num = 1 + rec_mc2(caches, amount - c, min_caches);
                    if cache_num < min_caches_num {
                        min_caches_num = cache_num;
                       min_caches[amount as usize] = min_caches_num; 
                    }
                }
        }
        min_caches_num
    }

    /// 人民币找零 动态规划
    pub fn dp_rec_mc(caches:&[u32],amount:u32,min_caches:&mut [u32])->u32{
        for denm in 1..=amount{
            let mut min_cache_num = denm;
            for c in caches.iter()
                .filter(|&c|*c<= denm)
                .collect::<Vec<&u32>>(){
                    let index = (denm -c) as usize;
                    let caches_num = 1 + min_caches[index];
                    if caches_num < min_cache_num{
                        min_cache_num = caches_num;
                    }
                }
                min_caches[denm as usize] = min_cache_num;
        }

        min_caches[amount as usize]
    }


    /// 人民币找零 动态规划, 返回使用的零钱
    pub fn dp_rec_mc_show(caches:&[u32],amount:u32,min_caches:&mut [u32], caches_used:&mut [u32])->u32{
        for denm in 1..=amount {
            let mut min_cache_num = denm;
            let mut used_cache = 1;
            for c in caches.iter()
            .filter(|&c|*c<=denm)
            .collect::<Vec<&u32>>(){
                let index = (denm - c) as usize;
                let cache_num = 1 + min_caches[index];

                if cache_num < min_cache_num {
                    min_cache_num = cache_num;
                    used_cache = *c;
                }
            } 

            min_caches[denm as usize] = min_cache_num;
            caches_used[denm as usize] = used_cache;
        }

        min_caches[amount as usize]
    }

    /// 打印零钱
    pub fn print_caches(caches_used:&[u32], mut amount:u32){
        while amount > 0 {
            let curr = caches_used[amount as usize];
            println!("￥{curr}");
            amount -= curr;
        }
    }

    pub fn fibnacci_rec(n:u32)->u32{
        if n == 1 || n == 2 {
           1 
        }
        else{
            fibnacci_rec(n-1) + fibnacci_rec(n-2)
        }
    }

    pub fn fibnacci_dp(n:u32)->u32{
        let mut dp =[1,1];
        for i in 2..=n{
            let idx1 = (i%2) as usize;
            let idx2 = ((i-1)%2) as usize;
            let idx3 = ((i-2)%2) as usize;
            dp[idx1] = dp[idx2] + dp[idx3];

        }
        dp[((n-1)%2) as usize]
    }


}