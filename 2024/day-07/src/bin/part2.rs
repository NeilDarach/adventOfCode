use day_07::part2::process;
use miette::Context;
use tracing_subscriber::fmt::format::FmtSpan;


#[tracing::instrument]
fn main() -> miette::Result<()> {

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_span_events(FmtSpan::ENTER | FmtSpan::CLOSE)
        .with_target(true)
        .init();

    let file = include_str!("../../input2.txt");
    let result = process(file).context("process part 2")?;
    println!("{}", result);
    Ok(())
}
