use anyhow::Context as _;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let (_, tokens) = args.split_first().context("args.split_first() failed")?;
    let expr = tokens.join(" ");
    let mut ctx = calc::Context::<f64>::default();
    let res = ctx.evaluate(&expr).context("evaluate() failed")?;
    println!("{expr}={res}");
    Ok(())
}
