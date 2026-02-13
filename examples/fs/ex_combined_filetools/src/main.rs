use aat_fs::{
    adapter::{SimpleFileSizeAdapter, SimpleReadChunkAdapter},
    create_file_size_tool_de, create_read_chunk_tool_de,
    service::{
        filesizetool::FileSizeToolService, sizeawarereadchunktool::SizeAwareReadChunkToolService,
    },
};
use adk_rust::{Launcher, agent::LlmAgentBuilder, model::openai::OpenAIClient};
use std::sync::Arc;

const MAI_SERVER_URL: &str = "https://mai-server.ipv64.net:8080/api/v1";
const MODEL: &str = "glm-4.7-flash-ud-q8_k_xl-200000";
const MAI_SERVER_APIKEY_VAR: &str = "MAI_SERVER_APIKEY";

#[tokio::main]
async fn main() {
    // configure tools
    let chunk_read_adapter = Arc::new(SimpleReadChunkAdapter);
    let file_size_adapter = Arc::new(SimpleFileSizeAdapter);

    let readchunk_tool = create_read_chunk_tool_de(SizeAwareReadChunkToolService {
        chunk_reader: chunk_read_adapter,
        filesize_determiner: file_size_adapter.clone(),
    });
    let filesizetool = create_file_size_tool_de(FileSizeToolService(file_size_adapter));

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
    .tool(readchunk_tool)
    .tool(filesizetool)
    .build()
    .expect("error creating llmagent");

    Launcher::new(Arc::new(agent))
        .run()
        .await
        .expect("runner terminated with error");
}

/* PROMPT:
Hi. Bitte lies die ersten 200 Bytes aus der Datei 'target/CACHEDIR.TAG' und sag mir, ob es sich um Text oder allgemeine Binärdaten handelt.
*/
