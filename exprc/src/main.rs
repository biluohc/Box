pub mod c;
use std::env;

fn main() {
    let mut args = env::args().skip(1).collect::<Vec<String>>();
    if args.len() == 0 {
        println!(
            "Exprc need args !
        Exprc support +-*/ and assgin(=): a=123; a*(2-1)/3 
        Exprc will go to repl's mode after deal with other args if args contains '-i'.
        Exprc will print all variable if arg/repl's line is `p'.
        Exprc will exit if repl's line is `q'.
    Notice: 
        ' ' is equal to ';'
        'e' stored 2.71828182845
        'pi' stored 3.1415926535
        '_' stored the value of the last expr if it is not a assign expr(Like Py)."
        );
        return;
    }
    let mut repl = false;
    if let Some(idx) = args.as_slice().iter().position(|arg| arg == "-i") {
        repl = true;
        args.remove(idx);
    }
    if let Err((idx, e)) = fun(&args[..], repl) {
        eprintln!("{:?} -> {}", args[idx], e);
    }
}

pub fn fun(args: &[String], repl_: bool) -> Result<(), (usize, String)> {
    let mut c = c::Caculater::new();
    for (idx, arg) in args.iter().enumerate() {
        c.msg_update(&arg);
        if arg.trim() == "p" {
            println!("{:?}", c.name_number_map);
        }
        while !c.msg.peek().is_none() {
            c.get_token().map_err(|e| (idx, e))?;

            if c.current_tok == c::Token::End {
                break;
            }
            if c.current_tok == c::Token::Print {
                continue;
            }

            let rest = c.expr(false).map_err(|e| (idx, e))?;

            println!("{} = {}", c.recent_unassign, rest);
            c.name_number_map.insert(c.recent_unassign.to_owned(), rest);
            c.recent_unassign = "_".to_owned();
        }
    }
    if repl_ {
        repl(&mut c);
    }
    Ok(())
}
use std::io;
use std::process::exit;

pub fn repl(mut c: &mut c::Caculater) {
    let mut str = String::default();
    loop {
        str.clear();
        match io::stdin().read_line(&mut str) {
            Ok(_) => {
                c.msg_update(&str);
                if str.trim() == "p" {
                    println!("{:?}", c.name_number_map);
                    continue;
                }
                if str.trim() == "q" {
                    println!("{:?}", c.name_number_map);
                    break;
                }
                if let Err(e) = repl_inner(&mut c) {
                    eprintln!("{:?} -> {}", str, e);
                }
            }
            Err(e) => {
                eprintln!("repl_read_line: {}", e);
                exit(1)
            }
        }
    }
}

pub fn repl_inner(c: &mut c::Caculater) -> Result<(), String> {

    while !c.msg.peek().is_none() {
        c.get_token()?;

        if c.current_tok == c::Token::End {
            break;
        }
        if c.current_tok == c::Token::Print {
            continue;
        }

        let rest = c.expr(false)?;
        println!("{} = {}", c.recent_unassign, rest);
        c.name_number_map.insert(c.recent_unassign.to_owned(), rest);
        c.recent_unassign = "_".to_owned();
    }
    Ok(())
}
