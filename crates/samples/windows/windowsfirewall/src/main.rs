use windows::Win32::Foundation::VARIANT_BOOL;
use windows::Win32::NetworkManagement::WindowsFirewall::{
        INetFwPolicy2,
        NetFwPolicy2, NET_FW_IP_PROTOCOL_TCP, NetFwRule, INetFwRule, NET_FW_ACTION_BLOCK
    };
use windows::Win32::System::Com::{
        CLSCTX_INPROC_SERVER,
        COINIT_MULTITHREADED,
        CoInitializeEx,
        CoCreateInstance
    };
use windows::core::{
        BSTR
    };

// Please Note - Firewall rules require administrative privileges, and you will be denied if you just run from a build in user mode

fn main() {

    let rule_name:String = String::from("My Rule Name By Rust"); 
    add_firewall_rule(rule_name.clone());
    validate_firewall_rule_exists(rule_name.clone());
    edit_firewall_rule(rule_name.clone());
    remove_firewall_rule(rule_name);
    
}

fn validate_firewall_rule_exists(rule_name: String) {
    unsafe {

        // First you need to initialize the com object
        CoInitializeEx(None, COINIT_MULTITHREADED).expect("couldn't CoInitializeEx");

        // then you need to create the firewall instance
        let fw_policy: windows::core::Result<INetFwPolicy2> = 
        CoCreateInstance(
            &NetFwPolicy2,
            None,
            CLSCTX_INPROC_SERVER
        );

        if let Ok(rules) = fw_policy.unwrap().Rules() {

            
            if let Ok(rule) = rules.Item(&BSTR::from(&rule_name)){
                // code to do when rule found
                println!("The rule '{:?}' does exist", rule.Name().unwrap());
            } else {
                // code to do if rule not found
                println!("The rule '{}' doesn't exist", rule_name);
            }
        } else {
            // code to do if the rules couldn't be loaded
        }
    }
}


fn edit_firewall_rule(rule_name: String) {

    unsafe {
        // First you need to initialize the com object
        CoInitializeEx(None, COINIT_MULTITHREADED).expect("couldn't CoInitializeEx");

        // then you need to create the firewall instance
        let mut fw_policy: windows::core::Result<INetFwPolicy2> = 
        CoCreateInstance(
            &NetFwPolicy2,
            None,
            CLSCTX_INPROC_SERVER,
        );

        // get a list of ips in comma separated format
        let ips = get_ips_to_block();

        //modify the rule
        if let Ok(rules) = fw_policy.as_mut().unwrap().Rules() {
            if let Ok(rule) = rules.Item(&BSTR::from(&rule_name)) {

                println!("Editing rule '{:?}' ", rule.Name().unwrap());

                // setRemoteAddresses automatically updates the firewall. no need to do more
                let _ = rule.SetRemoteAddresses(&BSTR::from(ips)); // modify RemoteAddresses - all options at https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/NetworkManagement/WindowsFirewall/struct.INetFwRule.html
                
                
            } else {
                println!("Rule '{}' Doesn't exist to edit", rule_name);
            }
        }

    }

}

fn add_firewall_rule(rule_name: String) {

    unsafe {
        // First you need to initialize the com object

        CoInitializeEx(None, COINIT_MULTITHREADED).expect("couldn't CoInitializeEx");


        // then you need to create the firewall instance
        let mut fw_policy: windows::core::Result<INetFwPolicy2> = 
        CoCreateInstance(
            &NetFwPolicy2,
            None,
            CLSCTX_INPROC_SERVER,
        );
        // get a list of ips in comma separated format
        let ips = get_ips_to_block();

        // build the rule instance
        let rule: INetFwRule = CoCreateInstance(
            &NetFwRule,
            None,
            CLSCTX_INPROC_SERVER,
        ).unwrap();
        
        // Set the properties of the new rule
        let _ = rule.SetName(&BSTR::from(&rule_name));
        let _ = rule.SetProtocol(NET_FW_IP_PROTOCOL_TCP.0);
        let _ = rule.SetRemoteAddresses(&BSTR::from(ips));
        let _ = rule.SetAction(NET_FW_ACTION_BLOCK);
        let _ = rule.SetDescription(&BSTR::from("This rule was created by rust. It blocks a list of IPs."));
        let _ = rule.SetEnabled(VARIANT_BOOL::from(true));

        // if we have access, it adds the rule
        if let Ok(rules) = fw_policy.as_mut().unwrap().Rules() {
            rules.Add(&rule).expect("Couldn't add rule"); // add new rule
            println!("Rule '{:?}' was added to the firewall", rule.Name().unwrap())
        } else {
            println!("Couldn't add rule '{}' to the firewall", rule_name)
        }
    }

}

fn remove_firewall_rule(rule_name: String) {

    unsafe {
        // First you need to initialize the com object

        CoInitializeEx(None, COINIT_MULTITHREADED).expect("couldn't CoInitializeEx");


        // then you need to create the firewall instance
        let mut fw_policy: windows::core::Result<INetFwPolicy2> = 
        CoCreateInstance(
            &NetFwPolicy2,
            None,
            CLSCTX_INPROC_SERVER,
        );

        
        // if we have access, it adds the rule
        if let Ok(rules) = fw_policy.as_mut().unwrap().Rules() {
            rules.Remove(&BSTR::from(&rule_name)).expect("Couldn't add rule"); // add new rule
            println!("Rule '{}' was removed from the firewall", &rule_name)
        } else {
            println!("Couldn't remove rule '{}' from the firewall", rule_name)
        }
    }

}

fn get_ips_to_block() -> String {

    // this is code to return a comma separated string
    let ip_list = "10.0.0.1,10.0.0.2";
    
    String::from(ip_list)
}

