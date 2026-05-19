use serde::Deserialize;

use crate::daemon::types::{FamiliesMode, OperationMode};

#[derive(Deserialize)]
pub struct SplitConfig {
    pub excluded_ips: Vec<(String, Option<String>)>,
    pub excluded_hosts: Vec<(String, Option<String>)>,
}

#[derive(Deserialize)]
pub struct FallbackDomain {
    pub suffix: String,
    pub description: Option<String>,
    pub dns_server: Option<String>,
}

#[derive(Deserialize)]
pub struct ConfigTime {
    pub secs: u64,
    pub nanos: u32,
}

#[derive(Deserialize)]
pub struct RegistrationScope {
    pub system: String,
}

#[derive(Deserialize)]
pub struct MasqueSettings {
    pub masque_http_version: String,
}

#[derive(Deserialize)]
pub struct EmergencyDisconnect {
    pub disconnect: bool,
    pub timestamp: String,
    pub poll_interval: ConfigTime,
    pub source: String,
}

#[derive(Deserialize)]
pub struct AnycastIp {
    pub ip: String,
}

#[derive(Deserialize)]
pub struct AnycastSplitIps {
    pub always_exclude: Vec<AnycastIp>,
    pub always_include: Vec<AnycastIp>,
}

#[derive(Deserialize)]
pub struct SpeedTestServer {
    pub throughput_base_url: String,
    pub turn_server_host_with_port: String,
    pub turn_cred_request_url: Option<String>,
}

#[derive(Deserialize)]
pub struct SpeedTestIp {
    pub ip: String,
}

#[derive(Deserialize)]
pub struct SpeedTestSettings {
    pub include_servers: SpeedTestServer,
    pub include_ips: Vec<SpeedTestIp>,
    pub exclude_servers: SpeedTestServer,
    pub exclude_ips: Vec<SpeedTestIp>,
}

#[derive(Deserialize)]
pub struct DnsSettings {
    pub doh_ips: Vec<String>,
}

#[derive(Deserialize)]
pub struct NetworkHealthThresholds {
    pub loss_rate: f32,
    pub latency_ms: u16,
}

/// Response structure for `get_app_settings` operation, contains all the application settings of the Cloudflare Warp service. This is the most complex response structure, as it contains a lot of fields with different types and some nested structures.
#[derive(Deserialize)]
pub struct GetAppSettings {
    pub always_on: bool,
    pub switch_locked: bool,
    pub auto_connect: String, // Unknown (got `null` on dump)
    pub operation_mode: OperationMode,
    pub disable_for_wifi: bool,
    pub disable_for_ethernet: bool,
    pub disable_for_networks: Vec<String>,
    pub families: FamiliesMode,
    pub gateway_id: String,
    pub dns_log_until: String,
    pub onboarding: bool,
    pub split_config: SplitConfig,
    pub fallback_domains: Vec<FallbackDomain>,
    pub disable_auto_fallback: bool,
    pub captive_portal: u16,
    pub support_url: String,
    pub organization: Option<String>,
    pub config_name: Option<String>,
    pub auth_client_id: Option<String>,
    pub auth_client_secret: Option<String>,
    pub allow_mode_switch: bool,
    pub unpause_time: Option<ConfigTime>,
    pub allow_updates: bool,
    pub allowed_to_leave: bool,
    pub profile_id: String,
    pub disable_connectivity_checks: Option<String>,
    pub doh_outside_tunnel: Option<String>,
    pub override_warp_endpoint: Option<String>,
    pub override_doh_endpoint: Option<String>,
    pub override_api_endpoint: Option<String>,
    pub override_tunnel_mtu: Option<u16>,
    pub non_primary_dns: Option<String>,
    pub warp_connector_token: Option<String>,
    pub lan_allow_minutes: Option<u64>,
    pub lan_allow_subnet_size: Option<u8>,
    pub warp_tunnel_protocol: Option<String>,
    pub qlog_log_until: Option<String>,
    pub registration_scope: RegistrationScope,
    pub pre_login_enabled: bool,
    pub masque_settings: MasqueSettings,
    pub emergency_disconnect: EmergencyDisconnect,
    pub register_interface_ip_with_dns: bool,
    pub anycast_split_ips: AnycastSplitIps,
    pub compliance_environment: String,
    pub firewall_scope: String,
    pub sccm_vpn_boundary_support: bool,
    pub post_quantum: String,
    pub speed_test_settings: SpeedTestSettings,
    pub external_emergency_signal_settings: Option<String>,
    pub enable_pmtud: bool,
    pub dns: DnsSettings,
    pub network_health_thresholds: NetworkHealthThresholds,
    pub collect_device_active_state: bool,
    pub proxy_cc_override: Option<String>,
}
