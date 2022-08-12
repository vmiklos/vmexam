#![deny(warnings)]
#![warn(clippy::all)]
#![warn(missing_docs)]

use anyhow::Context as _;

pub fn our_app(request: &rouille::Request) -> anyhow::Result<String> {
    if request.method() == "POST" {
        let data = rouille::post_input!(request, { formula: String, })
            .context("missing POST parameter")?;
        let evaluated = evalexpr::eval(&data.formula).context("evail() failed")?;
        Ok(format!(
            r#"<html>
    <head>
        <title>calc</title>
    </head>
    <body>
        {}
    </body>
</html>
"#,
            evaluated
        ))
    } else {
        Ok(r#"<html>
    <head>
        <title>calc</title>
    </head>
    <body>
        <label for="formula">Formula:</label><br/>
        <form action="/apps/calc/" method="POST" enctype="multipart/form-data">
            <p><input type="text" name="formula"/></p>
            <p><button>Evaluate</button></p>
        </form>
    </body>
</html>
"#
        .into())
    }
}

pub fn app(request: &rouille::Request) -> rouille::Response {
    match our_app(request) {
        Ok(html) => rouille::Response::html(&html),
        Err(err) => rouille::Response::text(&format!("{:?}", err)),
    }
}
