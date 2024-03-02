mod shodan;
mod helpers;

fn main() {
    let file_path = "YOUR_ABSOLUTE_PATH"; // Replace with your actual Shodan API key file path

    let mut shodan_api_key = String::new(); // Creating globally our shodan_api_key string variable

     // Use the read_file_content function from helpers module
    match helpers::read_api_key(file_path) {
        Ok(content) => {
            println!("[+] We found your Shodan API key => {}", content);
            shodan_api_key = content.clone();
        }
        Err(err) => {
            eprintln!("[-] Error reading file: {}", err);
        }
    }

    // REST API (https://developer.shodan.io/api)
    // Search Methods
    let ip_to_query = "8.8.8.8"; 

    println!("\nRust implementation of Shodan Search Methods REST API:\n[+] (GET /shodan/host/ip) Function currently being runned: get_host_ip()");
    helpers::handle_error_shodan(shodan::get_host_ip(ip_to_query, &shodan_api_key)); 

    let mut query = "port:22";
    let mut facets = "org,os";

    println!("\n[+] (GET /shodan/host/count) Function currently being runned: get_host_count()");
    helpers::handle_error_shodan(shodan::get_host_count(&shodan_api_key, query, facets)); 

    query = "product:nginx";
    facets = "country";

    println!("\n[+] (GET/shodan/host/search) Function currently being runned: get_host_search()");
    helpers::handle_error_shodan(shodan::get_host_search(&shodan_api_key, query, facets)); 

    println!("\n[+] (GET /shodan/host/search/facets) Function currently being runned: get_facets_list()");
    helpers::handle_error_shodan(shodan::get_facets_list(&shodan_api_key));

    println!("\n[+] (GET /shodan/host/search/filters) Function currently being runned: get_filters_list()");
    helpers::handle_error_shodan(shodan::get_filters_list(&shodan_api_key));

    query = "Raspbian port:22";

    println!("\n[+] (GET /shodan/host/search/tokens) Function currently being runned: get_host_search_tokens()");
    helpers::handle_error_shodan(shodan::get_host_search_tokens(&shodan_api_key, query));

    println!("\n--------------------------------------------------");

    // On-Demand Scanning
    println!("\nRust implementation of Shodan On-Demand Scanning REST API:\n[+] (GET /shodan/ports) Function currently being runned: get_ports()");
    helpers::handle_error_shodan(shodan::get_ports(&shodan_api_key));

    println!("\n[+] (GET /shodan/protocols) Function currently being runned: get_protocols()");
    helpers::handle_error_shodan(shodan::get_protocols(&shodan_api_key));

    // For request_scan() => EITHER:  
    // ips: [String] A comma-separated list of IPs or netblocks (in CIDR notation) that should get crawled.
    // let ips_to_scan = "8.8.8.8,1.1.1.1";
    
    // OR

    // ips: [String] A comma-separated list of IPs or netblocks (in CIDR notation) that should get crawled.
    // service: [Array] A list of services that should get scanned, where a service is defined as a [port, protocol].
    let ips_with_ports = r#"
    {
    "8.8.8.8": [
        [53, "dns-udp"],
        [443, "https"]
    ],
    "1.1.1.1": [
        [80, "http"],
        [443, "https"]
    ]
    }"#;

    println!("\n[+] (POST /shodan/scan) Function currently being runned: create_scan()");
    helpers::handle_error_shodan(shodan::create_scan(&shodan_api_key, ips_with_ports));

    println!("\n[+] (GET /shodan/scans) Function currently being runned: get_scans()");
    helpers::handle_error_shodan(shodan::get_scans(&shodan_api_key));  

    let scan_id = "YOUR_SCAN_ID"; 
    println!("\n[+] (GET /shodan/scan/id) Function currently being runned: get_scan_id()");
    helpers::handle_error_shodan(shodan::get_scan_id(scan_id, &shodan_api_key));  

    println!("\n--------------------------------------------------");

    // Network Alerts
    let name = "DNS Alert";
    let ips = vec!["8.8.8.8", "1.1.1.1"];
    let expires = 0;
    println!("\nRust implementation of Shodan Network Alerts REST API:\n[+] (POST /shodan/alert) Function currently being runned: create_alert()");
    helpers::handle_error_shodan(shodan::create_alert(&shodan_api_key, name, ips, expires)); 
  
    let alert_id = "YOUR_ALERT_ID";
    println!("\n[+] (GET /shodan/alert/id/info) Function currently being runned: get_alert_info_id()");
    helpers::handle_error_shodan(shodan::get_alert_info_id(alert_id, &shodan_api_key));   

    let alert_id = "YOUR_ALERT_ID";
    println!("\n[+] (DELETE /shodan/alert/id) Function currently being runned: delete_alert()");
    helpers::handle_error_shodan(shodan::delete_alert(alert_id, &shodan_api_key));

    let alert_id = "YOUR_ALERT_ID";
    let ips = vec!["1.1.1.1", "8.8.8.8"];
    println!("\n[+] (POST /shodan/alert/id) Function currently being runned: edit_alert()");
    helpers::handle_error_shodan(shodan::edit_alert(alert_id, &shodan_api_key, ips));

    println!("\n[+] (GET /shodan/alert/info) Function currently being runned: get_alert_info()");
    helpers::handle_error_shodan(shodan::get_alert_info(&shodan_api_key));

    println!("\n[+] (GET /shodan/alert/triggers) Function currently being runned: get_alert_triggers()");
    helpers::handle_error_shodan(shodan::get_alert_triggers(&shodan_api_key));

    let alert_id = "YOUR_ALERT_ID";
    let trigger = "new_service,vulnerable";
    println!("\n[+] (PUT/shodan/alert/id/trigger/trigger) Function currently being runned: add_trigger()");
    helpers::handle_error_shodan(shodan::add_trigger(alert_id, trigger, &shodan_api_key));

    let alert_id = "YOUR_ALERT_ID";
    let trigger = "new_service,vulnerable";
    println!("\n[+] (DELETE /shodan/alert/id/trigger/trigger) Function currently being runned: delete_trigger()");
    helpers::handle_error_shodan(shodan::delete_trigger(alert_id, trigger, &shodan_api_key));

    let alert_id = "YOUR_ALERT_ID";
    let trigger = "new_service";
    let service = "1.1.1.1:53";
    println!("\n[+] (PUT /shodan/alert/id/trigger/trigger/ignore/service) Function currently being runned: add_whitelist()");
    helpers::handle_error_shodan(shodan::add_whitelist(alert_id, trigger, service, &shodan_api_key));

    let alert_id = "YOUR_ALERT_ID";
    let trigger = "new_service";
    let service = "1.1.1.1:53";
    println!("\n[+] (DELETE /shodan/alert/id/trigger/trigger/ignore/service) Function currently being runned: delete_whitelist()");
    helpers::handle_error_shodan(shodan::delete_whitelist(alert_id, trigger, service, &shodan_api_key));

    let alert_id = "YOUR_ALERT_ID";
    let notifier_id = "default";
    println!("\n[+] (PUT /shodan/alert/id/notifier/notifier_id) Function currently being runned: add_notifier_alert()");
    helpers::handle_error_shodan(shodan::add_notifier_alert(alert_id, notifier_id, &shodan_api_key));

    let alert_id = "YOUR_ALERT_ID";
    let notifier_id = "default";
    println!("\n[+] (DELETE /shodan/alert/id/notifier/notifier_id) Function currently being runned: delete_notifier_alert()");
    helpers::handle_error_shodan(shodan::delete_notifier_alert(alert_id, notifier_id, &shodan_api_key));

    println!("\n--------------------------------------------------");
    
    // Notifiers
    println!("\nRust implementation of Shodan Notifiers REST API:\n[+] (GET /notifier) Function currently being runned: get_notifier()");
    helpers::handle_error_shodan(shodan::get_notifier(&shodan_api_key)); 

    println!("\n[+] (GET /notifier/provider) Function currently being runned: get_notifier_provider()");
    helpers::handle_error_shodan(shodan::get_notifier_provider(&shodan_api_key)); 

    let provider = "email";
    let description = "Email notifier";
    let argument = "to";
    let argument_value = "jmath@shodan.io";
    println!("\n[+] (POST /notifier) Function currently being runned: create_notifier()");
    helpers::handle_error_shodan(shodan::create_notifier(&shodan_api_key, provider, description, argument, argument_value)); 
 
    let notifier_id = "YOUR_NOTIFIER_ID";
    println!("\n[+] (DELETE /notifier/id) Function currently being runned: delete_notifier()");
    helpers::handle_error_shodan(shodan::delete_notifier(notifier_id, &shodan_api_key)); 

    let notifier_id = "YOUR_NOTIFIER_ID";
    println!("\n[+] (GET /notifier/id) Function currently being runned: get_notifier_info()");
    helpers::handle_error_shodan(shodan::get_notifier_info(notifier_id, &shodan_api_key));

    let notifier_id = "YOUR_NOTIFIER_ID";
    let argument = "to";
    let argument_value = "jmath@shodan.io";
    println!("\n[+] (PUT /notifier/id) Function currently being runned: edit_notifier()");
    helpers::handle_error_shodan(shodan::edit_notifier(notifier_id, &shodan_api_key, argument, argument_value));

    println!("\n--------------------------------------------------");

    // Directory Methods

    // Check get_query(), possibility to edit / remove these three parameters
    let page = "";
    let sort = "votes"; 
    let order = "asc";

    println!("\nRust implementation of Shodan Directory Methods REST API:\n[+] (GET /shodan/query) Function currently being runned: get_query()");
    helpers::handle_error_shodan(shodan::get_query(page, sort, order, &shodan_api_key));
  
    let query = "webcam";
    let page = ""; // Check get_query_search(), possibility to edit / remove this parameter
    println!("\n[+] (GET /shodan/query/search) Function currently being runned: get_query_search()");
    helpers::handle_error_shodan(shodan::get_query_search(query, page, &shodan_api_key));
    
    let size = "10"; // Check get_query_tags(), possibility to edit / remove this parameter
    println!("\n[+] (GET /shodan/query/tags) Function currently being runned: get_query_tags()");
    helpers::handle_error_shodan(shodan::get_query_tags(size, &shodan_api_key));
  
    println!("\n--------------------------------------------------");

    // Account Methods
    println!("\nRust implementation of Shodan Account Methods REST API:\n[+] (GET /account/profile) Function currently being runned: get_account_profile()");
    helpers::handle_error_shodan(shodan::get_account_profile(&shodan_api_key));

    println!("\n--------------------------------------------------");

    // DNS Methods
    let domain = "google.com";

    // Check get_dns_domain(), possibility to edit / remove these three parameters
    let history = "False";
    let type_ = "A";
    let page = "";

    println!("\nRust implementation of Shodan DNS Methods REST API:\n[+] (GET /dns/domain/domain) Function currently being runned: get_dns_domain()");
    helpers::handle_error_shodan(shodan::get_dns_domain(domain, history, type_, page, &shodan_api_key));

    let hostnames = "google.com,facebook.com";
    println!("\n[+] (GET /dns/resolve) Function currently being runned: get_dns_resolve()");
    helpers::handle_error_shodan(shodan::get_dns_resolve(hostnames, &shodan_api_key));

    let ips = "8.8.8.8,1.1.1.1";
    println!("\n[+] (GET /dns/reverse) Function currently being runned: get_dns_reverse()");
    helpers::handle_error_shodan(shodan::get_dns_reverse(ips, &shodan_api_key));

    println!("\n--------------------------------------------------");

    // Utility Methods
    println!("\nRust implementation of Shodan Utility Methods REST API:\n[+] (GET /tools/httpheaders) Function currently being runned: get_tools_httpheaders()");
    helpers::handle_error_shodan(shodan::get_tools_headers(&shodan_api_key));

    println!("\n[+] (GET /tools/myip) Function currently being runned: get_tools_myip()");
    helpers::handle_error_shodan(shodan::get_tools_myip(&shodan_api_key));

    println!("\n--------------------------------------------------");

    // API Status Methods
    println!("\nRust implementation of Shodan API Status Methods REST API:\n[+] (GET /api-info) Function currently being runned: get_api_info()");
    helpers::handle_error_shodan(shodan::get_api_info(&shodan_api_key));

    println!("\n--------------------------------------------------");

    /* 
        STREAMING API (https://developer.shodan.io/api/stream)
        Network Alerts
        These streams provide a Private Firehose that contain information about the networks that you're monitoring. 
        Use the REST API to create/ delete/ update the list of networks that you want Shodan to monitor.
    */
    println!("\nRust implementation of Shodan Network Alerts Streaming API:\n[+] (GET /shodan/alert) Function currently being runned: get_all_network_alerts()");
    helpers::handle_error_shodan(shodan::get_all_network_alerts(&shodan_api_key));
    
    let alert_id = "YOUR_ALERT_ID";
    println!("\n[+] (GET /shodan/alert/id) Function currently being runned: get_all_network_alert_for_alert_id()");
    helpers::handle_error_shodan(shodan::get_all_network_alert_for_alert_id(alert_id, &shodan_api_key));

}

