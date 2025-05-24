use async_trait::async_trait;

use crate::domain::model::tariff_policy::TariffPolicy;

#[async_trait]
pub trait TariffPolicyRepository {
    type Error;

    async fn find_by_parking_session_plate(&self, plate: &str)
    -> Result<TariffPolicy, Self::Error>;
    async fn find_by_id(&self, id: i64) -> Result<TariffPolicy, Self::Error>;
}
