//! `headroom-xray` binary entrypoint.

use anyhow::Result;
use clap::Parser;

/// headroom xray — multi-CLI context-bloat diagnostics.
///
/// Wraps CodeBurn (https://github.com/getagentseal/codeburn, MIT) and adds
/// a Headroom-specific compression-opportunity footer.
#[derive(Parser, Debug)]
#[command(name = "headroom-xray", version, about, long_about = None)]
struct Cli {
    /// Suppress the Headroom footer (CodeBurn output only).
    #[arg(long, env = "HEADROOM_XRAY_NO_FOOTER")]
    no_footer: bool,

    /// Emit debug logs about the footer pipeline to stderr.
    #[arg(long)]
    xray_debug: bool,

    /// Show CodeBurn's own --help (not headroom-xray's wrapper help).
    #[arg(long, conflicts_with_all = ["no_footer", "codeburn_args"])]
    help_codeburn: bool,

    /// All arguments forwarded to CodeBurn (e.g., `report`, `today`, `optimize`).
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    codeburn_args: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.xray_debug {
        tracing_subscriber::fmt()
            .with_env_filter("headroom_xray=debug")
            .with_writer(std::io::stderr)
            .init();
    }

    if let Err(e) = headroom_xray::node::check() {
        eprintln!("{e}");
        let code = match e {
            headroom_xray::node::NodeError::NotFound => 127,
            _ => 1,
        };
        std::process::exit(code);
    }

    // TODO Task 3: spawn npx codeburn and forward stdio.
    // (`cli` field accesses arrive in Task 3+; suppress unused-field warnings for now.)
    let _ = cli;
    eprintln!("headroom-xray: Node OK; CodeBurn invocation not yet wired");
    Ok(())
}
