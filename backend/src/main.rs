use moon::*;

async fn frontend() -> Frontend {
    Frontend::new().title("mpruchn").append_to_head("<meta name=\"author\" content=\"Maciej Pruchnik\"/>\n<meta name=\"description\" content=\"My personal website\"")
}

async fn up_msg_handler(_: UpMsgRequest<()>) {}

#[moon::main]
async fn main() -> std::io::Result<()> {
    start(frontend, up_msg_handler, |_| {}).await
}
