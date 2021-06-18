//!  Utilities for the client
//!  The interface to reqwest is defined with the ReqwClientWrapper
#[cfg(feature = "cookie-finder")]
pub fn get_cookie(regtest: bool) -> std::io::Result<String> {
    let mut cookie_path = match dirs::home_dir() {
        Some(x) => x,
        None => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!(
                    "{} {} {}",
                    "Could not find your home directory.",
                    "Please pass the contents of ~/.zcash/.cookie to the",
                    "enviroment variable ZCASH_TEST_AUTH."
                ),
            ))
        }
    };

    cookie_path.push(".zcash");
    if regtest {
        cookie_path.push("regtest");
    }
    cookie_path.push(".cookie");

    let mut cookie_file = std::fs::File::open(cookie_path)?;
    let mut cookie_string = String::new();
    use std::io::Read as _;
    cookie_file.read_to_string(&mut cookie_string)?;
    Ok(cookie_string)
}

pub fn get_zcashd_port() -> String {
    //This could theoretically be expanded to do some sort of
    //automatic port lookup
    String::from("127.0.0.1:18232")
}

pub fn make_client(regtest: bool) -> crate::Client {
    crate::Client::new(get_zcashd_port(), get_cookie(regtest).unwrap())
}

pub(crate) struct ReqwClientWrapper {
    pub(crate) url: String,
    pub(crate) auth: String,
    pub(crate) reqw_client: reqwest::Client,
    pub(crate) idit: std::ops::RangeFrom<u64>,
}

impl ReqwClientWrapper {
    pub(crate) fn new(hostport: String, authcookie: String) -> Self {
        Self {
            url: format!("http://{}/", hostport),
            auth: format!("Basic {}", base64::encode(&authcookie)),
            reqw_client: reqwest::Client::new(),
            idit: (0..),
        }
    }
    pub(crate) fn procedure_call(
        &mut self,
        method: &'static str,
        args: Vec<serde_json::Value>,
    ) -> (
        u64,
        impl std::future::Future<Output = Result<reqwest::Response, reqwest::Error>>,
    ) {
        let id = self.idit.next().unwrap();
        (
            id,
            self.reqw_client
                .post(&self.url)
                .header("Authorization", &self.auth)
                .body(&crate::envelope::RequestEnvelope::seal(id, method, args))
                .send(),
        )
    }
}

///Newtype wrapper around rust_decimal::Decimal to allow for custom
///Serialize impl
#[derive(Debug, serde::Deserialize)]
pub struct ZDecimal(rust_decimal::Decimal);

//Non-standard for new-type wrappers, but it seems prudent in this case
impl core::ops::Deref for ZDecimal {
    type Target = rust_decimal::Decimal;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl serde::Serialize for ZDecimal {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        let decimal = self.0.to_string();
        match decimal.parse::<i64>() {
            Ok(int) => serializer.serialize_i64(int),
            Err(i_err) => match decimal.parse::<f64>() {
                Ok(float) => serializer.serialize_f64(float),
                Err(f_err) => {
                    eprintln!("Not int or float, errors: {} {}", i_err, f_err);
                    serializer.serialize_none()
                }
            },
        }
    }
}
