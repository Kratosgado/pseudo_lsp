mod server;

use server::PseudoServer;
use tower_lsp::{Server, LspService};

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| PseudoServer { client });
    Server::new(stdin, stdout, socket).serve(service).await;
}
