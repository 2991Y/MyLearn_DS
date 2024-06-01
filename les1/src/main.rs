use les1::{base_converter, divide_by_two, infix_to_postfix, par_checker1, postfix_eval, queue::queue::{self, pal_checker, Deque, Queue}, Stack};
fn main() {
    println!("Hello, line struct !");

    // stack example 
    // stack_run()
    // par_check_run();
    // multi_par_check_run();
    // divide_by_two_run();
    // base_converter_run();
    // infix_to_postfix_run();
    // postfix_val_run();

    // queue_run();
    // hot_run();


    // dequeue_run();
    pal_check_run();
}

#[allow(dead_code)]
fn stack_run(){
    let mut s = Stack::new();

    s.push(1);
    s.push(2);
    s.push(4);

    println!("栈顶:{}, 数量：{}", s.peek().unwrap(),s.size());
    println!("栈顶:{}, 数量：{}", s.pop().unwrap(),s.size());
    println!("空:{}, Stack：{:?}", s.is_empty(),s);
}

#[allow(dead_code)]
fn par_check_run(){
    let sa = "()(())";
    let sb = "()((()";
    let res1 = par_checker1(sa);
    let res2 = par_checker1(sb);

    println!("sa 平衡性：{}, sb 平衡性:{}", res1,res2);
}

#[allow(dead_code)]
fn multi_par_check_run(){
    let sa = "(2+3){fun}{fun}[abc]";
    let sb = "(2+4)*4{3"; 
    let res1 = par_checker1(sa);
    let res2 = par_checker1(sb);

    println!("sa 平衡性：{}, sb 平衡性:{}", res1,res2);
}

#[allow(dead_code)]
fn divide_by_two_run(){
    let bin_str = divide_by_two(100);
    println!("100 us b'{}'", bin_str);
}

#[allow(dead_code)]
fn base_converter_run(){
    let bin_str = base_converter(100,16);
    println!("100 us x'{}'", bin_str);
}

#[allow(dead_code)]
fn infix_to_postfix_run(){
    let infix = "( A + B ) * ( C + D )";
    let postfix = infix_to_postfix(&infix);

    match postfix{
        Some(val)=>{
            println!("中缀:{} -> 后缀:{}",infix,val);
        }
        None=>{
            println!("中缀{}不是有效的中缀!",infix);
        }
    }
}

#[allow(dead_code)]
fn postfix_val_run(){
    let postfix = " 1 2 + 1 2 + *";
    let res = postfix_eval(postfix);
    match res {
        Some(val)=>{
            println!("{} = {}",postfix,val);
        }
        None=>{
            println!("{} 不是有效的表达式",postfix);
        }
    }
}

#[allow(dead_code)]
fn queue_run(){
    let mut q = Queue::new(3);
    let _r1 = q.enqueue(1);
    let _r2 = q.enqueue(2);
    let _r3 = q.enqueue(3);

    if let Err(error) = q.enqueue(4){
        println!("队列错误:{}",error);
    }

    if let Some(data) = q.dequeue(){
        println!("出队：{}",data);
    }
    else{
        println!("空队");
    }

    println!("队列大小:{}, 是否时空队列:{}",q.size(),q.is_empty());
    println!("队列:{:?}",q);
}

#[allow(dead_code)]
fn hot_run(){
    let names = vec!["Goodman","David","Susan","Jane","Kew","Brad"];

    let rem = queue::hot_potato(names,8);
    println!("最有一个人：{rem}");
}

#[allow(dead_code)]
fn dequeue_run(){
    let mut d = Deque::new(4);

    let _r1 = d.add_front(1);
    let _r2 = d.add_front(2);
    let _r3 = d.add_rear(3);
    let _r4 = d.add_rear(4);

    if let Err(error) = d.add_front(5){
        println!("添加错误：{}",error);
    }
    if let Some(data) = d.remove_rear(){
        println!("data:{}",data);
    }
    else{
        println!("空队列");
    }

    println!("大小:{}, 是否空双端队列:{}",d.size(),d.is_empty());
    println!("双端队列:{:?}",d);
}
#[allow(dead_code)]
fn pal_check_run(){
    let pal = "rustsur";

    let is_pal = pal_checker(pal);
    println!("{} 是否时回文？{}",pal,is_pal);
}