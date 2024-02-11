use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::json;

pub mod plugin;

pub struct Repo<'a, T, CreateT>
where
    T: Serialize + for<'de> Deserialize<'de>,
    CreateT: Serialize + for<'de> Deserialize<'de>,
{
    client: &'a reqwest::blocking::Client,
    origin: &'a str,
    resources: &'a str,
    resource: &'a str,
    _phantom: std::marker::PhantomData<T>,
    _phantom2: std::marker::PhantomData<CreateT>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Data<T> {
    data: T,
}

impl<'a, T, CreateT> Repo<'a, T, CreateT>
where
    T: Serialize + for<'de> Deserialize<'de>,
    CreateT: Serialize + for<'de> Deserialize<'de>,
{
    pub fn new(
        client: &'a reqwest::blocking::Client,
        origin: &'a str,
        resources: &'a str,
        resource: &'a str,
    ) -> Self {
        Self {
            client,
            origin,
            resources,
            resource,
            _phantom: std::marker::PhantomData,
            _phantom2: std::marker::PhantomData,
        }
    }

    pub fn index(&self) -> Result<Vec<T>> {
        let url = format!("{}/{}", self.origin, self.resources);
        let res: Data<Vec<T>> = self.client.get(&url).send()?.json()?;
        Ok(res.data)
    }

    pub fn _show(&self, id: &str) -> Result<T> {
        let url = format!("{}/{}/{}", self.origin, self.resources, id);
        let res: Data<T> = self.client.get(&url).send()?.json()?;
        Ok(res.data)
    }

    pub fn create(&self, data: &CreateT) -> Result<T> {
        let url = format!("{}/{}", self.origin, self.resources);
        let req = json!({
            self.resource: data,
        });
        let res: Data<T> = self.client.post(&url).json(&req).send()?.json()?;
        Ok(res.data)
    }

    pub fn _update(&self, id: &str, data: &T) -> Result<T> {
        let url = format!("{}/{}/{}", self.origin, self.resources, id);
        let req = json!({
            self.resource: data,
        });
        let res: Data<T> = self.client.put(&url).json(&req).send()?.json()?;
        Ok(res.data)
    }

    pub fn delete(&self, id: &str) -> Result<()> {
        let url = format!("{}/{}/{}", self.origin, self.resources, id);
        self.client.delete(&url).send()?;
        Ok(())
    }
}
