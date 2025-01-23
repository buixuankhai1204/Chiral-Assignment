use crate::response::NewInstanceResponse;
use crate::vm_interface::GoogleClouldPlatformInterface;
use reqwest::Response;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::env;
use std::fmt::Display;

const BASE_URL: &str = "https://compute.googleapis.com/compute/v1";

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Zone {
    #[default]
    Tokyo1,
    Tokyo2,
    Ishikari1,
    Ishikari2,
}

impl Display for Zone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match &self {
                Zone::Tokyo1 => "tk1a",
                Zone::Tokyo2 => "tk1b",
                Zone::Ishikari1 => "is1a",
                Zone::Ishikari2 => "is1b",
            }
        )
    }
}
#[derive(Clone, Debug)]
pub struct Client {
    key_1: String,
    key_2: Option<String>,
    zone: Zone,
    service: String,     // "cloud" for Normal API, "managed-container" for DOK API
    api_version: String, // 1.1 for Normal API, 1.0 for DOK API
    url: String,
    body: String,
}

impl Default for Client {
    fn default() -> Self {
        let key_1 = env::var("SAKURA_KEY1").unwrap();
        let key_2 = env::var("SAKURA_KEY2").ok();
        Self::new(key_1, key_2)
    }
}
impl Client {
    pub fn new(key_1: String, key_2: Option<String>) -> Self {
        let zone = Zone::Tokyo1;
        Self {
            key_1,
            key_2,
            zone,
            service: "cloud".to_string(),
            api_version: "1.1".to_string(),
            url: String::new(),
            body: String::new(),
        }
    }

    pub fn dok(mut self) -> Self {
        self.service = "managed-container".to_string();
        self.api_version = "1.0".to_string();
        self.zone = Zone::Ishikari1;
        self
    }

    pub fn get_zone(&self) -> Zone {
        self.zone
    }

    pub fn set_zone(mut self, zone: Zone) -> Self {
        self.zone = zone;
        self
    }

    pub fn clear(mut self) -> Self {
        self.url.clear();
        self.body.clear();
        self
    }

    pub fn set_params<P: Serialize>(mut self, params: &P) -> anyhow::Result<Self> {
        self.body = serde_json::to_string(params)?;
        Ok(self)
    }

    // for test purpose
    pub async fn get_raw(&self) {
        println!("Request URL: {}", self.full_url());
        let client = reqwest::Client::new();
        let res = client
            .get(self.full_url())
            .basic_auth(&self.key_1, self.key_2.as_ref())
            .send()
            .await
            .unwrap();
        if let Ok(text_response) = res.text().await {
            println!("Response Text:\n {} \n", &text_response);
            let v: serde_json::Value = serde_json::from_str(&text_response).unwrap();
            println!("{}", serde_json::to_string_pretty(&v).unwrap());
        } else {
            panic!("no text response");
        }
    }

    pub async fn get<T: DeserializeOwned>(&self) -> anyhow::Result<T> {
        let client = reqwest::Client::new();
        let res = client
            .get(self.full_url())
            .basic_auth(&self.key_1, self.key_2.as_ref())
            .send()
            .await?;
        let t: T = res.json().await?;
        Ok(t)
    }

    pub async fn get_list<T: DeserializeOwned>(&self) -> anyhow::Result<Vec<T>> {
        let client = reqwest::Client::new();
        let res = client
            .get(self.full_url())
            .basic_auth(&self.key_1, self.key_2.as_ref())
            .send()
            .await?;
        let t: Vec<T> = res.json().await?;
        Ok(t)
    }

    pub async fn get_with_params<T: DeserializeOwned, K: AsRef<str>, V: AsRef<str>>(
        &self,
        params: &[(K, V)],
    ) -> anyhow::Result<T> {
        let url = reqwest::Url::parse_with_params(self.full_url().as_str(), params)?;
        let client = reqwest::Client::new();
        let res = client
            .get(url)
            .basic_auth(&self.key_1, self.key_2.as_ref())
            .send()
            .await?;
        let t: T = res.json().await?;
        Ok(t)
    }

