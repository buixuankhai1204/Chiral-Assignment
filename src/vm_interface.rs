use crate::client::{Client, Zone};
use crate::response::{
    DeleteInstanceResponse, LaunchInstanceResponse, NewInstanceResponse, ShutdownInstanceResponse,
};
use async_trait::async_trait;

#[async_trait]
pub trait GoogleClouldPlatformInterface {
    async fn new_instance(&self, project_id: &str, zone: &Zone) -> Result<NewInstanceResponse, String>;
    async fn delete_instance(
        &self,
        project_id: &str,
        zone: &Zone,
        instance_name: &str,
    ) -> Result<DeleteInstanceResponse, String>;
    async fn list_instances(
        &self,
        project_id: &str,
        zone: &Zone,
    ) -> Result<Vec<NewInstanceResponse>, String>;
    async fn launch_instance(
        &self,
        project_id: &str,
        zone: &Zone,
        instance_name: &str,
    ) -> Result<LaunchInstanceResponse, String>;
    async fn shutdown_instance(
        &self,
        project_id: &str,
        zone: &Zone,
        instance_name: &str,
    ) -> Result<ShutdownInstanceResponse, String>;
}

#[async_trait]
impl GoogleClouldPlatformInterface for Client {
    async fn new_instance(
        &self,
        project_id: &str,
        zone: &Zone,
    ) -> Result<NewInstanceResponse, String> {
        let client = Client::default().set_zone(Zone::Ishikari1);
        let response: NewInstanceResponse =
             match client.clone().instances(project_id, zone).post().await {
                Ok(value) => value,
                Err(e) => {
                    return Err(format!("{:?}", e));
                }
            };
         Ok(response)
    }

    async fn delete_instance(
        &self,
        project_id: &str,
        zone: &Zone,
        instance_name: &str,
    ) -> Result<DeleteInstanceResponse, String> {
        let client = Client::default().set_zone(Zone::Ishikari1);
        let response: DeleteInstanceResponse = match client
            .clone()
            .delete_instance(project_id, instance_name, zone)
            .delete()
            .await
        {
            Ok(value) => return Ok(value.json().await.unwrap()),
            Err(e) => {
                return Err(format!("{:?}", e));
            }
        };
        Ok(response)
    }

    async fn list_instances(
        &self,
        project_id: &str,
        zone: &Zone,
    ) -> Result<Vec<NewInstanceResponse>, String> {
        let client = Client::default().set_zone(Zone::Ishikari1);
        let response: Vec<NewInstanceResponse> =
            match client.clone().list(project_id, zone).get().await {
                Ok(value) => value,
                Err(e) => {
                    return Err(format!("{:?}", e))
                }
            };
        Ok(response)
    }

    async fn launch_instance(
        &self,
        project_id: &str,
        zone: &Zone,
        instance_name: &str,
    ) -> Result<LaunchInstanceResponse, String> {
        let client = Client::default().set_zone(Zone::Ishikari1);
        let response: LaunchInstanceResponse = match client
            .clone()
            .launch(project_id, instance_name, zone)
            .put()
            .await
        {
            Ok(value) => return value.json().await.unwrap(),
            Err(e) => {
                return Err(format!("{:?}", e));
            }
        };
        Ok(response)
    }

    async fn shutdown_instance(
        &self,
        project_id: &str,
        zone: &Zone,
        instance_name: &str,
    ) -> Result<ShutdownInstanceResponse, String> {
        let client = Client::default().set_zone(Zone::Ishikari1);
        let response: ShutdownInstanceResponse = match client
            .clone()
            .shutdown(project_id, instance_name, zone)
            .put()
            .await
        {
            Ok(value) => return value.json().await.unwrap(),
            Err(e) => {
                return Err(format!("{:?}", e));
            }
        };
        Ok(response)
    }
}
