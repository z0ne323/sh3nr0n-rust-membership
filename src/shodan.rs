use serde_json::json;
use reqwest::blocking::Client;
use reqwest::blocking::Response;
use reqwest::Error as ReqwestError;

pub fn get_host_ip(ip: &str, api_key: &str) -> Result<Response, ReqwestError> {
    /*
        Description:
            Host Information - Returns all services that have been found on the given host IP. 
            (GET/shodan/host/{ip})   
        Parameters:
            ip (&str): Host IP address
            api_key (&str): Shodan API Key
        Returns:
            Result<Response, ReqwestError>: 
                Ok(Response): Returns the result of the GET request containing the response.
                Err(ReqwestError): Returns an error if the GET request fails.
    */
    let client = Client::new();
    let url = format!("https://api.shodan.io/shodan/host/{}?key={}", ip, api_key);
    let response = client.get(&url).send()?;
    Ok(response)
}

pub fn get_host_count(api_key: &str, query: &str, facets: &str) -> Result<Response, ReqwestError> {
    /*
        Description:
            Search Shodan without Results
            This method behaves identical to "/shodan/host/search" with the only difference that this method does not return any host results, 
            it only returns the total number of results that matched the query and any facet information that was requested. 
            As a result this method does not consume query credits.
            (GET /shodan/host/count)
        Parameters:
            api_key (&str): Shodan API Key
            query (&str): Shodan search query. The provided string is used to search the database of banners in Shodan, 
            with the additional option to provide filters inside the search query using a "filter:value" format. 
            For example, the following search query would find Apache Web servers located in Germany: "apache country:DE". (https://beta.shodan.io/search/filters)
            facets (&str):  A comma-separated list of properties to get summary information on. Property names can also be in the format of "property:count", 
            where "count" is the number of facets that will be returned for a property (i.e. "country:100" to get the top 100 countries for a search query). 
            Visit the Shodan website's Facet Analysis page for an up-to-date list of available facets: https://beta.shodan.io/search/facet
        Returns:
            Result<Response, ReqwestError>: 
                Ok(Response): Returns the result of the GET request containing the response.
                Err(ReqwestError): Returns an error if the GET request fails.
    */
    let client = Client::new();
    let url = format!("https://api.shodan.io/shodan/host/count?key={}&query={}&facets={}", api_key, query, facets);
    let response = client.get(&url).send()?;
    Ok(response)
}

pub fn get_host_search(api_key: &str, query: &str, facets: &str) -> Result<Response, ReqwestError> {
    /*
        Description:
            Search Shodan
            Search Shodan using the same query syntax as the website and use facets to get summary information for different properties.
            (GET /shodan/host/search)
        Parameters:
            api_key (&str): Shodan API Key
            query (&str): Shodan search query. The provided string is used to search the database of banners in Shodan, 
            with the additional option to provide filters inside the search query using a "filter:value" format. 
            For example, the following search query would find Apache Web servers located in Germany: "apache country:DE". (https://beta.shodan.io/search/filters)
            facets (&str):  A comma-separated list of properties to get summary information on. Property names can also be in the format of "property:count", 
            where "count" is the number of facets that will be returned for a property (i.e. "country:100" to get the top 100 countries for a search query). 
            Visit the Shodan website's Facet Analysis page for an up-to-date list of available facets: https://beta.shodan.io/search/facet
        Returns:
            Result<Response, ReqwestError>: 
                Ok(Response): Returns the result of the GET request containing the response.
                Err(ReqwestError): Returns an error if the GET request fails.
    */
    let client = Client::new();
    let url = format!("https://api.shodan.io/shodan/host/search?key={}&query={}&facets={}", api_key, query, facets);
    let response = client.get(&url).send()?;
    Ok(response)
}

pub fn get_facets_list(api_key: &str) -> Result<Response, ReqwestError> {
    /*
        Description:
            List all search facets - This method returns a list of facets that can be used to get a breakdown of the top values for a property.
            (GET /shodan/host/search/facets)
        Parameters:
            api_key (&str): Shodan API Key
        Returns:
            Result<Response, ReqwestError>: 
                Ok(Response): Returns the result of the GET request containing the response.
                Err(ReqwestError): Returns an error if the GET request fails.
    */
    let client = Client::new();
    let url = format!("https://api.shodan.io/shodan/host/search/facets?key={}", api_key);
    let response = client.get(&url).send()?;
    Ok(response)
}

