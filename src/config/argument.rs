use std::net::IpAddr;
use std::str::FromStr;

pub struct Argument {
    ip: IpAddr,
    threads: u16,
}

impl Argument {
    pub fn new(vec: &[String]) -> Result<Argument, &str> {
        if vec.len() < 2 {
            return Err("not enough params");
        } else if vec.len() > 5 {
            return Err("too many params");
        }

        if is_help_argument_present(vec) {
            return Err("help");
        }

        let default_thread_count: u16 = 1;

        let ip_adr = match find_ip_address(vec) {
            Some(ip) => ip,
            None => { return Err("ip should be specified with '-ip' param"); }
        };

        let thread_count = find_thread_count(vec).unwrap_or_else(|| default_thread_count);

        Ok(Argument { ip: ip_adr, threads: thread_count })
    }

    pub fn get_thread_count(&self) -> u16 {
        return self.threads;
    }

    pub fn get_ip_address(&self) -> IpAddr {
        return self.ip;
    }
}

fn is_help_argument_present(vec: &[String]) -> bool {
    for param in vec.into_iter() {
        if param.contains("h") {
            return true;
        }
    }
    false
}


fn find_ip_address(vec: &[String]) -> Option<IpAddr> {
    return match find_param(vec, "-ip")
        .map(|ip| { IpAddr::from_str(&ip) }) {
        Some(r) => match r {
            Ok(v) => Some(v),
            Err(_) => None
        },
        None => None
    };
}

fn find_thread_count(vec: &[String]) -> Option<u16> {
    return match find_param(vec, "-t")
        .map(|s| {
            s.parse::<u16>()
        }) {
        Some(r) => match r {
            Ok(v) => Some(v),
            Err(_) => None
        },
        None => None
    };
}

fn find_param(vec: &[String], param_name: &str) -> Option<String> {
    for (i, param) in vec.into_iter().enumerate() {
        if param.contains(param_name) {
            return match vec.get(i + 1) {
                Some(v) => Some(v.clone()),
                None => None
            };
        }
    }
    None
}