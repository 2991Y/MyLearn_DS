pub mod queue;
/// stack
#[derive(Debug)]
pub struct Stack<T>{
    top:usize, // 栈顶
    data:Vec<T>,
}

impl<T> Stack<T>{
    pub fn new()->Self{
        Stack{
            top:0,
            data:Vec::new()
        }
    }

    pub fn push(&mut self,val:T){
        self.data.push(val);
        self.top +=1;
    }

    pub fn pop(&mut self)->Option<T>{
        if self.top == 0 {
            return None;
        }
        self.top -=1;
        self.data.pop()
    }

    pub fn peek(&self)->Option<&T>{
        if self.top == 0{
            return None;
        }else{
           return self.data.get(self.top - 1); // 通过栈顶访问 
        }
    }

    pub fn is_empty(&self)->bool{
        0 == self.top
    }

    pub fn size(&self)->usize{
        self.top
    }
}


/// 括号匹配
pub fn par_checker1(par:&str)->bool{
    let mut char_list = Vec::new();
    for c in par.chars(){
        char_list.push(c);
    }

    let mut stack = Stack::new();
    let mut pos = 0usize;
    let mut is_ok = true;

    while pos < char_list.len() && is_ok{
        let c = char_list[pos];

        if c == '(' || c == '[' || c == '{'{
            stack.push(c);
        }
        if  c == ')' || c == ']' || c == '}' {
            if stack.is_empty(){
                is_ok = false;
            }else{
                let r = stack.pop().unwrap();
                if !par_match(r, c) {
                    is_ok=false;
                }
            }
        }
        pos +=1;
    } 
    
    is_ok && stack.is_empty()
}

/// 开闭括号是否匹配
fn par_match(open:char,close:char)->bool{
    let opens = "([{";
    let closes = ")]}";
    // println!("open {open} is {:?}", opens.find(open));
    // println!("close {close} is {:?}", closes.find(close));
    opens.find(open) == closes.find(close)
}

/// 二进制转换
pub fn divide_by_two(mut dec_num:u32)->String{
    let mut rem_stack = Stack::new();

    while dec_num > 0{
        let rem = dec_num %2;
        rem_stack.push(rem);
        dec_num = dec_num / 2;
    }

    let mut bin_str = "".to_string();
    while !rem_stack.is_empty(){
        let rem = rem_stack.pop().unwrap().to_string();
        bin_str += &rem;
    }
    bin_str
}

/// 10 进制转换
pub fn base_converter(mut dec_num:u32,base:u32)->String{
    let digits = ['0',
    '1','2','3','4','5',
    '6','7','8','9','A',
    'B','C','D','E','F',
    ];
    let mut rem_stack = Stack::new();

    while dec_num > 0{
        let rem = dec_num % base;
        rem_stack.push(rem);
        dec_num = dec_num / base;
    }

    let mut bin_str = "".to_string();
    while !rem_stack.is_empty(){
        let rem = rem_stack.pop().unwrap() as usize;
        bin_str += &digits[rem].to_string();
    }
    bin_str 
}


use core::panic;
use std::{collections::HashMap};
/// 中缀表达式转后缀表达式
pub fn infix_to_postfix(infix:&str)->Option<String>{
    if !par_checker1(infix){
        return None;
    }

    let mut prec = HashMap::new();
    prec.insert("(", 1);
    prec.insert(")", 1);
    prec.insert("+", 2);
    prec.insert("-", 2);
    prec.insert("*", 3);
    prec.insert("/", 3);

    let mut op_stack = Stack::new();// 操作栈
    let mut postfix=Vec::new();

    for token in infix.split_whitespace(){
        if ("A" <= token && token <= "Z") ||
         ("0" <= token && token <= "9") {
            postfix.push(token);
         }
         else if "(" == token {
            op_stack.push(token);
         }else if ")" == token{
            let mut top = op_stack.pop().unwrap();
            while top!="(" {
                postfix.push(top);
                top = op_stack.pop().unwrap();
            }
         }else{
            while (!op_stack.is_empty()) &&
            (prec[op_stack.peek().unwrap()] >= prec[token]) {
                postfix.push(op_stack.pop().unwrap());
            }
            op_stack.push(token);
         }

    }

    while !op_stack.is_empty() {
        postfix.push(op_stack.pop().unwrap());
    }

    let mut postfix_str = "".to_string();
    for c in postfix{
        postfix_str += &c.to_string();
        postfix_str.push(' ');
    }

    Some(postfix_str)
}

/// 计算后缀表达式的值
pub fn postfix_eval(postfix:&str)->Option<i32>{
    // 空格分隔的表达式，不能小于5
    if postfix.len() < 5 {
        return  None;
    }

    let mut op_stack = Stack::new();

    for token in postfix.split_whitespace(){
        if "0" <= token && token <= "9"{
            op_stack.push(token.parse::<i32>().unwrap());
        }
        else{
            let op2 = op_stack.pop().unwrap();
            let op1 = op_stack.pop().unwrap();

            let res = do_calc(token,op1,op2);
            op_stack.push(res);
        }
    }
    op_stack.pop()
}

fn do_calc(op:&str,op1:i32,op2:i32)->i32{
    match op{
        "+" => op1 + op2,
        "-" => op1 - op2,
        "*" => op1 * op2,
        "/" => {
            if 0 == op2 {
                panic!("零不能作为除数，计算失败");
            }
            op1 / op2
        },
        ot @ _=>{
            panic!("无效的操作符：{}",ot);
        }
    }
}