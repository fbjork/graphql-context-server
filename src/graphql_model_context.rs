use serde::Deserialize;
use std::env;
use zed::settings::ContextServerSettings;
use zed_extension_api::{self as zed, serde_json, Command, ContextServerId, Project, Result};

const PACKAGE_NAME: &str = "graphql-context-server";
const PACKAGE_VERSION: &str = "0.0.1";
const SERVER_PATH: &str = "node_modules/graphql-context-server/index.mjs";

struct GraphQLModelContextExtension;

#[derive(Debug, Deserialize)]
struct GraphQLContextServerSettings {
    api_url: String,
}

impl zed::Extension for GraphQLModelContextExtension {
    fn new() -> Self {
        Self
    }

    fn context_server_command(
        &mut self,
        _context_server_id: &ContextServerId,
        project: &Project,
    ) -> Result<Command> {
        let version = zed::npm_package_installed_version(PACKAGE_NAME)?;
        if version.as_deref() != Some(PACKAGE_VERSION) {
            zed::npm_install_package(PACKAGE_NAME, PACKAGE_VERSION)?;
        }

        let settings = ContextServerSettings::for_project("graphql-context-server", project)?;
        let Some(settings) = settings.settings else {
            return Err("missing `api_url` setting".into());
        };
        let settings: GraphQLContextServerSettings =
            serde_json::from_value(settings).map_err(|e| e.to_string())?;

        Ok(Command {
            command: "node".to_string(),
            args: vec![env::current_dir()
                .unwrap()
                .join(SERVER_PATH)
                .to_string_lossy()
                .to_string()],
            env: vec![("GRAPHQL_API_URL".into(), settings.api_url)],
        })
    }
}

zed::register_extension!(GraphQLModelContextExtension);
