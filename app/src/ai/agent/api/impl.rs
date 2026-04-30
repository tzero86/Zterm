use std::{collections::HashMap, sync::Arc};

use crate::{
    ai::{
        agent::{redaction, AIAgentInput},
        local_llm::{AgentMessage, ChatMessage, LocalLLMClient, LocalLLMProvider, ToolCallInfo},
    },
    terminal::model::session::SessionType,
};
use futures_util::StreamExt;
use uuid::Uuid;
use warp_multi_agent_api as api;
use zterm_core::features::FeatureFlag;

use crate::server::server_api::ServerApi;

use super::{convert_to::convert_input, ConvertToAPITypeError, RequestParams, ResponseStream};

pub async fn generate_multi_agent_output(
    server_api: Arc<ServerApi>,
    mut params: RequestParams,
    cancellation_rx: futures::channel::oneshot::Receiver<()>,
) -> Result<ResponseStream, ConvertToAPITypeError> {
    if let Some((provider, model_name)) = parse_local_model_id(&params.model) {
        return generate_local_llm_output(provider, model_name, params, cancellation_rx).await;
    }

    let supported_tools = params
        .supported_tools_override
        .take()
        .unwrap_or_else(|| get_supported_tools(&params));
    let supported_cli_agent_tools = get_supported_cli_agent_tools(&params);
    let mut logging_metadata = HashMap::new();
    if let Some(metadata) = params.metadata {
        logging_metadata.insert(
            "is_autodetected_user_query".to_owned(),
            prost_types::Value {
                kind: Some(prost_types::value::Kind::BoolValue(
                    metadata.is_autodetected_user_query,
                )),
            },
        );
        logging_metadata.insert(
            "entrypoint".to_owned(),
            prost_types::Value {
                kind: Some(prost_types::value::Kind::StringValue(
                    metadata.entrypoint.entrypoint(),
                )),
            },
        );
        logging_metadata.insert(
            "is_auto_resume_after_error".to_owned(),
            prost_types::Value {
                kind: Some(prost_types::value::Kind::BoolValue(
                    metadata.is_auto_resume_after_error,
                )),
            },
        );
    }

    if params.should_redact_secrets {
        redaction::redact_inputs(&mut params.input);
    }

    let mut api_keys = params.api_keys;
    if let Some(api_keys) = &mut api_keys {
        api_keys.allow_use_of_warp_credits = params.allow_use_of_warp_credits_with_byok;
    }

    let request = api::Request {
        task_context: Some(api::request::TaskContext {
            tasks: params.tasks,
        }),
        input: Some(convert_input(params.input)?),
        settings: Some(api::request::Settings {
            model_config: Some(api::request::settings::ModelConfig {
                base: params.model.into(),
                cli_agent: params.cli_agent_model.into(),
                computer_use_agent: params.computer_use_model.into(),
                ..Default::default()
            }),
            rules_enabled: params.is_memory_enabled,
            warp_drive_context_enabled: params.warp_drive_context_enabled,
            web_context_retrieval_enabled: true,
            supports_parallel_tool_calls: true,
            use_anthropic_text_editor_tools: false,
            planning_enabled: params.planning_enabled,
            supports_create_files: true,
            supported_tools: supported_tools.into_iter().map(Into::into).collect(),
            supports_long_running_commands: true,
            should_preserve_file_content_in_history: true,
            supports_todos_ui: true,
            supports_linked_code_blocks: FeatureFlag::LinkedCodeBlocks.is_enabled(),
            supports_started_child_task_message: true,
            supports_suggest_prompt: true,
            supports_read_image_files: FeatureFlag::ReadImageFiles.is_enabled(),
            supports_reasoning_message: true,
            api_keys,
            autonomy_level: params.autonomy_level.into(),
            isolation_level: params.isolation_level.into(),
            web_search_enabled: params.web_search_enabled,
            supported_cli_agent_tools: supported_cli_agent_tools
                .into_iter()
                .map(Into::into)
                .collect(),
            supports_v4a_file_diffs: FeatureFlag::V4AFileDiffs.is_enabled(),
            supports_summarization_via_message_replacement:
                FeatureFlag::SummarizationViaMessageReplacement.is_enabled(),
            supports_bundled_skills: FeatureFlag::BundledSkills.is_enabled(),
            supports_research_agent: params.research_agent_enabled,
            supports_orchestration_v2: FeatureFlag::OrchestrationV2.is_enabled(),
        }),
        metadata: Some(api::request::Metadata {
            logging: logging_metadata,
            conversation_id: params
                .conversation_token
                .as_ref()
                .map(|token| token.as_str().to_string())
                .unwrap_or_default(),
            ambient_agent_task_id: params
                .ambient_agent_task_id
                .map(|id| id.to_string())
                .unwrap_or_default(),
            forked_from_conversation_id: if params.conversation_token.is_none() {
                // We only include this param on our initial request to the server
                // (when the forked conversation has not been asigned a new id yet).
                params
                    .forked_from_conversation_token
                    .map(|token| token.as_str().to_string())
                    .unwrap_or_default()
            } else {
                String::new()
            },
            parent_agent_id: params.parent_agent_id.unwrap_or_default(),
            agent_name: params.agent_name.unwrap_or_default(),
        }),
        existing_suggestions: params
            .existing_suggestions
            .map(|suggestions| suggestions.into()),
        mcp_context: params.mcp_context.map(Into::into),
    };

    // TODO: Integrate local LLM when enabled
    // When AppContext is available, add:
    // use crate::ai::local_llm::{LocalLLMClient, LocalLLMSettings};
    //
    // if let Some(local_settings) = LocalLLMSettings::as_ref(app_context) {
    //     if local_settings.is_enabled() {
    //         if let Some(model_name) = &local_settings.selected_model {
    //             log::info!("Using local LLM model: {}", model_name);
    //             let client = LocalLLMClient::new(local_settings.base_url());
    //             if let Ok(stream) = generate_local_llm_output(client, model_name.clone(), params).await {
    //                 return Ok(stream);
    //             } else {
    //                 log::warn!("Local LLM inference failed, falling back to cloud");
    //             }
    //         }
    //     }
    // }

    let response_stream = server_api.generate_multi_agent_output(&request).await;
    match response_stream {
        Ok(stream) => {
            let output_stream = stream.take_until(cancellation_rx);
            Ok(Box::pin(output_stream))
        }
        Err(e) => {
            let (tx, rx) = async_channel::unbounded();
            let _ = tx.send(Err(e)).await;
            Ok(Box::pin(rx))
        }
    }
}

