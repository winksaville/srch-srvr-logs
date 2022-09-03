use clap::Parser;
use std::process::Command;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[rustfmt::skip]
struct Args {
    #[clap(value_parser, help = "rust regex string, an empty string, \"\", matches everything")]
    regex: String,

    #[clap(short, long, value_parser, help = "ssh dest, where dest is user@machine")]
    ssh_dest: Option<String>,

    #[clap(long, value_parser, default_value = "-1d",
            help = "journalctl --since parameter but use '=', Example: --since=-1h")]
    since: String,

    #[clap(long, value_parser, multiple = true, default_values = &["eth1", "beacon-chain", "validator"] )]
    services: Vec<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    //println!("acct_name: {:?}", args.acct_name);
    //println!("regex: {:?}", args.regex);
    //println!("since: {:?}", args.since);
    //println!("services: {:?}", args.services);

    for srvc in args.services {
        let srvc_str = srvc + ".service";
        println!("Process {srvc_str}:");

        // Get the regex to match
        let regex = if args.regex.is_empty() {
            // It's empty so specfically apss "" so there is something
            "\"\""
        } else {
            //
            &args.regex
        };

        // Check of we have a ssh_dest
        let rg_result = if let Some(ref dest) = args.ssh_dest {
            // Yes, so ssh to ssh_dest and use journalctl
            let mut cmd = Command::new("ssh");
            cmd.arg(dest);
            cmd.args(["journalctl", "-u", &srvc_str, "--since", &args.since]);
            cmd.arg("|");
            cmd.args(["rg", "--color", "always", "-e", regex]);
            cmd.output()?
        } else {
            // Maybe use https://github.com/hniksic/rust-subprocess
            // and more specifically probably pipelines:
            //   https://github.com/hniksic/rust-subprocess#pipelines
            todo!("Local execution isn't supported yet :)");
        };

        let code = rg_result.status.code();
        const EXIT_CODE_ZERO: i32 = 0;
        const EXIT_CODE_ONE: i32 = 1;
        match code {
            Some(EXIT_CODE_ZERO) => {
                let stdout = String::from_utf8_lossy(&rg_result.stdout);
                println!("{stdout}");
            }
            Some(EXIT_CODE_ONE) => {
                let stderr = String::from_utf8_lossy(&rg_result.stderr);
                if stderr.len() == 0 {
                    println!(" <no matches for regex: \"{}>\"", args.regex);
                } else {
                    println!("Exit code: {EXIT_CODE_ONE}, {stderr}");
                }
            }
            Some(v) => {
                // All other errors
                let stderr = String::from_utf8_lossy(&rg_result.stderr);
                return Err(format!("Exit code: {v}, {stderr}").into());
            }
            None => {
                return Err("No exit code, done".into());
            }
        }
    }

    Ok(())
}
