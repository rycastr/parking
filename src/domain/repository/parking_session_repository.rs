use async_trait::async_trait;

use crate::domain::model::parking_session::ParkingSession;

#[async_trait]
pub trait ParkingSessionRepository {
    type Error;

    async fn save(&self, parking_session: &ParkingSession) -> Result<(), Self::Error>;
    async fn update(&self, parking_session: &ParkingSession) -> Result<(), Self::Error>;
    async fn find_by_plate(&self, plate: &str) -> Result<ParkingSession, Self::Error>;
}