pub fn get_filters_list(api_key: &str) -> Result<Response, ReqwestError> {
    /*
        Description:
            List all filters that can be used when searching - This method returns a list of search filters that can be used in the search query.
            (GET /shodan/host/search/filters)
        Parameters:
            api_key (&str): Shodan API Key
        Returns:
            Result<Response, ReqwestError>: 
                Ok(Response): Returns the result of the GET request containing the response.
                Err(ReqwestError): Returns an error if the GET request fails.
    */
    let client = Client::new();
    let url = format!("https://api.shodan.io/shodan/host/search/filters?key={}", api_key);
    let response = client.get(&url).send()?;
    Ok(response)
}

pub fn get_host_search_tokens(api_key: &str, query: &str) -> Result<Response, ReqwestError> {
    /*
        Description:
            Break the search query into tokens - This method lets you determine which filters are being used by the query string and what parameters were provided to the filters.
            (GET /shodan/host/search/tokens)
        Parameters:
            api_key (&str): Shodan API Key
            query (&str): Shodan search query. The provided string is used to search the database of banners in Shodan, 
            with the additional option to provide filters inside the search query using a "filter:value" format. 
            For example, the following search query would find Apache Web servers located in Germany: "apache country:DE". (https://beta.shodan.io/search/filters)
        Returns:
            Result<Response, ReqwestError>: 
                Ok(Response): Returns the result of the GET request containing the response.
                Err(ReqwestError): Returns an error if the GET request fails.
    */
    let client = Client::new();
    let url = format!("https://api.shodan.io/shodan/host/search/tokens?key={}&query={}", api_key, query);
    let response = client.get(&url).send()?;
    Ok(response)
}

pub fn get_ports(api_key: &str) -> Result<Response, ReqwestError> {
    /*
        Description:
            List all ports that Shodan is crawling on the Internet. - This method returns a list of port numbers that the crawlers are looking for.
            (GET /shodan/ports)
        Parameters:
            api_key (&str): Shodan API Key
        Returns:
            Result<Response, ReqwestError>: 
                Ok(Response): Returns the result of the GET request containing the response.
                Err(ReqwestError): Returns an error if the GET request fails.
    */
    let client = Client::new();
    let url = format!("https://api.shodan.io/shodan/ports?key={}", api_key);
    let response = client.get(&url).send()?;
    Ok(response)
}

pub fn get_protocols(api_key: &str) -> Result<Response, ReqwestError> {
    /*
        Description:
            List all protocols that can be used when performing on-demand Internet scans via Shodan.
            This method returns an object containing all the protocols that can be used when launching an Internet scan.
            (GET /shodan/protocols)
        Parameters:
            api_key (&str): Shodan API Key
        Returns:
            Result<Response, ReqwestError>: 
                Ok(Response): Returns the result of the GET request containing the response.
                Err(ReqwestError): Returns an error if the GET request fails.
    */
    let client = Client::new();
    let url = format!("https://api.shodan.io/shodan/protocols?key={}", api_key);
    let response = client.get(&url).send()?;
    Ok(response)
}

pub fn create_scan(api_key: &str, ips_or_ips_and_services: &str) -> Result<Response, ReqwestError> {
    /*
        Description:
            Request Shodan to crawl an IP/ netblock - Use this method to request Shodan to crawl a network.
            WARNING ! -> This method uses API scan credits: 1 IP consumes 1 scan credit. You must have a paid API plan (either one-time payment or subscription) in order to use this method.
            (POST /shodan/scan)
        Parameters:
            api_key (&str): Shodan API Key
            ips_or_ips_and_services (&str): A string containing either single, multiple IP addresses or a JSON-formatted string representing a list of IPs with optional specified services.
        Returns:
            Result<Response, ReqwestError>: 
                Ok(Response): Returns the result of the POST request containing the response.
                Err(ReqwestError): Returns an error if the POST request fails.
    */
    let client = Client::new();
    let url = format!("https://api.shodan.io/shodan/scan?key={}", api_key);
    let response = client.post(url).form(&[("ips", ips_or_ips_and_services)]).send()?;
    Ok(response)
}

pub fn get_scans(api_key: &str) -> Result<Response, ReqwestError> {
    /*
        Description:
            Get list of all the created scans
            Returns a listing of all the on-demand scans that are currently active on the account.
            (GET /shodan/scans)
        Parameters:
            api_key (&str): Shodan API Key
        Returns:
            Result<Response, ReqwestError>: 
                Ok(Response): Returns the result of the GET request containing the response.
                Err(ReqwestError): Returns an error if the GET request fails.
    */
    let client = Client::new();
    let url = format!("https://api.shodan.io/shodan/scans?key={}", api_key);
    let response = client.get(&url).send()?;
    Ok(response)
}

