// The cat utility reads files sequentially, writing them to the standard output.
// The file operands are processed in command-line order.
// If file is a single dash (‘-’) or absent, cat reads from the standard input.
// If file is a UNIX domain socket, cat connects to it and then reads it until EOF.
// This complements the UNIX domain binding capability available in inetd(8).
//
// The options are as follows:
// -b      Number the non-blank output lines, starting at 1.
//
// -e      Display non-printing characters (see the -v option), and display a dollar sign (‘$’) at the end of each line.
//
// -l      Set an exclusive advisory lock on the standard output file descriptor.
//         This lock is set using fcntl(2) with the F_SETLKW command.
//         If the output file is already locked, cat will block until the lock is acquired.
//
// -n      Number the output lines, starting at 1.
//
// -s      Squeeze multiple adjacent empty lines, causing the output to be single spaced.
//
// -t      Display non-printing characters (see the -v option), and display tab characters as ‘^I’.
//
// -u      Disable output buffering.
//
// -v      Display non-printing characters so they are visible.
//         Control characters print as ‘^X’ for control-X; the delete character (octal 0177) prints as ‘^?’.
//         Non-ASCII characters (with the high bit set) are printed as ‘M-’ (for meta) followed by the character for the low 7 bits.

use std::env;

struct Option {
    // -b
    show_non_blank_line_numbers: bool,

    // -e
    print_dollar_at_end: bool,

    // -l
    lock_output: bool,

    // -n
    // NOTE: -b に負けそう
    show_line_numbers: bool,

    // -s
    squeeze_empty_lines: bool,

    // -t
    print_tab_as_control_i: bool,

    // -u
    disable_output_buffering: bool,

    // -v
    show_non_printing_characters: bool,
}

fn main() {
    let argv: Vec<String> = env::args().collect();
    let argc = argv.len();
}
