/*!
http://blog.adamperry.me/rust/2016/07/24/profiling-rust-perf-flamegraph/#perf

`git clone https://github.com/brendangregg/FlameGraph`

and `link -s` stackcollapse-perf.pl and flamegraph.pl to PATH

```toml
[profile.release]
debug = true
```

`cargo build --release`

```fish
# perf 获取样本, -g 生成图像信息
sudo perf record -g /home/mxo/.cache/mozilla/cargo/release/ap; 
# 产生的data属于root
and  sudo chmod 777 *data; 
 # 调用那些生成火焰图
and perf script  |stackcollapse-perf.pl | flamegraph.pl > flame.svg
```
## 每次取样数可能不同, 不够就没有什么信息, 另外可以配合 #[no_mangle] 使用
*/
#![feature(test)]
extern crate test;
#[bench]
fn name(b: &mut test::Bencher) {
    b.iter(||(0..1).into_iter().map(|x|fib(x)).count())
}

fn main() {
    fun()
}

#[no_mangle]
pub fn fun() {
    let f = fib(38);
    println!("{}",f);
}

#[no_mangle]
pub fn fib(msg: i32) -> i32 {
    match msg {
        0...2 => 1,
        x => fib(x - 1) + fib(x - 2),
    }
}