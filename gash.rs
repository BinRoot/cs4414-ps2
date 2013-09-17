use std::{io, run, libc, c_str, str};

#[fixed_stack_segment]
fn main() {
    static CMD_PROMPT: &'static str = "gash";
    static CMD_SEP: &'static str = "≻";
    static CMD_ERR: &'static str = "\\033[38;5;1m";
    static CMD_ERR_END: &'static str = "\\033[39m";
    static CMD_CMD: &'static str = "\\033[38;5;6m";
    static CMD_CMD_END: &'static str = "\\033[39m";
    static CMD_NUM: &'static str = "\\033[38;5;3m";
    static CMD_NUM_END: &'static str = "\\033[39m";
    let mut CMD_PATH: ~str = ~""; 
    let mut HISTORY: ~[~str] = ~[];

    loop {
        let prompt: ~str =
        if CMD_PATH == ~"" {
            fmt!("%s%s%s%s ", CMD_CMD, CMD_PROMPT, CMD_CMD_END, CMD_SEP)
        } else {
            fmt!("%s%s%s %s%s ", CMD_CMD, CMD_PROMPT, CMD_CMD_END, CMD_PATH, CMD_SEP)
        };
        run::process_status("echo", [~"-e", prompt+"\\c"]);

        let line = io::stdin().read_line();
        debug!(fmt!("line: %?", line));
        HISTORY.push(line.clone());
        let mut argv: ~[~str] = line.split_iter(' ').filter(|&x| x != "")
            .map(|x| x.to_owned()).collect();
//        println(fmt!("argv %?", argv));

        if argv.len() > 0 {
            let program = argv.remove(0);

            let mut i = 0;
            let mut args = ~"";
            while i<argv.len() {
                args = args + argv[i];
                i += 1;
            }

            match program {
                ~"exit"     => { return; }
                ~"cd"       => { 
                    let orig_dir = str::from_utf8(run::process_output("pwd", []).output);
                    let mycstr: c_str::CString = args.to_c_str();
                    unsafe { libc::funcs::posix88::unistd::chdir( mycstr.unwrap() ); }
                    let new_dir = str::from_utf8(run::process_output("pwd", []).output);

                    if new_dir==orig_dir {
                        let err_msg: ~str = 
                            fmt!("%s[ ERR ]%s cd: %s: No such file or directory", CMD_ERR, CMD_ERR_END, args);
                        run::process_status("echo", [~"-e", err_msg]);
                    } else {
                        CMD_PATH = new_dir.slice_to(new_dir.len()-1).to_owned();
                    }
                }
                ~"history"  => { 
                    let mut i: uint = 0;
                    while i<HISTORY.len() {
                        let hist_msg: ~str = fmt!("%s%s%s %s", CMD_NUM, (i+1).to_str(), CMD_NUM_END, HISTORY[i]);
                        run::process_status("echo", [~"-e", hist_msg]);

                        i += 1;
                    }
                }
                _           => {
                    if argv.len() > 0 && argv[argv.len()-1] == ~"&" {
                        unsafe { 
                            let pid = libc::funcs::posix88::unistd::fork();
                            match pid {
                                -1 => {
                                    let err_msg: ~str = 
                                        fmt!("%s[ ERR ]%s fork failed", CMD_ERR, CMD_ERR_END);
                                    run::process_status("echo", [~"-e", err_msg]);
                                }
                                0 => {
                                    run::process_status(program, argv);
                                }
                                _ => ()
                            }
                        }
                    } else {
                        run::process_status(program, argv);
                    }
                }
            }
        }
    }
}
