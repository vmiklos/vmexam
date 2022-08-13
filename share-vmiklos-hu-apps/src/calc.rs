#![deny(warnings)]
#![warn(clippy::all)]
#![warn(missing_docs)]

use crate::yattag;
use anyhow::Context as _;

pub fn our_app(request: &rouille::Request) -> anyhow::Result<String> {
    let doc = yattag::Doc::new();
    {
        let html = doc.tag("html", &[]);
        {
            let head = html.tag("head", &[]);
            head.stag(
                "meta",
                &[
                    ("name", "viewport"),
                    ("content", "width=device-width, initial-scale=1"),
                ],
            );
            let title = head.tag("title", &[]);
            title.text("calc");
        }
        {
            let body = html.tag("body", &[]);
            if request.method() == "POST" {
                let data = rouille::post_input!(request, { formula: String, })
                    .context("missing POST parameter")?;
                let evaluated = evalexpr::eval(&data.formula).context("evail() failed")?;
                body.text(&format!("Ok: {}", evaluated))
            } else {
                {
                    let label = body.tag("label", &[("for", "formula")]);
                    label.text("Formula:");
                }
                body.stag("br", &[]);
                {
                    let form = body.tag(
                        "form",
                        &[
                            ("action", "/apps/calc/"),
                            ("method", "POST"),
                            ("enctype", "multipart/form-data"),
                        ],
                    );
                    {
                        let p = form.tag("p", &[]);
                        p.tag("input", &[("type", "text"), ("name", "formula")]);
                    }
                    {
                        let p = form.tag("p", &[]);
                        let button = p.tag("button", &[]);
                        button.text("Evaluate");
                    }
                }
            }
        }
    }
    Ok(doc.get_value())
}

pub fn app(request: &rouille::Request) -> rouille::Response {
    match our_app(request) {
        Ok(html) => rouille::Response::html(&html),
        Err(err) => rouille::Response::text(&format!("Err: {:?}", err)),
    }
}
