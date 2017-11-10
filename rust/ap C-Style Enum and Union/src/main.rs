fn main() {
    let name = Name::Two;
    println!("{:?}", name);
    println!("as usize: {:?}", name as usize);
    let num: usize = name.into();
    println!("{:?}", num);

    union();
}

#[derive(Debug, Clone, Copy)]
pub enum Name {
    One = 1,
    Two = 2,
    Three = 3,
}
impl Name {
    pub fn as_usize(&self) -> usize {
        *self as usize
    }
    pub fn is_two(&self) -> bool {
        use Name::*;
        match *self {
            Two => true,
            _ => false,
        }
    }
}

impl Into<usize> for Name {
    fn into(self) -> usize {
        self as usize
    }
}

/// [Rust-1.19](https://blog.rust-lang.org/2017/07/20/Rust-1.19.html)
pub fn union() {
    let mut u = MyUnion { f1: 1 };
    u.f1 = 5;
    u.fun();    

    let mut u = MyUnion { f1: 2 };
    u.f1 = 10;
    u.fun();   

    let mut u = MyUnion { f2: 1.1f32 };
    u.f2 = 5.38;
    u.fun();

    let value = unsafe { u.f1 };
    println!("unsafe f1: {}", value);
}

#[derive(Clone, Copy)]
pub union MyUnion {
    f1: u32,
    f2: f32,
}

impl MyUnion {
    fn fun(self) {
        unsafe {
            match self {
                MyUnion { f1: 10 } => {
                    println!("f1==10");
                }
                MyUnion { f2 } => {
                    println!("f2: {}", f2);
                }
            }
        }
    }
}
