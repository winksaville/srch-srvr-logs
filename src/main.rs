use clap::Parser;
use std::process::Command;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(value_parser, help = "ssh login, account@machine")]
    acct_name: String,

    #[clap(
        value_parser,
        help = "rust regex string, an empty string, \"\", matches everything"
    )]
    regex: String,

    #[clap(
        short = 't',
        long,
        value_parser,
        default_value = "-1d",
        help = "journalctl --since parameter but use '=', Example: --since=-1h"
    )]
    since: String,

    #[clap(short, long, value_parser, multiple = true, default_values = &["eth1", "beacon-chain", "validator"] )]
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

        // Create the command
        let mut cmd = Command::new("ssh");
        let regex = if args.regex.is_empty() {
            "\"\""
        } else {
            &args.regex
        };
        cmd.args([
            &args.acct_name,
            "journalctl",
            "-u",
            &srvc_str,
            "--since",
            &args.since,
            "|",
            "rg",
            "--color",
            "always",
            "-e",
            regex,
        ]);
        //println!("cmd: {:#?}",cmd);

        // Execute the command
        let result = cmd.output()?;

        let code = result.status.code();
        const ONE: i32 = 1;
        match code {
            Some(0) => {
                let stdout = String::from_utf8_lossy(&result.stdout);
                println!("{stdout}");
            }
            Some(ONE) => {
                let stderr = String::from_utf8_lossy(&result.stderr);
                if stderr.len() == 0 {
                    println!(" <no matches for regex: \"{}>\"", args.regex);
                } else {
                    println!("Exit code: {ONE}, {stderr}");
                }
            }
            Some(v) => {
                // All other errors
                let stderr = String::from_utf8_lossy(&result.stderr);
                return Err(format!("Exit code: {v}, {stderr}").into());
            }

            None => {
                return Err("No exit code, done".into());
            }
        }
    }

    Ok(())
}
