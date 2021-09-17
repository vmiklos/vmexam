use anyhow::Context;

/*
Error: bar() failed:

Caused by:
    0: foo() failed:
    1: root cause
*/

fn foo() -> anyhow::Result<()> {
    Err(anyhow::anyhow!("root cause"))
}

fn bar() -> anyhow::Result<()> {
    foo().context("foo() failed:")?;
    Ok(())
}

fn main() -> anyhow::Result<()> {
    bar().context("bar() failed:")?;
    Ok(())
}
