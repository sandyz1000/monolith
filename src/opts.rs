use clap::{Arg, ArgAction, Command};
use std::env;

use crate::cookies::Cookie;

#[derive(Default)]
pub struct Options {
    pub no_audio: bool,
    pub base_url: Option<String>,
    pub blacklist_domains: bool,
    pub no_css: bool,
    pub cookie_file: Option<String>,
    pub cookies: Vec<Cookie>,
    pub domains: Option<Vec<String>>,
    pub ignore_errors: bool,
    pub encoding: Option<String>,
    pub no_frames: bool,
    pub no_fonts: bool,
    pub no_images: bool,
    pub isolate: bool,
    pub no_js: bool,
    pub insecure: bool,
    pub no_metadata: bool,
    pub output: String,
    pub silent: bool,
    pub timeout: u64,
    pub user_agent: Option<String>,
    pub no_video: bool,
    pub target: String,
    pub no_color: bool,
    pub unwrap_noscript: bool,
}

const ASCII: &'static str = " \
 _____     ______________    __________      ___________________    ___
|     \\   /              \\  |          |    |                   |  |   |
|      \\_/       __       \\_|    __    |    |    ___     ___    |__|   |
|               |  |            |  |   |    |   |   |   |   |          |
|   |\\     /|   |__|    _       |__|   |____|   |   |   |   |    __    |
|   | \\___/ |          | \\                      |   |   |   |   |  |   |
|___|       |__________|  \\_____________________|   |___|   |___|  |___|
";
const DEFAULT_NETWORK_TIMEOUT: u64 = 120;
const DEFAULT_USER_AGENT: &'static str =
    "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:73.0) Gecko/20100101 Firefox/73.0";
const ENV_VAR_NO_COLOR: &str = "NO_COLOR";
const ENV_VAR_TERM: &str = "TERM";

