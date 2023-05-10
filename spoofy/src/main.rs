pub mod util;

#[tokio::main]
async fn main() {
	util::cli::main_loop().await.unwrap();
}
