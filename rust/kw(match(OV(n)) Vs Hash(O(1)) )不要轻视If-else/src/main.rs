#![feature(plugin)]
#![plugin(phf_macros)]
#![feature(test)]
extern crate test;
#[macro_use]
extern crate lazy_static;
extern crate phf;

// 数量少的话, match/find最遭的情况依然比map好
//
// Hash的 O(1), 匹配时间仅取决于当前字符串/bytes的长度(Hash一次).
//
// match/if-else 的 O(n), 取决于匹配次数(直到匹配成功或全部失败), 另外还有目标字符串/bytes长度.
// 
// 参见b模块, if-else的最糟糕情况 O(n), 大概在 26个元素的集合与 phf 的平均值耗时相当.
//
// 总的来说, 二三十之内不用担心 match 低效率, 除非目标字符串/bytes非常长.
//
// 另外自己写个字典树试试?
mod b;

#[derive(Clone)]
pub enum Kw {
    Loop,
    Continue,
    Break,
    Fn,
    Extern,
}

fn match_parse_kw(kw: &str) -> Option<Kw> {
    match kw {
        "loop" => Some(Kw::Loop),
        "continue" => Some(Kw::Continue),
        "break" => Some(Kw::Break),
        "fn" => Some(Kw::Fn),
        "extern" => Some(Kw::Extern),
        _ => None,
    }
}

#[bench]
fn match_(b: &mut test::Bencher) {
    b.iter(|| {
        vec!["loop", "continue", "break", "fn", "extern"]
            .into_iter()
            .for_each(|e| assert!(match_parse_kw(e).is_some()))
    })
}

#[bench]
fn match_none(b: &mut test::Bencher) {
    b.iter(|| {
        vec!["loop ", "continue,", "breakx", "fnx", "externxx"]
            .into_iter()
            .for_each(|e| assert!(match_parse_kw(e).is_none()))
    })
}

fn vec_parse_kw(kw: &str)->Option<&'static Kw> {
     let svec= &[("loop" ,Kw::Loop),("continue", Kw::Continue), ("break" , Kw::Break), ("fn" , Kw::Fn), ("extern" , Kw::Extern) ][..];
     svec.iter().find(|e|e.0 ==kw).map(|ref b|&b.1)
}

#[bench]
fn vec(b: &mut test::Bencher) {
    b.iter(|| {
        vec!["loop", "continue", "break", "fn", "extern"]
            .into_iter()
            .for_each(|e| assert!(vec_parse_kw(e).is_some()))
    })
}

#[bench]
fn vec_none(b: &mut test::Bencher) {
    b.iter(|| {
        vec!["loop ", "continue,", "breakx", "fnx", "externxx"]
            .into_iter()
            .for_each(|e| assert!(vec_parse_kw(e).is_none()))
    })
}

static KWS: phf::Map<&'static str, &'static Kw> = phf_map! {
    "loop" => &Kw::Loop,
    "continue" => &Kw::Continue,
    "break" => &Kw::Break,
    "fn" => &Kw::Fn,
    "extern" => &Kw::Extern,
};

pub fn phf_parse_kw(kw: &str) -> Option<&'static Kw> {
    KWS.get(kw).map(|x|*x )
}
#[bench]
fn phf_(b: &mut test::Bencher) {
    b.iter(|| {
        vec!["loop", "continue", "break", "fn", "extern"]
            .into_iter()
            .for_each(|e| assert!(phf_parse_kw(e).is_some()))
    })
}

#[bench]
fn phf_none(b: &mut test::Bencher) {
    b.iter(|| {
        vec!["loop ", "continue,", "breakx", "fnx", "externxx"]
            .into_iter()
            .for_each(|e| assert!(phf_parse_kw(e).is_none()))
    })
}
fn hashmap_parse_kw(kw: &str)-> Option<&'static Kw> {
lazy_static!{
        static ref MAP: std::collections::HashMap<&'static str, Kw>= vec![("loop" ,Kw::Loop),("continue", Kw::Continue), ("break" , Kw::Break), ("fn" , Kw::Fn), ("extern" , Kw::Extern) ].into_iter().collect();
    }
    MAP.get(kw)
}
#[bench]
fn hashmap(b: &mut test::Bencher) {
    b.iter(|| {
        vec!["loop", "continue", "break", "fn", "extern"]
            .into_iter()
            .for_each(|e| assert!(hashmap_parse_kw(e).is_some()))
    })
}
#[bench]
fn hashmap_none(b: &mut test::Bencher) {
    b.iter(|| {
        vec!["loop ", "continue,", "breakx", "fnx", "externxx"]
            .into_iter()
            .for_each(|e| assert!(hashmap_parse_kw(e).is_none()))
    })
}

fn main() {
    println!("Hello, world!");
    for idx in 0..100 {
        print!("V{:02},",idx);
    }
    println!( );
    for idx in 0..100 {
        print!("(\"prifix_{}_suffix\",Kw::V{:02}),",idx, idx);
    }
    println!( );
    for idx in 0..100 {
        print!("\"prifix_{}_suffix\" => &Kw::V{:02},",idx, idx);
    }
}