fn get_supported_tools(params: &RequestParams) -> Vec<api::ToolType> {
    let mut supported_tools = vec![
        api::ToolType::Grep,
        api::ToolType::FileGlob,
        api::ToolType::FileGlobV2,
        api::ToolType::ReadMcpResource,
        api::ToolType::CallMcpTool,
        api::ToolType::InitProject,
        api::ToolType::OpenCodeReview,
        api::ToolType::RunShellCommand,
        api::ToolType::SuggestNewConversation,
        api::ToolType::Subagent,
        api::ToolType::WriteToLongRunningShellCommand,
        api::ToolType::ReadShellCommandOutput,
        api::ToolType::ReadDocuments,
        api::ToolType::CreateDocuments,
        api::ToolType::EditDocuments,
        api::ToolType::SuggestPrompt,
    ];

    if FeatureFlag::ConversationsAsContext.is_enabled() {
        supported_tools.push(api::ToolType::FetchConversation);
    }

    match params.session_context.session_type() {
        None | Some(SessionType::Local) => {
            supported_tools.extend(&[
                api::ToolType::ReadFiles,
                api::ToolType::ApplyFileDiffs,
                api::ToolType::SearchCodebase,
            ]);

            if FeatureFlag::ArtifactCommand.is_enabled() {
                supported_tools.push(api::ToolType::UploadFileArtifact);
            }
        }
        Some(SessionType::WarpifiedRemote { host_id: Some(_) }) => {
            // Remote session with a known host — enable tools that route
            // through RemoteServerClient. The host_id is only populated
            // after a successful connection handshake, so its presence is a
            // sufficient proxy for client availability.
            // SearchCodebase remains disabled (follow-up work).
            supported_tools.extend(&[api::ToolType::ReadFiles, api::ToolType::ApplyFileDiffs]);
        }
        Some(SessionType::WarpifiedRemote { host_id: None }) => {
            // Feature flag off or not yet connected — no remote tools.
        }
    }

    if FeatureFlag::AgentModeComputerUse.is_enabled() && params.computer_use_enabled {
        supported_tools.extend(&[api::ToolType::UseComputer]);
        supported_tools.extend(&[api::ToolType::RequestComputerUse])
    }

    if FeatureFlag::PRCommentsSlashCommand.is_enabled() {
        supported_tools.push(api::ToolType::InsertReviewComments);
    }

    if FeatureFlag::ListSkills.is_enabled() {
        supported_tools.push(api::ToolType::ReadSkill);
    }

    if params.orchestration_enabled {
        supported_tools.push(if FeatureFlag::OrchestrationV2.is_enabled() {
            api::ToolType::StartAgentV2
        } else {
            api::ToolType::StartAgent
        });
        supported_tools.push(api::ToolType::SendMessageToAgent);
    }

    if FeatureFlag::AskUserQuestion.is_enabled() && params.ask_user_question_enabled {
        supported_tools.push(api::ToolType::AskUserQuestion);
    }

    supported_tools
}

