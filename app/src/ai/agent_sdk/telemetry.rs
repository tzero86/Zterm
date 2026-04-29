use crate::features::FeatureFlag;
use serde_json::{json, Value};
use strum_macros::{EnumDiscriminants, EnumIter};
use zterm_core::telemetry::{EnablementState, TelemetryEvent, TelemetryEventDesc};

#[derive(Debug, EnumDiscriminants)]
#[strum_discriminants(derive(EnumIter))]
pub(super) enum CliTelemetryEvent {
    /// Executing `zterm agent run`
    AgentRun {
        gui: bool,
        requested_mcp_servers: usize,
        has_environment: bool,
        /// Optional task ID when running against an ambient agent task.
        task_id: Option<String>,
        /// Which execution harness was selected (e.g. "oz", "claude").
        harness: String,
    },
    /// Executing `zterm agent run-ambient`
    AgentRunAmbient,
    /// Executing `zterm agent profile list`
    AgentProfileList,
    /// Executing `zterm agent list`
    AgentList,
    /// Executing `zterm environment list`
    EnvironmentList,
    /// Executing `zterm environment create`
    EnvironmentCreate,
    /// Executing `zterm environment delete`
    EnvironmentDelete,
    /// Executing `zterm environment update`
    EnvironmentUpdate,
    /// Executing `zterm environment get`
    EnvironmentGet,
    /// Executing `zterm environment image list`
    EnvironmentImageList,
    /// Executing `zterm mcp list`
    MCPList,
    /// Executing `zterm model list`
    ModelList,
    /// Executing `zterm task list`
    TaskList,
    /// Executing `zterm task get`
    TaskGet,
    /// Executing `zterm run conversation get`
    ConversationGet,
    /// Executing `zterm run get <id> --conversation`
    RunConversationGet,
    /// Executing `zterm run message watch`
    RunMessageWatch { harness: &'static str },
    /// Executing `zterm run message send`
    RunMessageSend { harness: &'static str },
    /// Executing `zterm run message list`
    RunMessageList { harness: &'static str },
    /// Executing `zterm run message read`
    RunMessageRead { harness: &'static str },
    /// Executing `zterm run message mark-delivered`
    RunMessageMarkDelivered { harness: &'static str },
    /// Executing `zterm login`
    Login,
    /// Executing `zterm logout`
    Logout,
    /// Executing `zterm whoami`
    Whoami,
    /// Executing `zterm provider setup`
    ProviderSetup,
    /// Executing `zterm provider list`
    ProviderList,
    /// Executing `zterm integration create`
    IntegrationCreate,
    /// Executing `zterm integration update`
    IntegrationUpdate,
    /// Executing `zterm integration list`
    IntegrationList,
    /// Executing `zterm artifact upload`
    ArtifactUpload,
    /// Executing `zterm artifact get`
    ArtifactGet,
    /// Executing `zterm artifact download`
    ArtifactDownload,
    /// Executing `zterm schedule create`
    ScheduleCreate,
    /// Executing `zterm schedule list`
    ScheduleList,
    /// Executing `zterm schedule get`
    ScheduleGet,
    /// Executing `zterm schedule pause`
    SchedulePause,
    /// Executing `zterm schedule unpause`
    ScheduleUnpause,
    /// Executing `zterm schedule update`
    ScheduleUpdate,
    /// Executing `zterm schedule delete`
    ScheduleDelete,
    /// Executing `zterm secret create`
    SecretCreate,
    /// Executing `zterm secret delete`
    SecretDelete,
    /// Executing `zterm secret update`
    SecretUpdate,
    /// Executing `zterm secret list`
    SecretList,
    /// Executing `zterm federate issue-token`
    FederateIssueToken,
    /// Executing `zterm federate issue-gcp-token`
    FederateIssueGcpToken,
    /// Executing `zterm harness-support ping`
    HarnessSupportPing,
    /// Executing `zterm harness-support report-artifact`
    HarnessSupportReportArtifact { artifact_type: &'static str },
    /// Executing `zterm harness-support notify-user`
    HarnessSupportNotifyUser,
    /// Executing `zterm harness-support finish-task`
    HarnessSupportFinishTask { success: bool },
}

impl TelemetryEvent for CliTelemetryEvent {
    fn name(&self) -> &'static str {
        CliTelemetryEventDiscriminants::from(self).name()
    }

    fn payload(&self) -> Option<Value> {
        match self {
            CliTelemetryEvent::AgentRun {
                gui,
                requested_mcp_servers,
                has_environment,
                task_id,
                harness,
            } => Some(json!({
                "gui": gui,
                "requested_mcp_servers": requested_mcp_servers,
                "has_environment": has_environment,
                "task_id": task_id,
                "harness": harness,
            })),
            CliTelemetryEvent::AgentRunAmbient => None,
            CliTelemetryEvent::AgentProfileList => None,
            CliTelemetryEvent::AgentList => None,
            CliTelemetryEvent::EnvironmentList => None,
            CliTelemetryEvent::EnvironmentCreate => None,
            CliTelemetryEvent::EnvironmentDelete => None,
            CliTelemetryEvent::EnvironmentUpdate => None,
            CliTelemetryEvent::EnvironmentGet => None,
            CliTelemetryEvent::EnvironmentImageList => None,
            CliTelemetryEvent::MCPList => None,
            CliTelemetryEvent::ModelList => None,
            CliTelemetryEvent::TaskList => None,
            CliTelemetryEvent::TaskGet => None,
            CliTelemetryEvent::ConversationGet => None,
            CliTelemetryEvent::RunConversationGet => None,
            CliTelemetryEvent::RunMessageWatch { harness } => Some(json!({ "harness": harness })),
            CliTelemetryEvent::RunMessageSend { harness } => Some(json!({ "harness": harness })),
            CliTelemetryEvent::RunMessageList { harness } => Some(json!({ "harness": harness })),
            CliTelemetryEvent::RunMessageRead { harness } => Some(json!({ "harness": harness })),
            CliTelemetryEvent::RunMessageMarkDelivered { harness } => {
                Some(json!({ "harness": harness }))
            }
            CliTelemetryEvent::Login => None,
            CliTelemetryEvent::Logout => None,
            CliTelemetryEvent::Whoami => None,
            CliTelemetryEvent::ProviderSetup => None,
            CliTelemetryEvent::ProviderList => None,
            CliTelemetryEvent::IntegrationCreate => None,
            CliTelemetryEvent::IntegrationUpdate => None,
            CliTelemetryEvent::IntegrationList => None,
            CliTelemetryEvent::ArtifactUpload => None,
            CliTelemetryEvent::ArtifactGet => None,
            CliTelemetryEvent::ArtifactDownload => None,
            CliTelemetryEvent::ScheduleCreate => None,
            CliTelemetryEvent::ScheduleList => None,
            CliTelemetryEvent::ScheduleGet => None,
            CliTelemetryEvent::SchedulePause => None,
            CliTelemetryEvent::ScheduleUnpause => None,
            CliTelemetryEvent::ScheduleUpdate => None,
            CliTelemetryEvent::ScheduleDelete => None,
            CliTelemetryEvent::SecretCreate => None,
            CliTelemetryEvent::SecretDelete => None,
            CliTelemetryEvent::SecretUpdate => None,
            CliTelemetryEvent::SecretList => None,
            CliTelemetryEvent::FederateIssueToken => None,
            CliTelemetryEvent::FederateIssueGcpToken => None,
            CliTelemetryEvent::HarnessSupportPing => None,
            CliTelemetryEvent::HarnessSupportReportArtifact { artifact_type } => {
                Some(json!({ "artifact_type": artifact_type }))
            }
            CliTelemetryEvent::HarnessSupportNotifyUser => None,
            CliTelemetryEvent::HarnessSupportFinishTask { success } => {
                Some(json!({ "success": success }))
            }
        }
    }

    fn description(&self) -> &'static str {
        CliTelemetryEventDiscriminants::from(self).description()
    }

    fn enablement_state(&self) -> EnablementState {
        CliTelemetryEventDiscriminants::from(self).enablement_state()
    }

    fn contains_ugc(&self) -> bool {
        false
    }

    fn event_descs() -> impl Iterator<Item = Box<dyn TelemetryEventDesc>> {
        zterm_core::telemetry::enum_events::<Self>()
    }
}

impl TelemetryEventDesc for CliTelemetryEventDiscriminants {
    fn name(&self) -> &'static str {
        match self {
            CliTelemetryEventDiscriminants::AgentRun => "CLI.Execute.Agent.Run",
            CliTelemetryEventDiscriminants::AgentRunAmbient => "CLI.Execute.Agent.RunAmbient",
            CliTelemetryEventDiscriminants::AgentProfileList => "CLI.Execute.Agent.Profile.List",
            CliTelemetryEventDiscriminants::AgentList => "CLI.Execute.Agent.List",
            CliTelemetryEventDiscriminants::EnvironmentList => "CLI.Execute.Environment.List",
            CliTelemetryEventDiscriminants::EnvironmentCreate => "CLI.Execute.Environment.Create",
            CliTelemetryEventDiscriminants::EnvironmentDelete => "CLI.Execute.Environment.Delete",
            CliTelemetryEventDiscriminants::EnvironmentUpdate => "CLI.Execute.Environment.Update",
            CliTelemetryEventDiscriminants::EnvironmentGet => "CLI.Execute.Environment.Get",
            CliTelemetryEventDiscriminants::EnvironmentImageList => {
                "CLI.Execute.Environment.Image.List"
            }
            CliTelemetryEventDiscriminants::MCPList => "CLI.Execute.MCP.List",
            CliTelemetryEventDiscriminants::ModelList => "CLI.Execute.Model.List",
            CliTelemetryEventDiscriminants::TaskList => "CLI.Execute.Task.List",
            CliTelemetryEventDiscriminants::TaskGet => "CLI.Execute.Task.Get",
            CliTelemetryEventDiscriminants::ConversationGet => "CLI.Execute.Conversation.Get",
            CliTelemetryEventDiscriminants::RunConversationGet => {
                "CLI.Execute.Run.Conversation.Get"
            }
            CliTelemetryEventDiscriminants::RunMessageWatch => "CLI.Execute.Run.Message.Watch",
            CliTelemetryEventDiscriminants::RunMessageSend => "CLI.Execute.Run.Message.Send",
            CliTelemetryEventDiscriminants::RunMessageList => "CLI.Execute.Run.Message.List",
            CliTelemetryEventDiscriminants::RunMessageRead => "CLI.Execute.Run.Message.Read",
            CliTelemetryEventDiscriminants::RunMessageMarkDelivered => {
                "CLI.Execute.Run.Message.MarkDelivered"
            }
            CliTelemetryEventDiscriminants::Login => "CLI.Execute.Login",
            CliTelemetryEventDiscriminants::Logout => "CLI.Execute.Logout",
            CliTelemetryEventDiscriminants::Whoami => "CLI.Execute.Whoami",
            CliTelemetryEventDiscriminants::ProviderSetup => "CLI.Execute.Provider.Setup",
            CliTelemetryEventDiscriminants::ProviderList => "CLI.Execute.Provider.List",
            CliTelemetryEventDiscriminants::IntegrationCreate => "CLI.Execute.Integration.Create",
            CliTelemetryEventDiscriminants::IntegrationUpdate => "CLI.Execute.Integration.Update",
            CliTelemetryEventDiscriminants::IntegrationList => "CLI.Execute.Integration.List",
            CliTelemetryEventDiscriminants::ArtifactUpload => "CLI.Execute.Artifact.Upload",
            CliTelemetryEventDiscriminants::ArtifactGet => "CLI.Execute.Artifact.Get",
            CliTelemetryEventDiscriminants::ArtifactDownload => "CLI.Execute.Artifact.Download",
            CliTelemetryEventDiscriminants::ScheduleCreate => "CLI.Execute.Schedule.Create",
            CliTelemetryEventDiscriminants::ScheduleList => "CLI.Execute.Schedule.List",
            CliTelemetryEventDiscriminants::ScheduleGet => "CLI.Execute.Schedule.Get",
            CliTelemetryEventDiscriminants::SchedulePause => "CLI.Execute.Schedule.Pause",
            CliTelemetryEventDiscriminants::ScheduleUnpause => "CLI.Execute.Schedule.Unpause",
            CliTelemetryEventDiscriminants::ScheduleUpdate => "CLI.Execute.Schedule.Update",
            CliTelemetryEventDiscriminants::ScheduleDelete => "CLI.Execute.Schedule.Delete",
            CliTelemetryEventDiscriminants::SecretCreate => "CLI.Execute.Secret.Create",
            CliTelemetryEventDiscriminants::SecretDelete => "CLI.Execute.Secret.Delete",
            CliTelemetryEventDiscriminants::SecretUpdate => "CLI.Execute.Secret.Update",
            CliTelemetryEventDiscriminants::SecretList => "CLI.Execute.Secret.List",
            CliTelemetryEventDiscriminants::FederateIssueToken => "CLI.Execute.Federate.IssueToken",
            CliTelemetryEventDiscriminants::FederateIssueGcpToken => {
                "CLI.Execute.Federate.IssueGcpToken"
            }
            CliTelemetryEventDiscriminants::HarnessSupportPing => "CLI.Execute.HarnessSupport.Ping",
            CliTelemetryEventDiscriminants::HarnessSupportReportArtifact => {
                "CLI.Execute.HarnessSupport.ReportArtifact"
            }
            CliTelemetryEventDiscriminants::HarnessSupportNotifyUser => {
                "CLI.Execute.HarnessSupport.NotifyUser"
            }
            CliTelemetryEventDiscriminants::HarnessSupportFinishTask => {
                "CLI.Execute.HarnessSupport.FinishTask"
            }
        }
    }

    fn description(&self) -> &'static str {
        match self {
            CliTelemetryEventDiscriminants::AgentRun => "Ran an agent from the Zterm CLI",
            CliTelemetryEventDiscriminants::AgentRunAmbient => {
                "Ran an ambient agent from the Zterm CLI"
            }
            CliTelemetryEventDiscriminants::AgentProfileList => {
                "Listed agent profiles from the Zterm CLI"
            }
            CliTelemetryEventDiscriminants::AgentList => "Listed agents from the Zterm CLI",
            CliTelemetryEventDiscriminants::EnvironmentList => {
                "Listed cloud environments from the Zterm CLI"
            }
            CliTelemetryEventDiscriminants::EnvironmentCreate => {
                "Created a cloud environment from the Zterm CLI"
            }
            CliTelemetryEventDiscriminants::EnvironmentDelete => {
                "Deleted a cloud environment from the Zterm CLI"
            }
            CliTelemetryEventDiscriminants::EnvironmentUpdate => {
                "Updated a cloud environment from the Zterm CLI"
            }
            CliTelemetryEventDiscriminants::EnvironmentGet => {
                "Got cloud environment details from the Zterm CLI"
            }
            CliTelemetryEventDiscriminants::EnvironmentImageList => {
                "Listed available base images from the Zterm CLI"
            }
            CliTelemetryEventDiscriminants::MCPList => "Listed MCP servers from the Zterm CLI",
            CliTelemetryEventDiscriminants::ModelList => "Listed models from the Zterm CLI",
            CliTelemetryEventDiscriminants::TaskList => "Listed tasks from the Zterm CLI",
            CliTelemetryEventDiscriminants::TaskGet => "Got status of task from the Zterm CLI",
            CliTelemetryEventDiscriminants::ConversationGet => {
                "Got conversation by ID from the Zterm CLI"
            }
            CliTelemetryEventDiscriminants::RunConversationGet => {
                "Got run conversation from the Zterm CLI"
            }
            CliTelemetryEventDiscriminants::RunMessageWatch => {
                "Watched run messages from the Zterm CLI"
            }
            CliTelemetryEventDiscriminants::RunMessageSend => {
                "Sent a run message from the Zterm CLI"
            }
            CliTelemetryEventDiscriminants::RunMessageList => {
                "Listed run messages from the Zterm CLI"
            }
            CliTelemetryEventDiscriminants::RunMessageRead => {
                "Read a run message from the Zterm CLI"
            }
            CliTelemetryEventDiscriminants::RunMessageMarkDelivered => {
                "Marked a run message as delivered from the Zterm CLI"
            }
            CliTelemetryEventDiscriminants::Login => "Logged in via the Zterm CLI",
            CliTelemetryEventDiscriminants::Logout => "Logged out via the Zterm CLI",
            CliTelemetryEventDiscriminants::Whoami => {
                "Printed current user info from the Zterm CLI"
            }
            CliTelemetryEventDiscriminants::ProviderSetup => "Set up a provider via the Zterm CLI",
            CliTelemetryEventDiscriminants::ProviderList => "Listed providers from the Zterm CLI",
            CliTelemetryEventDiscriminants::IntegrationCreate => {
                "Created an integration from the Zterm CLI"
            }
            CliTelemetryEventDiscriminants::IntegrationUpdate => {
                "Updated an integration from the Zterm CLI"
            }
            CliTelemetryEventDiscriminants::IntegrationList => {
                "Listed integrations from the Zterm CLI"
            }
            CliTelemetryEventDiscriminants::ArtifactUpload => {
                "Uploaded an artifact from the Zterm CLI"
            }
            CliTelemetryEventDiscriminants::ArtifactGet => {
                "Got artifact metadata from the Zterm CLI"
            }
            CliTelemetryEventDiscriminants::ArtifactDownload => {
                "Downloaded an artifact from the Zterm CLI"
            }
            CliTelemetryEventDiscriminants::ScheduleCreate => {
                "Created a scheduled agent from the Zterm CLI"
            }
            CliTelemetryEventDiscriminants::ScheduleList => {
                "Listed scheduled agents from the Zterm CLI"
            }
            CliTelemetryEventDiscriminants::ScheduleGet => {
                "Got scheduled agent configuration from the Zterm CLI"
            }
            CliTelemetryEventDiscriminants::SchedulePause => {
                "Paused a scheduled agent from the Zterm CLI"
            }
            CliTelemetryEventDiscriminants::ScheduleUnpause => {
                "Unpaused a scheduled agent from the Zterm CLI"
            }
            CliTelemetryEventDiscriminants::ScheduleUpdate => {
                "Updated a scheduled agent from the Zterm CLI"
            }
            CliTelemetryEventDiscriminants::ScheduleDelete => {
                "Deleted a scheduled agent from the Zterm CLI"
            }
            CliTelemetryEventDiscriminants::SecretCreate => "Created a secret from the Zterm CLI",
            CliTelemetryEventDiscriminants::SecretDelete => "Deleted a secret from the Zterm CLI",
            CliTelemetryEventDiscriminants::SecretUpdate => "Updated a secret from the Zterm CLI",
            CliTelemetryEventDiscriminants::SecretList => "Listed secrets from the Zterm CLI",
            CliTelemetryEventDiscriminants::FederateIssueToken => {
                "Issued a federated identity token from the Zterm CLI"
            }
            CliTelemetryEventDiscriminants::FederateIssueGcpToken => {
                "Issued a GCP federated identity token from the Zterm CLI"
            }
            CliTelemetryEventDiscriminants::HarnessSupportPing => {
                "Pinged harness-support from the Zterm CLI"
            }
            CliTelemetryEventDiscriminants::HarnessSupportReportArtifact => {
                "Reported an artifact via harness-support from the Zterm CLI"
            }
            CliTelemetryEventDiscriminants::HarnessSupportNotifyUser => {
                "Sent a user notification via harness-support from the Zterm CLI"
            }
            CliTelemetryEventDiscriminants::HarnessSupportFinishTask => {
                "Reported task completion via harness-support from the Zterm CLI"
            }
        }
    }

    fn enablement_state(&self) -> EnablementState {
        match self {
            Self::FederateIssueToken | Self::FederateIssueGcpToken => {
                EnablementState::Flag(FeatureFlag::OzIdentityFederation)
            }
            Self::HarnessSupportPing
            | Self::HarnessSupportReportArtifact
            | Self::HarnessSupportNotifyUser
            | Self::HarnessSupportFinishTask => EnablementState::Flag(FeatureFlag::AgentHarness),
            Self::ArtifactUpload | Self::ArtifactGet | Self::ArtifactDownload => {
                EnablementState::Flag(FeatureFlag::ArtifactCommand)
            }
            Self::RunMessageWatch
            | Self::RunMessageSend
            | Self::RunMessageList
            | Self::RunMessageRead
            | Self::RunMessageMarkDelivered => EnablementState::Flag(FeatureFlag::OrchestrationV2),
            _ => EnablementState::Always,
        }
    }
}

zterm_core::register_telemetry_event!(CliTelemetryEvent);
