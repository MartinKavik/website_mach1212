use aws_sdk_ec2 as ec2;
use moon::*;

async fn frontend() -> Frontend {
    Frontend::new().title("mpruchn")
}

async fn up_msg_handler(_: UpMsgRequest<()>) {}

#[moon::main]
async fn main() -> std::io::Result<()> {
    start(frontend, up_msg_handler, |_| {}).await
}
