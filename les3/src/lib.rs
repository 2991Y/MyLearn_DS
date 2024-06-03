pub mod finder{

    pub fn sequential_search(nums:&[i32], num:i32)->bool{
        let mut pos = 0usize;
        let mut found = false;

        while pos < nums.len() && !found {
            found = nums[pos] == num; 
            pos+=1;
        }
        found
    }

    pub fn sequential_search_pos(nums:&[i32],num:i32)->Option<usize>{
        let mut pos = 0usize;
        let mut found = false;

        while pos < nums.len() && !found {
            if num == nums[pos] {
                found = true;
            }else{
                pos += 1;
            }
        }

        if found {
            Some(pos)
        }
        else{
            None
        }
    }

    pub fn ordered_sequential_search(nums:&[i32],num:i32)->bool{
        let mut pos = 0;
        let mut found = false;
        let mut stop = false;

        while pos < nums.len() && !found && !stop {
            if num == nums[pos]{
                found = true;
            }else if num < nums[pos] {
                stop = true;
            }else{
                pos += 1;
            }
        }
        found

    }


    pub fn ordered_binary_search1(nums:&[i32],num:i32)->bool{
        let mut low = 0;
        let mut high = nums.len() - 1;
        let mut found = false;

        while low <= high && !found {

            let mid = low + ((high - low) >> 1);

            if num == nums[mid] {
                found = true;
            }else if num < nums[mid]{
                high = mid -1;
            }else{
                low = mid +1;
            }
        }
        found
    }

    pub fn ordered_binary_search2(nums:&[i32],num:i32)->bool{
        if 0 == nums.len() {
            return false;
        }
        let mid = nums.len() >> 1;

        if num == nums[mid]{
            return true;
        }else if num < nums[mid] {
           return ordered_binary_search2(&nums[..mid], num);
        }else{
           return ordered_binary_search1(&nums[mid+1..], num);
        }
    }

    pub fn ordered_binary_search3(nums:&[i32],num:i32)->bool{
        if nums.is_empty() {
            return  false;
        }

        let mut low = 0;
        let mut high = nums.len() - 1;

        loop {
            let low_val = nums[low];
            let high_val = nums[high];

            if high <= low || num < low_val || num > high_val {
                break;
            }

            let offset = (num - low_val) * (high - low) as i32
             / (high_val - low_val);
             let interpolant = low + offset as usize;
             if nums[interpolant] > num {
                high = interpolant - 1;
             }else if nums[interpolant] < num {
                low = interpolant + 1;
             }
             else{
                return true;
             }
        }

        if num == nums[high] {
            return true;
        }else{
            return false;
        }
    }

    pub fn exponential_search(nums:&[i32],num:i32)->bool{
        let size = nums.len();
        if size == 0 {
            return false;
        }

        let mut high = 1;

        while high < size && nums[high] < num {
            high <<= 1;
        } 
        let low = high >> 1;

        ordered_binary_search1(&nums[low..size.min(high + 1)], num)
    }
}

