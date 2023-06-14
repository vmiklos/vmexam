#![deny(warnings)]
#![warn(clippy::all)]
#![warn(missing_docs)]

use crate::yattag;

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
            title.text("hyphen");
        }
        {
            let body = html.tag("body", &[]);
            {
                let form = body.tag(
                    "form",
                    &[
                        ("action", "/apps/hyphen/"),
                        ("method", "GET"),
                        ("enctype", "multipart/form-data"),
                    ],
                );
                {
                    let label = form.tag("label", &[("for", "word")]);
                    label.text("Word: ");
                }
                form.tag("input", &[("type", "text"), ("name", "word")]);
                form.text(" ");
                {
                    let button = form.tag("button", &[]);
                    button.text("Hyphenate");
                }
            }

            if let Some(word) = request.get_param("word") {
                let dict = hyphen::HyphenDict::new("/usr/share/hyphen/hyph_hu_HU.dic")?;
                let hyphenated = dict.hyphenate(&word)?;
                body.text(&format!("Hyphenated: {hyphenated}"));
            }
        }
    }
    Ok(doc.get_value())
}

pub fn app(request: &rouille::Request) -> rouille::Response {
    match our_app(request) {
        Ok(html) => rouille::Response::html(html),
        Err(err) => rouille::Response::text(format!("Err: {err:?}")),
    }
}
