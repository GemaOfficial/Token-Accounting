use std::env;
use gemaaccountscli::utils;
use gemaaccountscli::args::setup;
use gemaaccountscli::args::dispatch;
use gemaaccountscli::accounts::record;

fn main() {
    let mut rd: record::Record = match record::Record::new() {
	Ok(r) => r,
	Err(e) => { eprintln!("{}", e); std::process::exit(1); },
    };
    
    let mut args = setup::setup_args();
    let index = dispatch::check_args(&args);
    if index == -1 {
    	eprintln!("Error: no argument recognized.");
    	utils::helpers::help();
    	std::process::exit(1);
    }

    let mut env_args = env::args();
    env_args.next(); // skip binary.
    env_args.next(); // skip option.
    dispatch::fill_command(&mut env_args, &mut args.blocks[index as usize]);
    dispatch::execute_blocks(&mut rd, &mut args.blocks[index as usize]);

    match rd.save_record() {
	Ok(()) => (),
	Err(e) => { eprintln!("{}", e); std::process::exit(1); },
    };
}