fn get_supported_cli_agent_tools(params: &RequestParams) -> Vec<api::ToolType> {
    let mut supported_cli_agent_tools = vec![
        api::ToolType::WriteToLongRunningShellCommand,
        api::ToolType::ReadShellCommandOutput,
        api::ToolType::Grep,
        api::ToolType::FileGlob,
        api::ToolType::FileGlobV2,
    ];

    if FeatureFlag::TransferControlTool.is_enabled() {
        supported_cli_agent_tools.push(api::ToolType::TransferShellCommandControlToUser);
    }

    match params.session_context.session_type() {
        None | Some(SessionType::Local) => {
            supported_cli_agent_tools
                .extend(&[api::ToolType::ReadFiles, api::ToolType::SearchCodebase]);
        }
        Some(SessionType::WarpifiedRemote { host_id: Some(_) }) => {
            supported_cli_agent_tools.push(api::ToolType::ReadFiles);
        }
        Some(SessionType::WarpifiedRemote { host_id: None }) => {}
    }

    supported_cli_agent_tools
}

#[cfg(test)]
#[path = "impl_tests.rs"]
mod tests;

async fn generate_local_llm_output(
    provider: LocalLLMProvider,
    model: String,
    params: RequestParams,
    cancellation_rx: futures::channel::oneshot::Receiver<()>,
) -> Result<ResponseStream, ConvertToAPITypeError> {
    let cwd = params
        .session_context
        .current_working_directory()
        .clone()
        .unwrap_or_else(|| ".".to_owned());

    let base_url = params
        .local_llm_base_url
        .clone()
        .unwrap_or_else(|| provider.default_base_url().to_string());
    let client = LocalLLMClient::new(provider.clone(), base_url);

    let tools = build_agent_tools();
    let mut messages = build_initial_messages(&params.input, &cwd);

    let mut final_text = String::new();
    const MAX_ITERATIONS: usize = 10;

    for _ in 0..MAX_ITERATIONS {
        let response = client
            .generate_with_tools(messages.clone(), &model, Some(tools.clone()))
            .await
            .map_err(|e| {
                ConvertToAPITypeError::Other(anyhow::anyhow!("Local LLM request failed: {e}"))
            })?;

        let choice = response.choices.into_iter().next().ok_or_else(|| {
            ConvertToAPITypeError::Other(anyhow::anyhow!("LLM returned no choices"))
        })?;

        let tool_calls = choice.message.tool_calls.clone().unwrap_or_default();

        if tool_calls.is_empty() {
            final_text = choice.message.content.unwrap_or_default();
            break;
        }

        messages.push(AgentMessage {
            role: "assistant".to_owned(),
            content: choice.message.content,
            tool_calls: Some(tool_calls.clone()),
            tool_call_id: None,
            name: None,
        });

        for tc in &tool_calls {
            let result = execute_tool_call(tc, &cwd).await;
            messages.push(AgentMessage {
                role: "tool".to_owned(),
                content: Some(result),
                tool_calls: None,
                tool_call_id: Some(tc.id.clone()),
                name: Some(tc.function.name.clone()),
            });
        }
    }

    if final_text.trim().is_empty() {
        final_text = "The local model did not return any content.".to_owned();
    }

    let request_id = Uuid::new_v4().to_string();
    let (task_id, needs_create_task) = if let Some(existing_task) = params.tasks.first() {
        (existing_task.id.clone(), false)
    } else {
        (Uuid::new_v4().to_string(), true)
    };
    let message_id = format!("local-message-{request_id}");

    let init_event = api::ResponseEvent {
        r#type: Some(api::response_event::Type::Init(
            api::response_event::StreamInit {
                request_id: request_id.clone(),
                conversation_id: params
                    .conversation_token
                    .as_ref()
                    .map(|token| token.as_str().to_string())
                    .unwrap_or_default(),
                run_id: String::new(),
            },
        )),
    };

    let mut actions: Vec<api::ClientAction> = Vec::new();

    if needs_create_task {
        actions.push(api::ClientAction {
            action: Some(api::client_action::Action::CreateTask(
                api::client_action::CreateTask {
                    task: Some(api::Task {
                        id: task_id.clone(),
                        messages: vec![],
                        dependencies: None,
                        description: String::new(),
                        summary: String::new(),
                        server_data: String::new(),
                    }),
                },
            )),
        });
    }

    actions.push(api::ClientAction {
        action: Some(api::client_action::Action::AddMessagesToTask(
            api::client_action::AddMessagesToTask {
                task_id: task_id.clone(),
                messages: vec![api::Message {
                    id: message_id,
                    task_id,
                    server_message_data: String::new(),
                    citations: vec![],
                    message: Some(api::message::Message::AgentOutput(
                        api::message::AgentOutput { text: final_text },
                    )),
                    request_id,
                    timestamp: None,
                }],
            },
        )),
    });

    let client_actions_event = api::ResponseEvent {
        r#type: Some(api::response_event::Type::ClientActions(
            api::response_event::ClientActions { actions },
        )),
    };

    let finished_event = api::ResponseEvent {
        r#type: Some(api::response_event::Type::Finished(
            api::response_event::StreamFinished {
                reason: Some(api::response_event::stream_finished::Reason::Done(
                    api::response_event::stream_finished::Done {},
                )),
                conversation_usage_metadata: None,
                token_usage: vec![],
                should_refresh_model_config: false,
                request_cost: None,
            },
        )),
    };

    let output_stream = futures_util::stream::iter(vec![
        Ok(init_event),
        Ok(client_actions_event),
        Ok(finished_event),
    ])
    .take_until(cancellation_rx);
    Ok(Box::pin(output_stream))
}

