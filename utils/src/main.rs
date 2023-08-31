use utils::{build, gen_bindings};

#[tokio::main]
async fn main() {
    build().await.unwrap();
}
