use clap::Parser;
use sysinfo::{
    Components, Disks, Networks, System,
};

#[derive(Parser)]
struct Args {
    #[arg(long, default_value = "false")]
    noart: String,

    #[arg(long, default_value = "false")]
    notext: String,
}

fn main() {
    let args = Args::parse();


    let mut sys = System::new_all();
    sys.refresh_all();

    println!("Kernel Name: {}", System::name().unwrap_or("Unknown".to_string()));
    println!("Kernel: {}", System::kernel_version().unwrap_or("Unknown".to_string()));

}
