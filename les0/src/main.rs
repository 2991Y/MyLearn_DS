use std::{char::ToLowercase, time::SystemTime};

fn main() {
    let s1 = "rust";
    let s2 = "trus";
    run_time("检查法", ||is_disorder(s1, s2));
    run_time("排序比较法", ||is_disorder2(s1, s2));
    run_time("计数比较法", ||is_disorder3(s1, s2));
}
// 检查法：
// 串1 中的字符在串2中出现。每次检查时将串2中查找到的字符☞' '
fn is_disorder(s1:&str,s2:&str)->bool{
    if s1.len() != s2.len() {
        return false;
    }

    let  alist:Vec<char> = s1.chars().collect();
    let mut blist:Vec<char> = s2.chars().collect();

    let mut pos1:usize = 0;
    let mut ok = true;

    while pos1 < alist.len() && ok {
        let mut pos2 = 0usize;

        let mut found = false;
        while pos2<blist.len() && !found {
            if alist[pos1] == blist[pos2] {
                found = true;
                blist[pos2] = ' ';
            }
            else{
                pos2+=1;
            }
        }
        pos1+=1;

        ok = found;
    }

    ok
}
// 排序比较
fn is_disorder2(s1:&str,s2:&str)->bool{
    if s1.len() !=s2.len(){
        return  false;
    }

    let mut alist = s1.chars().collect::<Vec<char>>();
    let mut blist = s2.chars().collect::<Vec<char>>();

    alist.sort();
    blist.sort();

    let mut pos = 0usize;
    let mut isok = true;

    while pos < alist.len() && isok{
        isok = alist[pos] == blist[pos] ;
        pos+=1;
    }
    isok
}
// 计数比较法
fn is_disorder3(s1:&str,s2:&str)->bool{
    if s1.len() != s2.len(){
        return  false;
    }

    let mut c1 = [0;26];
    let mut c2 = [0;26];

    for c in s1.chars(){

        let pos = (c.to_ascii_lowercase() as usize) - 97;
        c1[pos] +=1;
    }
    for c in s2.chars(){
        let pos = (c.to_ascii_lowercase() as usize) - 97;
        c2[pos] +=1;
    }

    let mut pos = 0;
    let mut is_ok = true;

    while pos < 26 && is_ok{
        if c1[pos] == c2[pos]{
            pos +=1;
        }
        else{
            is_ok= false;
        }
    }

    is_ok
}

fn run_time(func_name:&str,fun:impl Fn()->bool){
    let now = SystemTime::now();
    let is_ok = fun();
    let dur = now.elapsed().unwrap();
    println!("{}:{} used {}ns", func_name, is_ok,dur.as_nanos());
}