fn parse_local_model_id(model_id: &crate::ai::llms::LLMId) -> Option<(LocalLLMProvider, String)> {
    let model_id = model_id.to_string();
    let parse_provider =
        |prefix: &str, provider: LocalLLMProvider| -> Option<(LocalLLMProvider, String)> {
            if !model_id.starts_with(prefix) {
                return None;
            }
            let encoded = &model_id[prefix.len()..];
            decode_hex_model_name(encoded).map(|decoded| (provider, decoded))
        };

    parse_provider("local-ollama-hex-", LocalLLMProvider::Ollama)
        .or_else(|| parse_provider("local-lmstudio-hex-", LocalLLMProvider::LMStudio))
        .or_else(|| parse_provider("local-custom-hex-", LocalLLMProvider::Custom))
        // Legacy IDs used before hex encoding.
        .or_else(|| {
            model_id
                .strip_prefix("local-ollama-")
                .map(|name| (LocalLLMProvider::Ollama, name.to_owned()))
        })
        .or_else(|| {
            model_id
                .strip_prefix("local-lmstudio-")
                .map(|name| (LocalLLMProvider::LMStudio, name.to_owned()))
        })
        .or_else(|| {
            model_id
                .strip_prefix("local-custom-")
                .map(|name| (LocalLLMProvider::Custom, name.to_owned()))
        })
}

fn decode_hex_model_name(encoded: &str) -> Option<String> {
    if encoded.is_empty() || encoded.len() % 2 != 0 {
        return None;
    }

    let bytes = (0..encoded.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&encoded[i..i + 2], 16))
        .collect::<Result<Vec<_>, _>>()
        .ok()?;
    String::from_utf8(bytes).ok()
}

fn extract_local_chat_messages(inputs: &[AIAgentInput]) -> Vec<ChatMessage> {
    let mut messages = Vec::new();
    for input in inputs {
        match input {
            AIAgentInput::UserQuery { query, .. }
            | AIAgentInput::AutoCodeDiffQuery { query, .. }
            | AIAgentInput::CreateNewProject { query, .. } => {
                if !query.trim().is_empty() {
                    messages.push(ChatMessage {
                        role: "user".to_owned(),
                        content: query.clone(),
                    });
                }
            }
            AIAgentInput::SummarizeConversation {
                prompt: Some(prompt),
            } if !prompt.trim().is_empty() => {
                messages.push(ChatMessage {
                    role: "user".to_owned(),
                    content: prompt.clone(),
                });
            }
            _ => {}
        }
    }
    messages
}

fn build_agent_tools() -> Vec<serde_json::Value> {
    vec![
        serde_json::json!({
            "type": "function",
            "function": {
                "name": "run_shell_command",
                "description": "Run a shell command in the current working directory and return its output. Use this to explore the filesystem, run programs, check git status, etc.",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "command": {
                            "type": "string",
                            "description": "The shell command to execute"
                        }
                    },
                    "required": ["command"]
                }
            }
        }),
        serde_json::json!({
            "type": "function",
            "function": {
                "name": "read_file",
                "description": "Read the contents of a file. Path can be absolute or relative to the current working directory.",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "path": {
                            "type": "string",
                            "description": "Path to the file to read"
                        }
                    },
                    "required": ["path"]
                }
            }
        }),
    ]
}

