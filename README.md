# twlu
> Small CLI tool to query the Twilio Lookup API V2 for a phone number

The [Twilio Lookup API](https://www.twilio.com/docs/lookup/v2-api) provides a paid 
([per request](https://www.twilio.com/en-us/trusted-activation/pricing/lookup)) way to 
query the [line information database](https://en.wikipedia.org/wiki/Line_information_database) 
for [CNAM](https://en.wikipedia.org/wiki/Calling_Name_Presentation) data used in caller 
ID services. The API also has an endpoint to identify the line type (landline, mobile, voip, etc) 
and carrier.

This tool provides a quick interface to query this API for a phone number and return some basic 
data in JSON format. 

## Setup 
1. Create a Twilio Account with access to the Lookup API. The API costs ~$0.01 per request. 
2. Set the environment variables `TWILIO_ACCOUNT_SID` and `TWILIO_AUTH_TOKEN`. You can retrieve these
values by followng the directions in [this article](https://support.twilio.com/hc/en-us/articles/223136027-Auth-Tokens-and-How-to-Change-Them).

## Usage
```
$> twlu --help      
CLI interface for the Twilio Lookup API V2

Usage: twlu [OPTIONS] <NUMBER>

Arguments:
  <NUMBER>  Phone number to lookup

Options:
  -n, --caller-name  Lookup Caller Name
  -t, --line-type    Lookup Line Type Intelligence
  -h, --help         Print help
```

## Example nonFixedVoip Query
```
$> twlu +14159929960
{
  "call_forwarding": null,
  "caller_name": {
    "caller_name": null,
    "caller_type": "UNDETERMINED",
    "error_code": null
  },
  "calling_country_code": "1",
  "country_code": "US",
  "identity_match": null,
  "line_type_intelligence": {
    "carrier_name": "Bandwidth/13 - Bandwidth.com - SVR",
    "error_code": null,
    "mobile_country_code": "313",
    "mobile_network_code": "981",
    "type": "nonFixedVoip"
  }
}
```