pub fn get_scan_id(scan_id: &str, api_key: &str) -> Result<Response, ReqwestError> {
    /*
        Description:
            Get the status of a scan request
            Check the progress of a previously submitted scan request. Possible values for the status are:
                - SUBMITTING
                - QUEUE   
                - PROCESSING
                - DONE
            (GET /shodan/scan/{id})
        Parameters:
            id (&str): The unique scan ID that was returned by /shodan/scan.
            api_key (&str): Shodan API Key
        Returns:
            Result<Response, ReqwestError>: 
                Ok(Response): Returns the result of the GET request containing the response.
                Err(ReqwestError): Returns an error if the GET request fails.
    */
    let client = Client::new();
    let url = format!("https://api.shodan.io/shodan/scan/{}?key={}", scan_id, api_key);
    let response = client.get(&url).send()?;
    Ok(response)
}

pub fn create_alert(api_key: &str, name: &str, ips: Vec<&str>, expires: i32) -> Result<Response, reqwest::Error> {
    /*
        Description:
            Create an alert to monitor a network range
            Use this method to create a network alert for a defined IP/ netblock which can be used to subscribe to changes/ events that are discovered within that range. 
            (POST /shodan/alert)
        Parameters:
            api_key (&str): Shodan API Key
            name (&str): The name to describe the network alert.
            ips (Vec<&str>): An object specifying the criteria that an alert should trigger. The only supported option at the moment is the "ip" filter. -  A list of IPs or network ranges defined using CIDR notation.
            expires (i32): Number of seconds that the alert should be active.
        Returns:
            Result<Response, Box<dyn StdError>>: 
                Ok(Response): Returns the result of the POST request containing the response.
                Err(Box<dyn StdError>): Returns an error if the POST request fails.
    */
    let client = Client::new();
    let url = format!("https://api.shodan.io/shodan/alert?key={}", api_key);

    let data_received = json!({
        "name": name,
        "filters": {
            "ip": ips
        },
        "expires": expires
    });

    let alert_data = serde_json::to_string(&data_received).unwrap_or_else(|err| {
        eprintln!("Error converting JSON: {}", err);
        String::new()
    });

    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .body(alert_data.into_bytes())
        .send()?;

    Ok(response)
}

pub fn get_alert_info_id(alert_id: &str, api_key: &str) -> Result<Response, ReqwestError> {
    /*
        Description:
            Get the details for a network alert
            Returns the information about a specific network alert.
            (GET /shodan/alert/{id}/info)
        Parameters:
            alert_id (&str): The unique ID that was returned by /shodan/alert.
            api_key (&str): Shodan API Key
        Returns:
            Result<Response, ReqwestError>: 
                Ok(Response): Returns the result of the GET request containing the response.
                Err(ReqwestError): Returns an error if the GET request fails.
    */
    let client = Client::new();
    let url = format!("https://api.shodan.io/shodan/alert/{}/info?key={}", alert_id, api_key);
    let response = client.get(&url).send()?;
    Ok(response)
}

pub fn delete_alert(alert_id: &str, api_key: &str) -> Result<Response, reqwest::Error> {
    /*
        Description:
            Delete an alert
            Remove the specified network alert.
            (DELETE /shodan/alert/{id})
        Parameters:
            alert_id (&str): The unique ID that was returned by /shodan/alert.
            api_key (&str): Shodan API Key
        Returns:
            Result<Response, ReqwestError>: 
                Ok(Response): Returns the result of the DELETE request containing the response.
                Err(ReqwestError): Returns an error if the DELETE request fails.
    */
    let url = format!("https://api.shodan.io/shodan/alert/{}?key={}", alert_id, api_key);
    let client = Client::new();
    let response = client.delete(&url).send()?;
    Ok(response)
}


