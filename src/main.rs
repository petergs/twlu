use clap::{arg, Arg, Command};
use serde::{Deserialize, Serialize};
use std::env;
use std::process;
use ureq::serde_json;

struct TwClient {
    sid: String,
    token: String,
    fields: Vec<LookupField>,
}

enum LookupField {
    CallerName,
    LineTypeIntelligence,
}

impl LookupField {
    fn name(&self) -> &str {
        match self {
            LookupField::CallerName => "caller_name",
            LookupField::LineTypeIntelligence => "line_type_intelligence",
        }
    }
}

// using Option heavily because everything has the possibility of being null
#[derive(Debug, Serialize, Deserialize)]
struct CallerName {
    caller_name: Option<String>,
    caller_type: Option<String>,
    error_code: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
struct LineTypeIntelligence {
    carrier_name: Option<String>,
    error_code: Option<u64>,
    mobile_country_code: Option<String>,
    mobile_network_code: Option<String>,
    #[serde(rename = "type")]
    line_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Lookup {
    call_forwarding: Option<String>,
    caller_name: Option<CallerName>,
    calling_country_code: Option<String>,
    country_code: Option<String>,
    identity_match: Option<String>,
    line_type_intelligence: Option<LineTypeIntelligence>,
}

impl TwClient {
    const URL_BASE: &str = "https://lookups.twilio.com/v2/PhoneNumbers";

    fn lookup(&self, number: &str) -> Result<Lookup, ureq::Error> {
        let unpw = format!("{}:{}", self.sid, self.token);
        let encoded = data_encoding::BASE64.encode(unpw.as_bytes());
        let basic = format!("Basic {}", encoded);
        let fref = &self.fields;
        let fields: Vec<&str> = fref.into_iter().map(|f| f.name()).collect();
        let url = {
            if fields.len() == 0 {
                format!("{}/{}", TwClient::URL_BASE, number)
            } else {
                format!(
                    "{}/{}?Fields={}",
                    TwClient::URL_BASE,
                    number,
                    fields.join(",")
                )
            }
        };
        let body: Lookup = ureq::get(&url)
            .set("Authorization", &basic)
            .call()?
            .into_json()?;
        Ok(body)
    }
}

fn cli() -> Command {
    Command::new("twlu")
        .about("CLI interface for the Twilio Lookup API V2")
        .arg(arg!(<NUMBER> "Phone number to lookup"))
        .arg_required_else_help(true)
        .arg(
            Arg::new("CALLER_NAME")
                .short('n')
                .long("caller-name")
                .required(false)
                .num_args(0)
                .help("Lookup Caller Name"),
        )
        .arg(
            Arg::new("LINE_TYPE")
                .short('t')
                .long("line-type")
                .required(false)
                .num_args(0)
                .help("Lookup Line Type Intelligence"),
        )
}

fn main() {
    let sid = env::var("TWILIO_ACCOUNT_SID").expect("TWILIO_ACCOUNT_SID is not defined.");
    let token = env::var("TWILIO_AUTH_TOKEN").expect("TWILIO_AUTH_TOKEN is not defined.");

    // handle cli
    // might want to refactor the fields argument to a comma separated list
    let cli = cli();
    let matches = cli.get_matches();
    let mut fields: Vec<LookupField> = vec![];

    if matches.get_flag("CALLER_NAME") {
        fields.push(LookupField::CallerName);
        println!("CallerName");
    }
    if matches.get_flag("LINE_TYPE") {
        fields.push(LookupField::LineTypeIntelligence);
        println!("LineType");
    }
    if fields.len() == 0 {
        // set the default case (no flags to return all information)
        fields = vec![LookupField::CallerName, LookupField::LineTypeIntelligence]
    }

    let tw = TwClient { sid, token, fields };
    let number = matches.get_one::<String>("NUMBER").unwrap();
    let res = tw.lookup(number);
    match res {
        Ok(val) => {
            let output = serde_json::to_string_pretty(&val).unwrap();
            println!("{}", output);
        }
        Err(e) => {
            println!("Error retrieving information on {}: {}", number, e);
            process::exit(1);
        }
    };
}
