use deadpool::managed::Manager;
use log::info;

use crate::fight_instance::SpacetimeConnectionInstance;

pub struct InstancePool {
    pub db_name: String,
    pub db_url: String,
}

impl Manager for InstancePool {
    type Type = SpacetimeConnectionInstance;

    type Error = String;

    async fn create(&self) -> Result<Self::Type, Self::Error> {
        info!("Created connection");
        Ok(SpacetimeConnectionInstance::new(&self.db_name, &self.db_url).await)
    }

    async fn recycle(
        &self,
        _obj: &mut Self::Type,
        _metrics: &deadpool::managed::Metrics,
    ) -> deadpool::managed::RecycleResult<Self::Error> {
        Ok(())
    }
}