pub fn edit_alert(alert_id: &str, api_key: &str, ips: Vec<&str>) -> Result<Response, reqwest::Error> {
    /*
        Description:
            Edit the networks monitored in an alert
            Use this method to edit a network alert with a new list of IPs/ networks to keep track of. 
            (POST /shodan/alert/{id})
        Parameters:
            alert_id (&str): The unique ID that was returned by /shodan/alert
            api_key (&str): Shodan API Key
            ips (Vec<&str>): An object specifying the criteria that an alert should trigger. The only supported option at the moment is the "ip" filter. -  A list of IPs or network ranges defined using CIDR notation.
        Returns:
            Result<Response, Box<dyn StdError>>: 
                Ok(Response): Returns the result of the POST request containing the response.
                Err(Box<dyn StdError>): Returns an error if the POST request fails.
    */
    let client = Client::new();
    let url = format!("https://api.shodan.io/shodan/alert/{}?key={}", alert_id, api_key);

    let data_received = json!({
        "filters": {
            "ip": ips
        },
    });

    let alert_data = serde_json::to_string(&data_received).unwrap_or_else(|err| {
        eprintln!("Error converting JSON: {}", err);
        String::new()
    });

    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .body(alert_data.into_bytes())
        .send()?;

    Ok(response)
}

pub fn get_alert_info(api_key: &str) -> Result<Response, ReqwestError> {
    /*
        Description:
            Get a list of all the created alerts
            Returns a listing of all the network alerts that are currently active on the account
            (GET /shodan/alert/info)
        Parameters:
            api_key (&str): Shodan API Key
        Returns:
            Result<Response, ReqwestError>: 
                Ok(Response): Returns the result of the GET request containing the response.
                Err(ReqwestError): Returns an error if the GET request fails.
    */
    let client = Client::new();
    let url = format!("https://api.shodan.io/shodan/alert/info?key={}", api_key);
    let response = client.get(&url).send()?;
    Ok(response)
}

pub fn get_alert_triggers(api_key: &str) -> Result<Response, ReqwestError> {
    /*
        Description:
            Get a list of available triggers
            Returns a list of all the triggers that can be enabled on network alerts.
            (GET /shodan/alert/triggers)
        Parameters:
            api_key (&str): Shodan API Key
        Returns:
            Result<Response, ReqwestError>: 
                Ok(Response): Returns the result of the GET request containing the response.
                Err(ReqwestError): Returns an error if the GET request fails.
    */
    let client = Client::new();
    let url = format!("https://api.shodan.io/shodan/alert/triggers?key={}", api_key);
    let response = client.get(&url).send()?;
    Ok(response)
}

pub fn add_trigger(alert_id: &str, trigger: &str, api_key: &str) -> Result<Response, ReqwestError> {
    /*
        Description:
            Enable a trigger
            Get notifications when the specified trigger is met.
            (PUT /shodan/alert/{id}/trigger/{trigger})
        Parameters:
            alert_id (&str): The unique ID that was returned by /shodan/alert.
            trigger (&str): Comma-separated list of trigger names
            api_key (&str): Shodan API Key
        Returns:
            Result<Response, ReqwestError>: 
                Ok(Response): Returns the result of the PUT request containing the response.
                Err(ReqwestError): Returns an error if the PUT request fails.
    */
    let client = Client::new();
    let url = format!("https://api.shodan.io/shodan/alert/{}/trigger/{}?key={}", alert_id, trigger, api_key);
    let response = client.put(&url).send()?;
    Ok(response)
}   

pub fn delete_trigger(alert_id: &str, trigger: &str, api_key: &str) -> Result<Response, ReqwestError> {
    /*
        Description:
            Disable a trigger
            Stop getting notifications for the specified trigger.
            (DELETE /shodan/alert/{id}/trigger/{trigger})
        Parameters:
            alert_id (&str): The unique ID that was returned by /shodan/alert.
            trigger (&str): Comma-separated list of trigger names
            api_key (&str): Shodan API Key
        Returns:
            Result<Response, ReqwestError>: 
                Ok(Response): Returns the result of the DELETE request containing the response.
                Err(ReqwestError): Returns an error if the DELETE request fails.
    */
    let client = Client::new();
    let url = format!("https://api.shodan.io/shodan/alert/{}/trigger/{}?key={}", alert_id, trigger, api_key);
    let response = client.delete(&url).send()?;
    Ok(response)
}   

pub fn add_whitelist(alert_id: &str, trigger: &str, service: &str, api_key: &str) -> Result<Response, ReqwestError> {
    /*
        Description:
            Add to Whitelist
            Ignore the specified service when it is matched for the trigger.
            (PUT /shodan/alert/{id}/trigger/{trigger}/ignore/{service})
        Parameters:
            alert_id (&str): The unique ID that was returned by /shodan/alert.
            trigger (&str): Trigger name
            service (&str): Service specified in the format "ip:port" (ex. "1.1.1.1:80")
            api_key (&str): Shodan API Key
        Returns:
            Result<Response, ReqwestError>: 
                Ok(Response): Returns the result of the PUT request containing the response.
                Err(ReqwestError): Returns an error if the PUT request fails.
    */
    let client = Client::new();
    let url = format!("https://api.shodan.io/shodan/alert/{}/trigger/{}/ignore/{}?key={}", alert_id, trigger, service, api_key);
    let response = client.put(&url).send()?;
    Ok(response)
}   

