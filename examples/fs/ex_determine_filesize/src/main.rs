use aat_fs::{
    adapter::SimpleFileSizeAdapter, create_file_size_tool_de,
    service::filesizetool::FileSizeToolService,
};
use adk_rust::{Launcher, agent::LlmAgentBuilder, model::openai::OpenAIClient};
use std::sync::Arc;

const MAI_SERVER_URL: &str = "https://mai-server.ipv64.net:8080/api/v1";
const MODEL: &str = "glm-4.7-flash-ud-q8_k_xl-200000";
const MAI_SERVER_APIKEY_VAR: &str = "MAI_SERVER_APIKEY";

#[tokio::main]
async fn main() {
    // configure tool
    let filesizetool =
        create_file_size_tool_de(FileSizeToolService(Arc::new(SimpleFileSizeAdapter)));

    let api_key = {
        dotenv::from_path(".env").expect("could not load environment");
        std::env::var(MAI_SERVER_APIKEY_VAR).expect("could not get api_key from environment")
    };

    let model = OpenAIClient::compatible(api_key, MAI_SERVER_URL, MODEL)
        .expect("error creating mai-server-client");

    let agent = LlmAgentBuilder::new("simple-agent")
    .model(Arc::new(model))
    .description("Einfacher Agent")
    .instruction("Du bist ein einfacher Assistent für Docs-as-Code mit Spezialisierung auf Asciidoc und Antora")
    .tool(filesizetool)
    .build()
    .expect("error creating llmagent");

    Launcher::new(Arc::new(agent))
        .run()
        .await
        .expect("runner terminated with error");
}

/* PROMPT:
Hi. Wie groß ist die Datei 'target/CACHEDIR.TAG'?
*/
