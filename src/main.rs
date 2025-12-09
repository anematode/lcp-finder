mod finder;

use crate::finder::find_lcps;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to executable file
    #[arg(short, long)]
    file: String,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let elf_bytes = std::fs::read(&args.file)?;
    let lcp_instructions = find_lcps(&elf_bytes)?;

    println!(
        "Found {} instructions that would cause a length-changing prefix stall on Intel CPUs.",
        lcp_instructions.len()
    );
    println!("{}", "=".repeat(10));
    for lcp in lcp_instructions {
        let bytes = lcp
            .instruction_bytes
            .iter()
            .map(|b| format!("{:02x}", *b))
            .collect::<Vec<String>>()
            .join(" ");
        println!("{:>8x}: {:<45} {}", lcp.code_addr, bytes, lcp.decoded);
    }

    Ok(())
}