pub fn delete_whitelist(alert_id: &str, trigger: &str, service: &str, api_key: &str) -> Result<Response, ReqwestError> {
    /*
        Description:
            Remove from Whitelist
            Start getting notifications again for the specified trigger.
            (DELETE /shodan/alert/{id}/trigger/{trigger}/ignore/{service})
        Parameters:
            alert_id (&str): The unique ID that was returned by /shodan/alert.
            trigger (&str): Trigger name
            service (&str): Service specified in the format "ip:port" (ex. "1.1.1.1:80")
            api_key (&str): Shodan API Key
        Returns:
            Result<Response, ReqwestError>: 
                Ok(Response): Returns the result of the DELETE request containing the response.
                Err(ReqwestError): Returns an error if the DELETE request fails.
    */
    let client = Client::new();
    let url = format!("https://api.shodan.io/shodan/alert/{}/trigger/{}/ignore/{}?key={}", alert_id, trigger, service, api_key);
    let response = client.delete(&url).send()?;
    Ok(response)
}  

pub fn add_notifier_alert(alert_id: &str, notifier_id: &str, api_key: &str) -> Result<Response, ReqwestError> {
    /*
        Description:
            Add the notifier to the alert
            Add the specified notifier to the network alert. Notifications are only sent if triggers have also been enabled. 
            For each created user, there is a default notifier which will sent via email.
            (PUT /shodan/alert/{id}/notifier/{notifier_id})
        Parameters:
            alert_id (&str): The unique ID that was returned by /shodan/alert.
            notifier_id (&str): Notifier ID
            api_key (&str): Shodan API Key
        Returns:
            Result<Response, ReqwestError>: 
                Ok(Response): Returns the result of the PUT request containing the response.
                Err(ReqwestError): Returns an error if the PUT request fails.
    */
    let client = Client::new();
    let url = format!("https://api.shodan.io/shodan/alert/{}/notifier/{}?key={}", alert_id, notifier_id, api_key);
    let response = client.put(&url).send()?;
    Ok(response)
}  

pub fn delete_notifier_alert(alert_id: &str, notifier_id: &str, api_key: &str) -> Result<Response, ReqwestError> {
    /*
        Description:
            Remove the notifier from the alert
            Remove the notification service from the alert. Notifications are only sent if triggers have also been enabled.
            (DELETE /shodan/alert/{id}/notifier/{notifier_id})
        Parameters:
            alert_id (&str): The unique ID that was returned by /shodan/alert.
            notifier_id (&str): Notifier ID
            api_key (&str): Shodan API Key
        Returns:
            Result<Response, ReqwestError>: 
                Ok(Response): Returns the result of the DELETE request containing the response.
                Err(ReqwestError): Returns an error if the DELETE request fails.
    */
    let client = Client::new();
    let url = format!("https://api.shodan.io/shodan/alert/{}/notifier/{}?key={}", alert_id, notifier_id, api_key);
    let response = client.delete(&url).send()?;
    Ok(response)
}  

pub fn get_notifier(api_key: &str) -> Result<Response, ReqwestError> {
    /*
        Description:
            List all user-created notifiers
            Get a list of all the notifiers that the user has created.
            (GET /notifier)
        Parameters:
            api_key (&str): Shodan API Key
        Returns:
            Result<Response, ReqwestError>: 
                Ok(Response): Returns the result of the GET request containing the response.
                Err(ReqwestError): Returns an error if the GET request fails.
    */
    let client = Client::new();
    let url = format!("https://api.shodan.io/notifier?key={}", api_key);
    let response = client.get(&url).send()?;
    Ok(response)
}

pub fn get_notifier_provider(api_key: &str) -> Result<Response, ReqwestError> {
    /*
        Description:
            List of available notification providers
            Get a list of all the notification providers that are available and the parameters to submit when creating them.
            (GET /notifier/provider)
        Parameters:
            api_key (&str): Shodan API Key
        Returns:
            Result<Response, ReqwestError>: 
                Ok(Response): Returns the result of the GET request containing the response.
                Err(ReqwestError): Returns an error if the GET request fails.
    */
    let client = Client::new();
    let url = format!("https://api.shodan.io/notifier/provider?key={}", api_key);
    let response = client.get(&url).send()?;
    Ok(response)
}

