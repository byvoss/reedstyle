use anyhow::Result;
use clap::Parser;
use colored::*;

#[derive(Parser, Debug)]
#[command(name = "reedstyle")]
#[command(author = "Vivian Burkhard Voss <vivian.voss@byvoss.tech>")]
#[command(version = "0.1.0")]
#[command(about = "Semantic HTML styling system", long_about = None)]
struct Args {
    /// Watch for file changes
    #[arg(short, long)]
    watch: bool,

    /// Output directory
    #[arg(short, long, default_value = "dist")]
    output: String,

    /// Minify output
    #[arg(short, long)]
    minify: bool,

    /// Generate source maps
    #[arg(short, long)]
    sourcemaps: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    println!("{}", "ReedSTYLE Build System".bright_cyan().bold());
    println!("{}", "═══════════════════════".bright_cyan());

    let reedstyle = reedstyle::ReedStyle::new()?;
    reedstyle.build()?;

    println!("{} Build complete!", "✓".green().bold());

    if args.watch {
        println!("Watching for changes...");
        // TODO: Implement file watching
    }

    Ok(())
}