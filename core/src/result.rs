use std::{net::{IpAddr, Ipv4Addr}, time::Duration};

use serde::{Deserialize, Serialize};
use xenet::packet::ip::IpNextLevelProtocol;
use crate::{model::{NodeInfo, NodeType}, option::{CommandType, PortScanType, HostScanType}};

/// Exit status of probe
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ProbeStatus {
    /// Successfully completed
    Done,
    /// Interrupted by error
    Error,
    /// Execution time exceeds the configured timeout value
    Timeout,
}

impl ProbeStatus {
    pub fn name(&self) -> String {
        match *self {
            ProbeStatus::Done => String::from("Done"),
            ProbeStatus::Error => String::from("Error"),
            ProbeStatus::Timeout => String::from("Timeout"),
        }
    }
    pub fn from_netprobe_type(status: netprobe::result::ProbeStatusKind) -> ProbeStatus {
        match status {
            netprobe::result::ProbeStatusKind::Done => ProbeStatus::Done,
            netprobe::result::ProbeStatusKind::Error => ProbeStatus::Error,
            netprobe::result::ProbeStatusKind::Timeout => ProbeStatus::Timeout,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PingResponse {
    /// Sequence number
    pub seq: u8,
    /// IP address
    pub ip_addr: IpAddr,
    /// Host name
    pub host_name: String,
    /// Port
    pub port_number: Option<u16>,
    /// Time To Live
    pub ttl: u8,
    /// Number of hops
    pub hop: u8,
    /// Round Trip Time
    pub rtt: Duration,
    /// Status
    pub status: ProbeStatus,
    /// Protocol
    pub protocol: String,
    /// Node type
    pub node_type: NodeType,
}

impl PingResponse {
    pub fn new() -> PingResponse {
        PingResponse {
            seq: 0,
            ip_addr: IpAddr::V4(Ipv4Addr::LOCALHOST),
            host_name: String::new(),
            port_number: None,
            ttl: 0,
            hop: 0,
            rtt: Duration::new(0, 0),
            status: ProbeStatus::Done,
            protocol: String::new(),
            node_type: NodeType::Destination,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PingStat {
    /// Ping responses
    pub responses: Vec<PingResponse>,
    /// The entire ping probe time 
    pub probe_time: Duration,
    /// Transmitted packets
    pub transmitted_count: usize,
    /// Received packets
    pub received_count: usize,
    /// Minimum RTT 
    pub min: Duration,
    /// Avarage RTT 
    pub avg: Duration,
    /// Maximum RTT 
    pub max: Duration,
}

impl PingStat {
    pub fn new() -> PingStat {
        PingStat {
            responses: vec![],
            probe_time: Duration::new(0, 0),
            transmitted_count: 0,
            received_count: 0,
            min: Duration::new(0, 0),
            avg: Duration::new(0, 0),
            max: Duration::new(0, 0),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PortScanResult {
    pub probe_id: String,
    pub nodes: Vec<NodeInfo>,
    pub probe_status: ProbeStatus,
    /// start-time in RFC 3339 and ISO 8601 date and time string
    pub start_time: String,
    /// end-time in RFC 3339 and ISO 8601 date and time string
    pub end_time: String,
    /// Elapsed time in milliseconds
    pub elapsed_time: Duration,
    pub protocol: IpNextLevelProtocol,
    pub command_type: CommandType,
    pub scan_type: PortScanType,
    pub issued_at: String,
}

impl PortScanResult {
    pub fn new() -> PortScanResult {
        PortScanResult {
            probe_id: String::new(),
            nodes: Vec::new(),
            probe_status: ProbeStatus::Done,
            start_time: String::new(),
            end_time: String::new(),
            elapsed_time: Duration::new(0, 0),
            protocol: IpNextLevelProtocol::Tcp,
            command_type: CommandType::PortScan,
            scan_type: PortScanType::TcpSynScan,
            issued_at: String::new(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HostScanResult {
    pub probe_id: String,
    pub nodes: Vec<NodeInfo>,
    pub probe_status: ProbeStatus,
    /// start-time in RFC 3339 and ISO 8601 date and time string
    pub start_time: String,
    /// end-time in RFC 3339 and ISO 8601 date and time string
    pub end_time: String,
    /// Elapsed time in milliseconds
    pub elapsed_time: Duration,
    pub protocol: IpNextLevelProtocol,
    pub command_type: CommandType,
    pub scan_type: HostScanType,
    pub issued_at: String,
}

impl HostScanResult {
    pub fn new() -> HostScanResult {
        HostScanResult {
            probe_id: String::new(),
            nodes: Vec::new(),
            probe_status: ProbeStatus::Done,
            start_time: String::new(),
            end_time: String::new(),
            elapsed_time: Duration::new(0, 0),
            protocol: IpNextLevelProtocol::Icmp,
            command_type: CommandType::HostScan,
            scan_type: HostScanType::IcmpPingScan,
            issued_at: String::new(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PingResult {
    pub probe_id: String,
    pub stat: PingStat,
    pub probe_status: ProbeStatus,
    /// start-time in RFC 3339 and ISO 8601 date and time string
    pub start_time: String,
    /// end-time in RFC 3339 and ISO 8601 date and time string
    pub end_time: String,
    /// Elapsed time
    pub elapsed_time: Duration,
    pub protocol: IpNextLevelProtocol,
    pub command_type: CommandType,
    pub issued_at: String,
}

impl PingResult {
    pub fn new() -> PingResult {
        PingResult {
            probe_id: String::new(),
            stat: PingStat::new(),
            probe_status: ProbeStatus::Done,
            start_time: String::new(),
            end_time: String::new(),
            elapsed_time: Duration::new(0, 0),
            protocol: IpNextLevelProtocol::Icmp,
            command_type: CommandType::Ping,
            issued_at: String::new(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TracerouteResult {
    pub probe_id: String,
    pub nodes: Vec<PingResponse>,
    pub probe_status: ProbeStatus,
    /// start-time in RFC 3339 and ISO 8601 date and time string
    pub start_time: String,
    /// end-time in RFC 3339 and ISO 8601 date and time string
    pub end_time: String,
    /// Elapsed time in milliseconds
    pub elapsed_time: Duration,
    pub protocol: IpNextLevelProtocol,
    pub command_type: CommandType,
    pub issued_at: String,
}

impl TracerouteResult {
    pub fn new() -> TracerouteResult {
        TracerouteResult {
            probe_id: String::new(),
            nodes: Vec::new(),
            probe_status: ProbeStatus::Done,
            start_time: String::new(),
            end_time: String::new(),
            elapsed_time: Duration::new(0, 0),
            protocol: IpNextLevelProtocol::Udp,
            command_type: CommandType::Traceroute,
            issued_at: String::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Domain {
    pub domain_name: String,
    pub ips: Vec<IpAddr>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DomainScanResult {
    pub probe_id: String,
    pub base_domain: String,
    pub domains: Vec<Domain>,
    pub probe_status: ProbeStatus,
    /// start-time in RFC 3339 and ISO 8601 date and time string
    pub start_time: String,
    /// end-time in RFC 3339 and ISO 8601 date and time string
    pub end_time: String,
    /// Elapsed time in milliseconds
    pub elapsed_time: Duration,
    pub protocol: IpNextLevelProtocol,
    pub command_type: CommandType,
    pub issued_at: String,
}

impl DomainScanResult {
    pub fn new() -> DomainScanResult {
        DomainScanResult {
            probe_id: String::new(),
            base_domain: String::new(),
            domains: Vec::new(),
            probe_status: ProbeStatus::Done,
            start_time: String::new(),
            end_time: String::new(),
            elapsed_time: Duration::new(0, 0),
            protocol: IpNextLevelProtocol::Udp,
            command_type: CommandType::DomainScan,
            issued_at: String::new(),
        }
    }
}