    pub async fn post_raw(&self) {
        let client = reqwest::Client::new();
        let res = client
            .post(self.full_url())
            .basic_auth(&self.key_1, self.key_2.as_ref())
            .body(self.body.clone())
            .send()
            .await
            .unwrap();
        let v: serde_json::Value = serde_json::from_str(&res.text().await.unwrap()).unwrap();
        println!("{}", serde_json::to_string_pretty(&v).unwrap());
    }

    pub async fn post<T: DeserializeOwned>(&self) -> anyhow::Result<T> {
        let client = reqwest::Client::new();
        let res = client
            .post(self.full_url())
            .basic_auth(&self.key_1, self.key_2.as_ref())
            .body(self.body.clone())
            .send()
            .await?;
        let t: T = res.json().await?;
        Ok(t)
    }

    pub async fn delete_raw(&self) {
        let client = reqwest::Client::new();
        let res = client
            .delete(self.full_url())
            .basic_auth(&self.key_1, self.key_2.as_ref())
            .body(self.body.clone())
            .send()
            .await
            .unwrap();
        let v: serde_json::Value = serde_json::from_str(&res.text().await.unwrap()).unwrap();
        println!("{}", serde_json::to_string_pretty(&v).unwrap());
    }

    pub async fn delete(&self) -> anyhow::Result<Response> {
        let client = reqwest::Client::new();
        client
            .delete(self.full_url())
            .basic_auth(&self.key_1, self.key_2.as_ref())
            .body(self.body.clone())
            .send()
            .await
            .map_err(|e| anyhow::Error::msg(format!("{e}")))
    }

    pub async fn put_raw(&self) {
        let client = reqwest::Client::new();
        let res = client
            .put(self.full_url())
            .basic_auth(&self.key_1, self.key_2.as_ref())
            .body(self.body.clone())
            .send()
            .await
            .unwrap();
        let v: serde_json::Value = serde_json::from_str(&res.text().await.unwrap()).unwrap();
        println!("{}", serde_json::to_string_pretty(&v).unwrap());
    }

    pub async fn put(&self) -> anyhow::Result<Response> {
        let client = reqwest::Client::new();
        let res = client
            .put(self.full_url())
            .basic_auth(&self.key_1, self.key_2.as_ref())
            .body(self.body.clone())
            .send()
            .await?;
        Ok(res)
    }
    pub fn full_url(&self) -> String {
        // Normal API URL
        // https://secure.sakura.ad.jp/cloud/zone/tk1a/api/cloud/1.1/
        // DOK API
        // https://secure.sakura.ad.jp/cloud/zone/is1a/api/managed-container/1.0/
        format!(
            "{}/{:?}/api/{}/{}{}",
            BASE_URL, self.zone, self.service, self.api_version, self.url
        )
    }
    fn extend_url(mut self, s: &str) -> Self {
        self.url += s;
        self
    }
    pub fn instances(self, project_id: &str, zone: &Zone) -> Self {
        self.extend_url(format!("/projects/{project_id}/zones/{zone}/instances").as_str())
    }
    pub fn launch(self, project_id: &str, instance_name: &str, zone: &Zone) -> Self {
        self.extend_url(
            format!("/projects/{project_id}/zones/{zone}/instances/{instance_name}/start").as_str(),
        )
    }
    pub fn shutdown(self, project_id: &str, instance_name: &str, zone: &Zone) -> Self {
        self.extend_url(
            format!("/projects/{project_id}/zones/{zone}/instances/{instance_name}/stop").as_str(),
        )
    }
    pub fn delete_instance(self, project_id: &str, instance_name: &str, zone: &Zone) -> Self {
        self.extend_url(format!("/projects/{project_id}/zones/{zone}/instances/{instance_name}").as_str())
    }

    pub fn list(self, project_id: &str, zone: &Zone) -> Self {
        self.extend_url(format!("/projects/{project_id}/zones/{zone}/instances").as_str())
    }
}

