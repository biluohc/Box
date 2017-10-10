fn main() {
    // https://doc.rust-lang.org/std/slice/trait.SliceConcatExt.html
    let s: String = vec!["下", "xd", "77"].join("-");
    println!("{:?}", s);

    fun()
}

fn fun() {
    let bs = vec![1, 23, 4, 5, 5, 8];

    println!("{}", bs[..].joins());
    println!("{}", bs[..].joinss("-"));
    println!("{}", bs[..].joinss("*"));
    println!("{}", bs.joinss("&"));
    println!("{}", bs.as_slice().joinss("#"));
}


pub trait SliceJoin {
    fn joins(&self) -> String; // 官方为[&T]实现了个同名的, join命名冲突
    fn joinss(&self, sep: &str) -> String;
}

impl<T: std::fmt::Display> SliceJoin for [T] {
    fn joins(&self) -> String {
        self.iter().fold(String::with_capacity(0), |acc, ref x| {
            if acc.is_empty() {
                x.to_string()
            } else {
                acc + &format!(",{}", x)
            }
        })
    }
    fn joinss(&self, s: &str) -> String {
        self.iter().fold(String::with_capacity(0), |acc, ref x| {
            if acc.is_empty() {
                x.to_string()
            } else {
                acc + &format!("{}{}", s, x)
            }
        })
    }
}
