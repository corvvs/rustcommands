use std::env;

// /bin/echo
// NOTE: シェルの echo ではない
fn main() {
    let argv: Vec<String> = env::args().collect();
    let argc = argv.len();

    let mut i_from = 1;
    let mut with_newline = true;
    if argc > 1 && argv[1] == "-n" {
        i_from = 2;
        with_newline = false;
    }

    for i in i_from..argc {
        if i > i_from {
            print!(" ");
        }
        print!("{}", argv[i]);
    }
    if with_newline {
        println!();
    }
}
