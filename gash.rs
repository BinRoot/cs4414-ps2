extern mod extra;
use std::{run, libc, c_str, str, io};
static CMD_ERR: &'static str = "\\033[38;5;1m";
static CMD_ERR_END: &'static str = "\\033[39m";
static CMD_CMD: &'static str = "\\033[38;5;6m";
static CMD_CMD_END: &'static str = "\\033[39m";
static CMD_NUM: &'static str = "\\033[38;5;3m";
static CMD_NUM_END: &'static str = "\\033[39m";
static CMD_PROMPT: &'static str = "gash";
static CMD_SEP: &'static str = "≻";


#[fixed_stack_segment]
fn main() {
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
        //println(fmt!("argv %?", argv));

        let mut current_command: ~[~str] = ~[];
        let mut carry_in = ~[];

        while argv.len() > 0 {
            let word = argv.remove(0);

            match word {
                ~"<" => {
                    if argv.len() == 0 {
                        println("No path/file found!");
                        break;
                    }

                    let path = argv.remove(0);
                    let input = load_file(path);
                    let process_out = run_command(HISTORY.clone(), current_command[0].clone(), current_command.slice(1, current_command.len()).to_owned(),input);
                    carry_in = process_out;
                    current_command = ~[];
                }
                ~">" => {
                    if argv.len() == 0 {
                        println("No target path/file found!");
                        break;
                    }

                    let path = argv.remove(0);

                    let process_out = run_command(HISTORY.clone(), current_command[0].clone(), current_command.slice(1, current_command.len()).to_owned(), carry_in.clone());

                    write_file(path, process_out);
                    carry_in = ~[];
                    current_command = ~[];
                }
                ~"|" => {
                    if current_command.len() != 0 {

                        let process_out = run_command(HISTORY.clone(), current_command[0].clone(), current_command.slice(1, current_command.len()).to_owned(), carry_in.clone());

                        carry_in = process_out;
                        current_command = ~[];
                    }
                }
                _ => {
                    current_command.push(word.clone());
                }
            }
        }

        if current_command.len() > 0 {
            let args: ~[~str] = current_command.slice(1, current_command.len()).to_owned();
            let process_out = run_command(HISTORY.clone(), current_command[0].clone(), args.clone(), carry_in.clone());
            run::process_status("echo", [~"-e", str::from_utf8(process_out) + "\\c"]);
//            print(str::from_utf8(process_out));

            match current_command[0] {
                ~"cd" => { 
                    let orig_dir = str::from_utf8(run::process_output("pwd", []).output);
                    let mycstr: c_str::CString = if args.len() > 0 {args[0].to_c_str()} else {orig_dir.to_c_str()};
                    unsafe { libc::funcs::posix88::unistd::chdir( mycstr.unwrap() ); }
                    let new_dir = str::from_utf8(run::process_output("pwd", []).output);
                    if new_dir==orig_dir {
                        let err_msg: ~str =
                            fmt!("%s[ ERR ]%s cd: %s: No such file or directory", CMD_ERR, CMD_ERR_END, args.to_str());
                        run::process_status("echo", [~"-e", err_msg]);
                    } else {
                        CMD_PATH = new_dir.slice_to(new_dir.len()-1).to_owned();
                    }
                },
                _     => ()
            }
        } else {
            run::process_status("echo", [~"-e", str::from_utf8(carry_in.clone()) + "\\c"]);
//            print(str::from_utf8(carry_in.clone()));
        }

    }
}

#[fixed_stack_segment]
fn run_command(mut HISTORY: ~[~str], prog: ~str, args: ~[~str], input: ~[u8]) -> ~[u8] {
    let mut ret_val = ~[];
    match prog {
        ~"exit"     => {
            unsafe {
                libc::exit(0);
            }
        }
        ~"history"  => {
            let mut i: uint = 0;
            let mut output = ~"";
            while i<HISTORY.len() {
                let hist_msg: ~str = fmt!("%s%s%s %s", CMD_NUM, (i+1).to_str(), CMD_NUM_END, HISTORY[i]);
                output = output + hist_msg + "\r\n";
                i += 1;
            }
            ret_val = output.into_bytes();
        }
        _           => {
            if args.len() > 0 && args[args.len()-1] == ~"&" {
                unsafe {
                    let pid = libc::funcs::posix88::unistd::fork();
                    match pid {
                        -1 => {
                            let err_msg: ~str =
                                fmt!("%s[ ERR ]%s fork failed", CMD_ERR, CMD_ERR_END);
                            run::process_status("echo", [~"-e", err_msg]);
                        }
                        0 => {
                            run::process_status(prog, args);
                        }
                        _ => ()
                    }
                }
            } else {
                let mut p = run::Process::new(prog, args, run::ProcessOptions::new());
                let std_in = p.input();
                std_in.write(input);
                let process_out = p.finish_with_output();
                ret_val = process_out.output
            }
        }
    }
    ret_val
}

fn load_file(pathname : ~str) ->  ~[u8] {
    let read_result = io::read_whole_file(&PosixPath(pathname));
    match read_result {
        Err(msg) => {
            fail!(fmt!("Error: %?", msg));
        },
        Ok(contents) => {
            return contents;
        }
    }
}

fn write_file(pathname : ~str, contents: ~[u8]) {
    let writer = io::file_writer(&Path(pathname), [io::Create]).unwrap();
    writer.write(contents);
}