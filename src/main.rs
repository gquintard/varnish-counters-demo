use anyhow::Result;
use clap::Parser;
use tabled::{Table, Tabled};
use varnish::vsc::VSCBuilder;

/// demo tool, showing varnish counter in a nicely formatted table
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Varnish workdir (must be the same as the varnishd -n argument)
    #[arg(short, value_parser)]
    name: Option<std::path::PathBuf>,

    /// counters with a name matching the globbing pattern(s) will be shown (can be repeated)
    #[arg(short, long, value_parser)]
    glob: Vec<String>,
}

#[derive(Tabled)]
struct PrettyStat<'a> {
    section: &'a str,
    name: &'a str,
    value: u64,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let mut builder = VSCBuilder::new();

    // if there's a workdir, use it
    if let Some(s) = args.name {
        builder = builder.work_dir(&s)?;
    }
    for g in args.glob {
        builder = builder.include(&g)?;
    }
    let mut vsc = builder.build()?;

    let mut pretty_stats = Vec::new();

    vsc.update();
    for (_, stat) in vsc.stats() {
        let cut: Vec<&str> = stat.name.splitn(2, '.').collect();
        pretty_stats.push(PrettyStat {
            section: cut[0],
            name: cut[1],
            value: stat.get_raw_value(),
        });
    }

    let table = Table::new(pretty_stats).to_string();
    println!("{}", table);
    Ok(())
}
