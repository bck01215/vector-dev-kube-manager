use anyhow::Result;
use futures::{Stream, StreamExt};
use k8s_openapi::api::{
    apps::v1::Deployment,
    core::v1::{ConfigMap, Pod},
};
use kube::{
    api::{Api, ListParams, PostParams},
    runtime::{watcher, watcher::Error, WatchStreamExt},
    Client,
};
use tracing::log::warn;

#[derive(Clone)]
pub struct KubeClient {
    client: Client,
}

impl KubeClient {
    pub async fn new() -> anyhow::Result<Self> {
        Ok(Self {
            client: Client::try_default().await?,
        })
    }
    pub async fn set_deployment(
        &self,
        name: &str,
        config: &str,
        label_match: &str,
    ) -> Result<String> {
        let options = self.get_maps(label_match).await?;
        if !options.contains(&config.to_string()) {
            warn!("ConfigMap {} is not valid", config);
            return Err(anyhow::anyhow!("ConfigMap {} is not valid", config));
        }

        let deployments: Api<Deployment> =
            Api::namespaced(self.client.clone(), self.client.default_namespace());
        let mut deployment = deployments.get(name).await?;

        deployment
            .spec
            .as_mut()
            .unwrap()
            .template
            .spec
            .as_mut()
            .unwrap()
            .volumes
            .as_mut()
            .unwrap()
            .iter_mut()
            .for_each(|v| {
                if v.name == "config" {
                    v.projected
                        .as_mut()
                        .unwrap()
                        .sources
                        .as_mut()
                        .unwrap()
                        .iter_mut()
                        .for_each(|s| {
                            s.config_map.as_mut().unwrap().name = Some(config.to_string());
                        })
                }
            });

        deployments
            .replace(name, &PostParams::default(), &deployment)
            .await?;

        Ok("Updated deployment".to_string())
    }
    pub async fn get_maps(&self, label_match: &str) -> Result<Vec<String>> {
        let mut names: Vec<String> = Vec::new();
        let configmaps: Api<ConfigMap> =
            Api::namespaced(self.client.clone(), self.client.default_namespace());
        let lp = ListParams::default().labels(label_match);
        let list = configmaps.list(&lp).await?;
        for item in list.items {
            if let Some(name) = item.metadata.name {
                names.push(name);
            }
        }
        Ok(names)
    }
    pub async fn watch_pods_stream(
        &self,
        label_match: &str,
    ) -> impl Stream<Item = Result<Pod, Error>> {
        let pods: Api<Pod> = Api::namespaced(self.client.clone(), self.client.default_namespace());
        watcher(pods, watcher::Config::default().labels(label_match))
            .applied_objects()
            .default_backoff()
            .map(|pod| pod)
    }
}
