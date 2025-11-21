
// define server
struct Server {
    server_ip: String,
    server_port: u16
}

// define health check
struct HealthCheck {
    health_check_servers: Vec<Server>,
    health_check_time: u32  // miliseconds
}

// rate limitation
struct RateLimiting {
    alogrithm: String,  // tokenbucket , fixwindow , slidingwindow
}

// logical routing (API gateways)
struct VirtualRouting {

    // define servers and path
    path: String,
    servers: Vec<Server>
}

// configuration struct
struct Configuration {
    protocol: String,
    server_run_port: u16,
    upstream_servers: Vec<String>,
    health_check: HealthCheck,
    http_headers: bool,     // bool value -> http headers forward to upstream server (true -> forward header, false -> without forward)
    ssl_support: bool,
    rate_limiting: RateLimiting,
    virtual_paths: Vec<VirtualRouting>
}