fn build_initial_messages(inputs: &[AIAgentInput], cwd: &str) -> Vec<AgentMessage> {
    let system_content = format!(
        "You are a helpful terminal assistant running inside a terminal application. \
         The user's current working directory is: {cwd}\n\
         You have access to tools to run shell commands and read files. \
         Use them freely to explore the filesystem and answer questions accurately.\n\
         When running commands on Windows, use PowerShell or cmd syntax."
    );

    let mut messages = vec![AgentMessage {
        role: "system".to_owned(),
        content: Some(system_content),
        tool_calls: None,
        tool_call_id: None,
        name: None,
    }];

    for input in inputs {
        match input {
            AIAgentInput::UserQuery { query, .. }
            | AIAgentInput::AutoCodeDiffQuery { query, .. }
            | AIAgentInput::CreateNewProject { query, .. } => {
                if !query.trim().is_empty() {
                    messages.push(AgentMessage {
                        role: "user".to_owned(),
                        content: Some(query.clone()),
                        tool_calls: None,
                        tool_call_id: None,
                        name: None,
                    });
                }
            }
            AIAgentInput::SummarizeConversation {
                prompt: Some(prompt),
            } if !prompt.trim().is_empty() => {
                messages.push(AgentMessage {
                    role: "user".to_owned(),
                    content: Some(prompt.clone()),
                    tool_calls: None,
                    tool_call_id: None,
                    name: None,
                });
            }
            _ => {}
        }
    }

    messages
}

async fn execute_tool_call(tc: &ToolCallInfo, cwd: &str) -> String {
    let args: serde_json::Value = serde_json::from_str(&tc.function.arguments).unwrap_or_default();

    match tc.function.name.as_str() {
        "run_shell_command" => {
            let command = match args["command"].as_str() {
                Some(c) => c.to_owned(),
                None => return "Error: missing 'command' argument".to_owned(),
            };
            run_shell_command_in_cwd(&command, cwd).await
        }
        "read_file" => {
            let path = match args["path"].as_str() {
                Some(p) => p.to_owned(),
                None => return "Error: missing 'path' argument".to_owned(),
            };
            let full_path = if std::path::Path::new(&path).is_absolute() {
                path
            } else {
                format!("{}/{}", cwd, path)
            };
            match tokio::fs::read_to_string(&full_path).await {
                Ok(content) => {
                    if content.len() > 8192 {
                        format!("[truncated to 8192 chars]\n{}", &content[..8192])
                    } else {
                        content
                    }
                }
                Err(e) => format!("Error reading file: {e}"),
            }
        }
        _ => format!("Unknown tool: {}", tc.function.name),
    }
}

async fn run_shell_command_in_cwd(command: &str, cwd: &str) -> String {
    const SHELL_TIMEOUT_SECS: u64 = 30;

    let output_future = async {
        #[cfg(target_os = "windows")]
        {
            tokio::process::Command::new("cmd")
                .args(["/C", command])
                .current_dir(cwd)
                .output()
                .await
        }
        #[cfg(not(target_os = "windows"))]
        {
            tokio::process::Command::new("sh")
                .args(["-c", command])
                .current_dir(cwd)
                .output()
                .await
        }
    };

    let output = match tokio::time::timeout(
        std::time::Duration::from_secs(SHELL_TIMEOUT_SECS),
        output_future,
    )
    .await
    {
        Ok(result) => result,
        Err(_) => {
            return format!("Error: command timed out after {SHELL_TIMEOUT_SECS}s");
        }
    };

    match output {
        Ok(out) => {
            let stdout = String::from_utf8_lossy(&out.stdout);
            let stderr = String::from_utf8_lossy(&out.stderr);
            let mut result = String::new();
            if !stdout.is_empty() {
                result.push_str(&stdout);
            }
            if !stderr.is_empty() {
                if !result.is_empty() {
                    result.push('\n');
                }
                result.push_str("[stderr] ");
                result.push_str(&stderr);
            }
            if result.is_empty() {
                result = "(no output)".to_owned();
            }
            if result.len() > 8192 {
                result.truncate(8192);
                result.push_str("\n[output truncated]");
            }
            result
        }
        Err(e) => format!("Command execution error: {e}"),
    }
}
