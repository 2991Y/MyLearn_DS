pub mod queue{


    #[derive(Debug)]
    pub struct Queue<T>{
        cap:usize,
        data:Vec<T>,
    }

    impl<T> Queue<T>{
       pub fn new(size:usize)->Self{
            Queue{
                data:Vec::with_capacity(size),
                cap:size,
            }
        }

        pub fn enqueue(&mut self,val:T)->Result<(),String>{
            if self.size() == self.cap {
                return Err("队列已满！".into());
            }
            self.data.insert(0,val);
            Ok(())
        }

        pub fn dequeue(&mut self)->Option<T>{
            if self.size() > 0{
                self.data.pop()
            }else{
                None
            }
        }

        pub fn is_empty(&self)->bool{
            0 == self.size()
        }

        pub fn size(&self)->usize{
            self.data.len()
        }
    }

    /// 约瑟夫问题：烫手山芋
    pub fn hot_potato(names:Vec<&str>, num:usize)->&str{
        let mut q = Queue::new(names.len());

        for name in names{
            let _= q.enqueue(name);
        }

        while q.size()>1{
            for _i in 0..num{
                let name = q.dequeue().unwrap();
                let _rm = q.enqueue(name);
            }

            let _rm = q.dequeue();
        }

        q.dequeue().unwrap()
    }

    #[derive(Debug)]
    pub struct Deque<T>{
        cap:usize,
        data:Vec<T>,
    }

    impl<T> Deque<T>{
        pub fn new(cap:usize)->Self{
            Deque{
                cap:cap,
                data:Vec::with_capacity(cap),
            }
        }

        pub fn add_front(&mut self,val:T)->Result<(),String>{
            if self.size() == self.cap{
                return  Err("双端队列已满!".into());
            }

            self.data.push(val);
            Ok(())
        }

        pub fn add_rear(&mut self,val:T)->Result<(),String>{
            if self.size() == self.cap{
                return  Err("双端队列已满!".into());
            }

            self.data.insert(0,val);
            Ok(())

        }

        pub fn remove_front(&mut self)->Option<T>{
            if self.size() > 0{
                self.data.pop()
            }
            else{
                None
            }
        }

        pub fn remove_rear(&mut self)->Option<T>{
            if self.size()>0{
                Some(self.data.remove(0))
            }
            else{
                None
            }
        }

        pub fn is_empty(&self)->bool{
            0 == self.size()
        }

        pub fn size(&self)->usize{
            self.data.len()
        }
    }

    // 回文检测
    pub fn pal_checker(pal:&str)->bool{
        let mut d = Deque::new(pal.len());

        for c in pal.chars(){
           let _r = d.add_rear(c);
        }

        let mut is_ok = true;
        while d.size() > 1 && is_ok {
            let head = d.remove_front();
            let rear = d.remove_rear();

            if head != rear {
                is_ok = false;
            }
        }

        is_ok
    }

    type Link<T> = Option<Box<Node<T>>>;
    pub struct List<T>{
        size:usize,
        head:Link<T>,
    }
    
    struct Node<T>{
        elem:T,
        next:Link<T>,
    }

    impl<T> List<T> {
        pub fn new()->Self{
            List{size:0,head:None}
        }
        
        pub fn is_empty(&self)->bool{
            self.size == 0
        }

        pub fn size(&self)->usize{
            self.size
        }

        /// 头插
        pub fn push(&mut self,val:T){
            let node = Box::new(Node{
                elem:val,
                next:self.head.take()
            });

            self.head = Some(node);
            self.size +=1;
        }

        pub fn pop(&mut self)->Option<T>{
            self.head.take().map(|node|{
                self.head = node.next;
                self.size -=1;
                node.elem
            })
        }

        pub fn peek(&self)->Option<&T>{
            self.head.as_ref().map(|n|&n.elem)
        }

        pub fn into_iter(self)->IntoIter<T>{
            IntoIter(self)
        }

        pub fn iter(&self)->Iter<T>{
            Iter{next:self.head.as_deref()}
        }

        pub fn iter_mut(&mut self)->IterMut<T>{
            IterMut{next:self.head.as_deref_mut()}
        }

    }

    pub struct IntoIter<T>(List<T>);

    impl<T> Iterator for IntoIter<T>{
        type Item = T;
        fn next(&mut self)->Option<Self::Item>{
            self.0.pop()
        }
    }

    pub struct Iter<'a,T:'a> {next:Option<&'a Node<T>>}

    impl<'a,T> Iterator for Iter<'a,T>{
        type Item = &'a T;
        fn next(&mut self) -> Option<Self::Item> {
            self.next.map(|node|{
                self.next = node.next.as_deref();
                &node.elem
            })
        }
    }

    pub struct IterMut<'a,T:'a>{next:Option<&'a mut Node<T>>}

    impl<'a,T> Iterator for IterMut<'a,T>{
        type Item = &'a mut T;
        fn next(&mut self) -> Option<Self::Item> {
            self.next.take().map(|node|{
               self.next = node.next.as_deref_mut(); 
               &mut node.elem
            })
        }
    }

    impl<T> Drop for List<T>{
        fn drop(&mut self) {
            let mut link = self.head.take();
            while let Some(mut node) = link {
                link = node.next.take();
            }
        }
    }
}