pub fn create_notifier(api_key: &str, provider: &str, description: &str, argument: &str, argument_value: &str) -> Result<Response, reqwest::Error> {
    /*
        Description:
            Create a new notification service for the user
            Use this method to create a new notification service endpoint that Shodan services can send notifications through.
            (POST /notifier)
        Parameters:
            api_key (&str): Shodan API Key.
            provider (&str): Provider name as returned by /notifier/provider
            description (&str): Description of the notifier
            argument (&str): Argument required by the provider (ex: to)
            argument_value (&str): Value of the argument required by the provider (ex: jmath@shodan.io) 
        Returns:
            Result<Response, reqwest::Error>:
                Ok(Response): Returns the result of the POST request containing the response.
                Err(reqwest::Error): Returns an error if the POST request fails.
    */
    let client = Client::new();
    let url = format!("https://api.shodan.io/notifier?key={}", api_key);
    let response = client
        .post(url)
        .form(&[
            ("provider", provider),
            ("description", description),
            (argument, argument_value),
        ])
        .send()?;

    Ok(response)
}

pub fn delete_notifier(notifier_id: &str, api_key: &str) -> Result<Response, reqwest::Error> {
    /*
        Description:
           Delete a notification service
            Remove the notification service created for the user.
            (DELETE /notifier/{id})
        Parameters:
            notifier_id (&str): Notifier ID returned by (POST /notifier)
            api_key (&str): Shodan API Key
        Returns:
        Result<Response, ReqwestError>: 
            Ok(Response): Returns the result of the DELETE request containing the response.
            Err(ReqwestError): Returns an error if the DELETE request fails.
    */
    let url = format!("https://api.shodan.io/notifier/{}?key={}", notifier_id, api_key);
    let client = Client::new();
    let response = client.delete(&url).send()?;
    Ok(response)
}

pub fn get_notifier_info(notifier_id: &str, api_key: &str) -> Result<Response, ReqwestError> {
    /*
        Description:
            Get information about a notifier
            Use this method to create a new notification service endpoint that Shodan services can send notifications through.
            (GET /notifier/{id})
        Parameters:
            notifier_id (&str): Notifier ID returned by (POST /notifier)
            api_key (&str): Shodan API Key
        Returns:
            Result<Response, ReqwestError>: 
                Ok(Response): Returns the result of the GET request containing the response.
                Err(ReqwestError): Returns an error if the GET request fails.
    */
    let client = Client::new();
    let url = format!("https://api.shodan.io/notifier/{}?key={}", notifier_id, api_key);
    let response = client.get(&url).send()?;
    Ok(response)
}

pub fn edit_notifier(notifier_id: &str, api_key: &str, argument: &str, argument_value: &str) -> Result<Response, ReqwestError> {
    /*
    Description:
        Edit a notifier
        Use this method to update the parameters of a notifier.
        (PUT /notifier/{id})
    Parameters:
        notifier_id (&str): Notifier ID returned by (POST /notifier)
        api_key (&str): Shodan API Key
        argument (&str): Argument required by the provider (ex: to)
        argument_value (&str): Value of the argument required by the provider (ex: jmath@shodan.io) 
    Returns:
        Result<Response, ReqwestError>: 
            Ok(Response): Returns the result of the PUT request containing the response.
            Err(ReqwestError): Returns an error if the PUT request fails.
    */
    let client = Client::new();
    let url = format!("https://api.shodan.io/notifier/{}?key={}", notifier_id, api_key);
    let response = client.put(url).form(&[(argument, argument_value)]).send()?;
    Ok(response)
}

pub fn get_query(page: &str, sort: &str, order: &str, api_key: &str) -> Result<Response, ReqwestError> {
    /*
        Description:
            List the saved search queries
            Use this method to obtain a list of search queries that users have saved in Shodan.
            (GET /shodan/query)
        Parameters:
            page (optional) (&str): Page number to iterate over results; each page contains 10 items
            sort (optional) (&str): Sort the list based on a property. Possible values are: votes, timestamp
            order (optional) (&str): Whether to sort the list in ascending or descending order. Possible values are: asc, desc
            api_key (&str): Shodan API Key
        Returns:
            Result<Response, ReqwestError>: 
                Ok(Response): Returns the result of the GET request containing the response.
                Err(ReqwestError): Returns an error if the GET request fails.
    */
    let client = Client::new();
    let url = format!("https://api.shodan.io/shodan/query?page={}&sort={}&order={}&key={}", page, sort, order, api_key);
    let response = client.get(&url).send()?;
    Ok(response)
}

