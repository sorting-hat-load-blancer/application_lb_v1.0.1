
// define server
pub struct Server {
    server_ip: String,
    server_port: u16
}

// define health check
pub struct HealthCheck {
    health_check_servers: Vec<Server>,
    health_check_time: u32  // miliseconds
}

// rate limitation
pub struct RateLimiting {
    alogrithm: String,  // tokenbucket , fixwindow , slidingwindow
}

// logical routing (API gateways)
pub struct VirtualRouting {

    // define servers and path
    path: String,
    servers: Vec<Server>
}

// configuration struct
pub struct Configuration {
    protocol: String,
    server_run_port: u16,
    upstream_servers: Vec<String>,
    health_check: HealthCheck,
    http_headers: bool,     // bool value -> http headers forward to upstream server (true -> forward header, false -> without forward)
    ssl_support: bool,
    rate_limiting: RateLimiting,
    virtual_paths: Vec<VirtualRouting>
}

impl Server {
    pub fn socket_address(&self) -> String {
        format!("{}:{}", self.server_ip, self.server_port)
    }
}