impl Options {
    pub fn from_args() -> Options {
        let about = format!("{}\n{}", ASCII, env!("CARGO_PKG_DESCRIPTION"));
        let app = Command::new(env!("CARGO_PKG_NAME"))
            .version(env!("CARGO_PKG_VERSION"))
            .author("sandip.dey1988 <sandip.dey1988@gmail>")
            .about(&about)
            .arg(
                Arg::new("no-audio")
                    .short('a')
                    .long("--no-audio")
                    .action(ArgAction::SetTrue)
                    .help("Remove audio sources"),
            )
            .arg(
                Arg::new("base-url")
                    .short('b')
                    .long("base-url")
                    .default_value("[http://localhost/]")
                    .help("Set custom base URL"),
            )
            .arg(
                Arg::new("blacklist-domains")
                    .short('B')
                    .long("blacklist-domains")
                    .action(ArgAction::SetTrue)
                    .help("Treat list of specified domains as blacklist"),
            )
            .arg(
                Arg::new("no-css")
                    .short('c')
                    .long("no-css")
                    .action(ArgAction::SetTrue)
                    .help("Remove CSS"),
            )
            .arg(
                Arg::new("ignore-errors")
                    .short('e')
                    .long("ignore-errors")
                    .action(ArgAction::SetTrue)
                    .help("Ignore network errors"),
            )
            .arg(
                Arg::new("no-frames")
                    .short('f')
                    .long("no-frames")
                    .action(ArgAction::SetTrue)
                    .help("Remove frames and iframes"),
            )
            .arg(
                Arg::new("no-fonts")
                    .short('F')
                    .long("no-fonts")
                    .action(ArgAction::SetTrue)
                    .help("Remove fonts"),
            )
            .arg(
                Arg::new("no-images")
                    .short('i')
                    .long("no-images")
                    .action(ArgAction::SetTrue)
                    .help("Remove images"),
            )
            .arg(
                Arg::new("isolate")
                    .short('I')
                    .long("isolate")
                    .action(ArgAction::SetTrue)
                    .help("Cut off document from the Internet"),
            )
            .arg(
                Arg::new("no-js")
                    .short('j')
                    .long("no-js")
                    .action(ArgAction::SetTrue)
                    .help("Remove JavaScript"),
            )
            .arg(
                Arg::new("insecure")
                    .short('k')
                    .long("insecure")
                    .action(ArgAction::SetTrue)
                    .help("Allow invalid X.509 (TLS) certificates"),
            )
            .arg(
                Arg::new("no-metadata")
                    .short('M')
                    .long("no-metadata")
                    .action(ArgAction::SetTrue)
                    .help("Exclude timestamp and source information"),
            )
            .arg(
                Arg::new("unwrap-noscript")
                    .short('n')
                    .long("unwrap-noscript")
                    .action(ArgAction::SetTrue)
                    .help("Replace NOSCRIPT elements with their contents"),
            )
            .arg(
                Arg::new("silent")
                    .short('s')
                    .long("silent")
                    .action(ArgAction::SetTrue)
                    .help("Suppress verbosity"),
            )
            .arg(
                Arg::new("no-video")
                    .short('v')
                    .long("no-video")
                    .action(ArgAction::SetTrue)
                    .help("Remove video sources"),
            )
            .arg(
                Arg::new("domains")
                    .short('d')
                    .long("domain")
                    .action(ArgAction::Set)
                    .value_name("example.com")
                    .action(ArgAction::Append)
                    .help("Specify domains to use for white/black-listing"),
            )
            .arg(
                Arg::new("target")
                    .required(true)
                    .action(ArgAction::Set)
                    .index(1)
                    .help("URL or file path, use - for STDIN"),
            )
            .arg(
                Arg::new("cookies")
                    .short('C')
                    .long("cookies")
                    .default_value("[cookies.txt]")
                    .help("Specify cookie file"),
            )
            .arg(
                Arg::new("encoding")
                    .short('E')
                    .long("encoding")
                    .default_value("[UTF-8]")
                    .help("Enforce custom charset"),
            )
            .arg(
                Arg::new("timeout")
                    .short('t')
                    .long("timeout")
                    .default_value("[60]")
                    .help("Adjust network request timeout"),
            )
            .arg(
                Arg::new("user-agent")
                    .short('u')
                    .long("user-agent")
                    .default_value("[Firefox]")
                    .help("Set custom User-Agent string"),
            )
            .arg(
                Arg::new("output")
                    .short('o')
                    .long("output")
                    .default_value("[document.html]")
                    .help("Write output to <file>, use - for STDOUT"),
            )
            .get_matches();
        let mut options: Options = Options::default();

        // Process the command
        options.target = app
            .get_one::<String>("target")
            .expect("please set target")
            .to_string();
        
        options.no_audio = app.get_flag("no-audio");
        options.blacklist_domains = app.get_flag("blacklist-domains");
        options.no_css = app.get_flag("no-css");
        options.ignore_errors = app.get_flag("ignore-errors");
        options.no_frames = app.get_flag("no-frames");
        options.no_fonts = app.get_flag("no-fonts");
        options.no_images = app.get_flag("no-images");
        options.isolate = app.get_flag("isolate");
        options.no_js = app.get_flag("no-js");
        options.insecure = app.get_flag("insecure");
        options.no_metadata = app.get_flag("no-metadata");
        options.silent = app.get_flag("silent");
        options.unwrap_noscript = app.get_flag("unwrap-noscript");
        options.no_video = app.get_flag("no-video");
        options.output = app
            .get_one::<String>("output")
            .unwrap_or(&"".to_string())
            .to_string();
        options.timeout = app
            .get_one::<String>("timeout")
            .unwrap_or(&DEFAULT_NETWORK_TIMEOUT.to_string())
            .parse::<u64>()
            .unwrap();

        if let Some(base_url) = app.get_one::<String>("base-url") {
            options.base_url = Some(base_url.to_string());
        }
        if let Some(cookie_file) = app.get_one::<String>("cookies") {
            options.cookie_file = Some(cookie_file.to_string());
        }
        if let Some(encoding) = app.get_one::<String>("encoding") {
            options.encoding = Some(encoding.to_string());
        }
        if let Some(domains) = app.get_many::<String>("domains") {
            let list_of_domains: Vec<String> = domains.map(|v| v.clone()).collect::<Vec<_>>();
            options.domains = Some(list_of_domains);
        }
        if let Some(user_agent) = app.get_one::<String>("user-agent") {
            options.user_agent = Some(user_agent.to_string());
        } else {
            options.user_agent = Some(DEFAULT_USER_AGENT.to_string());
        }
        
        options.no_color =
            env::var_os(ENV_VAR_NO_COLOR).is_some() || atty::isnt(atty::Stream::Stderr);
        if let Some(term) = env::var_os(ENV_VAR_TERM) {
            if term == "dumb" {
                options.no_color = true;
            }
        }

        options
    }
}