pub fn get_query_search(query: &str, page: &str, api_key: &str) -> Result<Response, ReqwestError> {
    /*
        Description:
            Search the directory of saved search queries.
            Use this method to search the directory of search queries that users have saved in Shodan.
            (GET /shodan/query/search)
        Parameters:
            query: [String] What to search for in the directory of saved search queries.
            page (optional) (&str): Page number to iterate over results; each page contains 10 items
            api_key (&str): Shodan API Key
        Returns:
            Result<Response, ReqwestError>: 
                Ok(Response): Returns the result of the GET request containing the response.
                Err(ReqwestError): Returns an error if the GET request fails.
    */
    let client = Client::new();
    let url = format!("https://api.shodan.io/shodan/query/search?query={}&page={}&key={}", query, page, api_key);
    let response = client.get(&url).send()?;
    Ok(response)
}

pub fn get_query_tags(size: &str, api_key: &str) -> Result<Response, ReqwestError> {
    /*
        Description:
            List the most popular tags
            Use this method to obtain a list of popular tags for the saved search queries in Shodan.
            (GET /shodan/query/tags)
        Parameters:
            size (optional): [Integer] The number of tags to return (default: 10).
            api_key (&str): Shodan API Key
        Returns:
            Result<Response, ReqwestError>: 
                Ok(Response): Returns the result of the GET request containing the response.
                Err(ReqwestError): Returns an error if the GET request fails.
    */
    let client = Client::new();
    let url = format!("https://api.shodan.io/shodan/query/tags?size={}&key={}", size, api_key);
    let response = client.get(&url).send()?;
    Ok(response)
}

pub fn get_account_profile(api_key: &str) -> Result<Response, ReqwestError> {
    /*
        Description:
            Account Profile
            Returns information about the Shodan account linked to this API key.
            (GET /account/profile)
        Parameters:
            api_key (&str): Shodan API Key
        Returns:
            Result<Response, ReqwestError>: 
                Ok(Response): Returns the result of the GET request containing the response.
                Err(ReqwestError): Returns an error if the GET request fails.
    */
    let client = Client::new();
    let url = format!("https://api.shodan.io/account/profile?key={}", api_key);
    let response = client.get(&url).send()?;
    Ok(response)
}

pub fn get_dns_domain(domain: &str, history: &str, type_: &str, page: &str, api_key: &str) -> Result<Response, ReqwestError> {
    /*
        Description:
            Domain Information
            Get all the subdomains and other DNS entries for the given domain. Uses 1 query credit per lookup.
            (GET /dns/domain/{domain})
        Parameters:
            domain: (&str) Domain name to lookup; example "cnn.com"
            history (optional) (&str): True if historical DNS data should be included in the results (default: False)
            type (optional) (&str):  DNS type, possible values are: A, AAAA, CNAME, NS, SOA, MX, TXT
            page (optional) (&str): The page number to page through results 100 at a time (default: 1)
            api_key (&str): Shodan API Key
        Returns:
            Result<Response, ReqwestError>: 
                Ok(Response): Returns the result of the GET request containing the response.
                Err(ReqwestError): Returns an error if the GET request fails.
    */
    let client = Client::new();
    let url = format!("https://api.shodan.io/dns/domain/{}?history={}&type={}&page={}&key={}", domain, history, type_, page, api_key);
    let response = client.get(&url).send()?;
    Ok(response)
}

pub fn get_dns_resolve(hostname: &str, api_key: &str) -> Result<Response, ReqwestError> {
    /*
        Description:
            DNS Lookup
            Look up the IP address for the provided list of hostnames.
            (GET /dns/resolve)
        Parameters:
            hostnames (&str): Comma-separated list of hostnames; example "google.com,bing.com"
            api_key (&str): Shodan API Key
        Returns:
            Result<Response, ReqwestError>: 
                Ok(Response): Returns the result of the GET request containing the response.
                Err(ReqwestError): Returns an error if the GET request fails.
    */
    let client = Client::new();
    let url = format!("https://api.shodan.io/dns/resolve?hostnames={}&key={}", hostname, api_key);
    let response = client.get(&url).send()?;
    Ok(response)
}

