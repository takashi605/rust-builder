#[cfg(test)]
mod tests;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    eprintln!("これは API テスト用のクレートです。cargo test コマンドでテストを実行してください。");

    anyhow::bail!("テストではなく main 関数を実行しようとしました。");
}