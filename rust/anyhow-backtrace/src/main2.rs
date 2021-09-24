use anyhow::Context;

/*
baz() failed:

Caused by:
    0: bar() failed:
    1: foo() failed:
    2: root cause
*/

fn foo() -> anyhow::Result<()> {
    Err(anyhow::anyhow!("root cause"))
}

fn bar() -> anyhow::Result<()> {
    foo().context("foo() failed:")?;
    Ok(())
}

fn baz() -> anyhow::Result<()> {
    bar().context("bar() failed:")?;
    Ok(())
}

fn main() {
    match baz().context("baz() failed:") {
        Ok(value) => value,
        Err(err) => println!("{:?}", err),
    };
}
