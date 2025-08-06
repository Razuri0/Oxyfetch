use clap::Parser;
use sysinfo::System;

mod module;
use module::{Module, EXPRESSIONS};

mod config_reader;
use config_reader::config_reader;

#[derive(Parser)]
struct Args {
    #[arg(long, default_value = "false")]
    config: bool,
}

fn main() {
    let args = Args::parse();

    let config = config_reader();


    let mut sys = System::new_all();
    sys.refresh_all();

    let mut models = Vec::new();
    models.push(EXPRESSIONS::NAME);
    models.push(EXPRESSIONS::KERNEL);
    models.push(EXPRESSIONS::RELEASE_TYPE);
    let mut output = Module::new(models);
    for bullet in output.format_bullets() {
        println!("{}", bullet);
    }

}
