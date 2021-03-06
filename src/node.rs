//! the info and logic needed to run the bitcoind node

use std::{path::PathBuf, process::Command, str::FromStr};

use mentat::{
    axum::async_trait,
    conf::{Configuration, NodeConf},
    reqwest::Url,
    serde::{Deserialize, Serialize},
};

/// configuration information/logic for the bitcoind node
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(crate = "mentat::serde")]
pub struct NodeConfig {
    /// where to store blocks and other data
    data_dir: PathBuf,
    /// the username to set on bitcoind
    user: String,
    /// the password to set on bitcoind
    pass: String,
}

#[async_trait]
impl NodeConf for NodeConfig {
    const BLOCKCHAIN: &'static str = "Bitcoin";

    fn build_url(conf: &Configuration<Self>) -> Url {
        let url = format!(
            "{}://{}:{}@{}:{}",
            if conf.secure_http { "https" } else { "http" },
            conf.custom.as_ref().unwrap().user,
            conf.custom.as_ref().unwrap().pass,
            conf.node_address,
            conf.node_rpc_port
        );

        Url::from_str(&url).expect("Invalid node url: {url}")
    }

    fn node_command(config: &Configuration<Self>) -> Command {
        let mut command = Command::new(&config.node_path);
        command.args(&[
            // TODO cant bind to address without setting a whitelist
            // &format!("--bind={address}:4132"),
            // &format!("--rpcbind={address}:3032"),
            "-port=4132",
            // TODO `Config options rpcuser and rpcpassword will soon be deprecated.
            // Locally-run instances may remove rpcuser to use cookie-based auth, or may be
            // replaced with rpcauth. Please see share/rpcauth for rpcauth auth generation.`
            &format!("-rpcport={}", config.node_rpc_port),
            &format!("-rpcuser={}", config.custom.as_ref().unwrap().user),
            &format!("-rpcpassword={}", config.custom.as_ref().unwrap().pass),
            "-txindex=1",
            &format!(
                "--datadir={}",
                config.custom.as_ref().unwrap().data_dir.display()
            ),
        ]);
        command
    }
}
