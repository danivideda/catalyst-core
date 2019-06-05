use jcli_app::utils::{DebugFlag, HostAddr, OutputFormat, RestApiSender};
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(rename_all = "kebab-case")]
pub enum Account {
    /// Get account state
    Get {
        #[structopt(flatten)]
        addr: HostAddr,
        #[structopt(flatten)]
        debug: DebugFlag,
        #[structopt(flatten)]
        output_format: OutputFormat,
        /// ID of an account, bech32-encoded
        account_id: String,
    },
}

impl Account {
    pub fn exec(self) {
        let Account::Get {
            addr,
            debug,
            output_format,
            account_id,
        } = self;
        let url = addr
            .with_segments(&["v0", "account", &account_id])
            .unwrap()
            .into_url();
        let builder = reqwest::Client::new().get(url);
        let response = RestApiSender::new(builder, &debug).send().unwrap();
        response.response().error_for_status_ref().unwrap();
        let state = response.body().json_value().unwrap();
        let formatted = output_format.format_json(state).unwrap();
        println!("{}", formatted);
    }
}
