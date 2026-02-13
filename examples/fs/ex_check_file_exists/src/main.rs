use aat_fs::{
    adapter::FileToolsAdapter,
    create_file_exists_tool_de, create_file_size_tool_de, create_read_chunk_tool_de,
    service::{
        fileexiststool::FileExistsToolService, filesizetool::FileSizeToolService,
        sizeawarereadchunktool::SizeAwareReadChunkToolService,
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
    let file_tools_adapter = Arc::new(FileToolsAdapter);
    let file_exists_tool =
        create_file_exists_tool_de(FileExistsToolService(file_tools_adapter.clone()));
    let filesizetool = create_file_size_tool_de(FileSizeToolService(file_tools_adapter.clone()));
    let read_chunk_tool = create_read_chunk_tool_de(SizeAwareReadChunkToolService {
        chunk_reader: file_tools_adapter.clone(),
        filesize_determiner: file_tools_adapter.clone(),
    });

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
    .tool(file_exists_tool)
    .tool(read_chunk_tool)
    .tool(filesizetool)
    .build()
    .expect("error creating llmagent");

    Launcher::new(Arc::new(agent))
        .run()
        .await
        .expect("runner terminated with error");
}

/* PROMPT:
Hi.
Hier sind drei Dateien: /home/technose02/Downloads/ki_und_klo.adoc, Cargo.toml und /data0/inference/ComfyUI/output/video/LTX_2.0_i2v_00001_.mp4.
Prüfe für jede Datei
- ob sie existiert,
- wenn sie existiert, wie groß sie ist und (sonst ignorieren)
- wenn sie existiert und kleiner als 1MB ist, lies 100 Bytes ab offset 42 aus der Datei (sonst ignorieren)

Stelle die Ergebnisse als AsciiDoc-Tabelle dar
*/
