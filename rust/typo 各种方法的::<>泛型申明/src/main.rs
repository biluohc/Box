fn main() {
    let s = std::env::args().collect::<Vec<String>>();
    println!("{:?}", s);

    let s = Vec::<&str>::new();
    println!("{:?}", s);

    let s = std::collections::BinaryHeap::<u64>::default();
    println!("{:?}", s);    

    let s = std::collections::LinkedList::<char>::new();
    println!("{:?}", s);

    let _ = std::collections::VecDeque::<Box<FnMut()->()>>::new();
    
    let s = std::collections::HashSet::<std::path::PathBuf>::new();
    println!("{:?}", s);
    let s = std::collections::BTreeSet::<std::path::PathBuf>::new();
    println!("{:?}", s);

    let s = std::collections::HashMap::<i32, i32>::new();
    println!("{:?}", s);  
    let s = std::collections::BTreeMap::<i32, i32>::new();
    println!("{:?}", s);

    let s =  std::borrow::Cow::Borrowed("std::borrow::Cow<'_, str>");
    println!("{:?}", s);
}