pub fn get_dns_reverse(ips: &str, api_key: &str) -> Result<Response, ReqwestError> {
    /*
        Description:
            Reverse DNS Lookup
            Look up the hostnames that have been defined for the given list of IP addresses.
            (GET /dns/reverse)
        Parameters:
            ips (&str): Comma-separated list of IP addresses; example "74.125.227.230,204.79.197.200"
            api_key (&str): Shodan API Key
        Returns:
            Result<Response, ReqwestError>: 
                Ok(Response): Returns the result of the GET request containing the response.
                Err(ReqwestError): Returns an error if the GET request fails.
    */
    let client = Client::new();
    let url = format!("https://api.shodan.io/dns/reverse?ips={}&key={}", ips, api_key);
    let response = client.get(&url).send()?;
    Ok(response)
}

pub fn get_tools_headers(api_key: &str) -> Result<Response, ReqwestError> {
    /*
        Description:
            HTTP Headers
            Shows the HTTP headers that your client sends when connecting to a webserver.
            (GET /tools/httpheaders)
        Parameters:
            api_key (&str): Shodan API Key
        Returns:
            Result<Response, ReqwestError>: 
                Ok(Response): Returns the result of the GET request containing the response.
                Err(ReqwestError): Returns an error if the GET request fails.
    */
    let client = Client::new();
    let url = format!("https://api.shodan.io/tools/httpheaders?key={}", api_key);
    let response = client.get(&url).send()?;
    Ok(response)
}

pub fn get_tools_myip(api_key: &str) -> Result<Response, ReqwestError> {
    /*
        Description:
            My IP Address
            Get your current IP address as seen from the Internet.
            (GET /tools/myip)
        Parameters:
            api_key (&str): Shodan API Key
        Returns:
            Result<Response, ReqwestError>: 
                Ok(Response): Returns the result of the GET request containing the response.
                Err(ReqwestError): Returns an error if the GET request fails.
    */
    let client = Client::new();
    let url = format!("https://api.shodan.io/tools/myip?key={}", api_key);
    let response = client.get(&url).send()?;
    Ok(response)
}

pub fn get_api_info(api_key: &str) -> Result<Response, ReqwestError> {
    /*
        Description:
            API Plan Information
            Returns information about the API plan belonging to the given API key.
            (GET /api-info)
        Parameters:
            api_key (&str): Shodan API Key
        Returns:
            Result<Response, ReqwestError>: 
                Ok(Response): Returns the result of the GET request containing the response.
                Err(ReqwestError): Returns an error if the GET request fails.
    */
    let client = Client::new();
    let url = format!("https://api.shodan.io/api-info?key={}", api_key);
    let response = client.get(&url).send()?;
    Ok(response)
}

pub fn get_all_network_alerts(api_key: &str) -> Result<Response, ReqwestError> {
    /*
        Description:
            All Network Alerts
            Subscribe to banners discovered on all IP ranges described in the network alerts. The network alerts are renewed periodically every 1 hour.
            (GET /shodan/alert)
        Parameters:
            api_key (&str): Shodan API Key
        Returns:
            Result<Response, ReqwestError>: 
                Ok(Response): Returns the result of the GET request containing the response.
                Err(ReqwestError): Returns an error if the GET request fails.
    */
    let client = Client::builder().timeout(None).build()?;
    let url = format!("https://stream.shodan.io/shodan/alert?key={}", api_key);
    let response = client.get(&url).send()?;
    Ok(response)
}

pub fn get_all_network_alert_for_alert_id(alert_id: &str, api_key: &str) -> Result<Response, ReqwestError> {
    /*
        Description:
            Filtered by Alert ID
            Subscribe to banners discovered on the IP range defined in a specific network alert. The network alert is renewed periodically every 1 hour.
            (GET /shodan/alert/{id})
        Parameters:
            alert_id (&str): The unique ID that was returned by (POST /shodan/alert) in the Network Alerts REST API.
            api_key (&str): Shodan API Key
        Returns:
            Result<Response, ReqwestError>: 
                Ok(Response): Returns the result of the GET request containing the response.
                Err(ReqwestError): Returns an error if the GET request fails.
    */
    let client = Client::builder().timeout(None).build()?;
    let url = format!("https://stream.shodan.io/shodan/alert/{}?key={}", alert_id, api_key);
    let response = client.get(&url).send()?;
    Ok(response)
}