mod hyphen;

fn main() -> anyhow::Result<()> {
    let port = 8001;
    let prefix = "/apps";
    println!(
        "Starting the server at <http://127.0.0.1:{}{}/>.",
        port, prefix
    );
    rouille::start_server_with_pool(format!("127.0.0.1:{}", port), None, move |request| {
        if request.url().starts_with("/apps/hyphen") {
            return hyphen::app(request);
        }

        rouille::Response::empty_404()
    });
}
