use crate::{
    app::error::AppError,
    domain::repository::{
        parking_session_repository::ParkingSessionRepository,
        tariff_policy_repository::TariffPolicyRepository,
    },
};
use chrono::Utc;

pub struct CheckOutInput {
    plate: String,
}

pub struct CheckOut {
    parking_session_repository: Box<dyn ParkingSessionRepository<Error = AppError>>,
    tariff_policy_repository: Box<dyn TariffPolicyRepository<Error = AppError>>,
}

impl CheckOut {
    pub fn new(
        parking_session_repository: Box<dyn ParkingSessionRepository<Error = AppError>>,
        tariff_policy_repository: Box<dyn TariffPolicyRepository<Error = AppError>>,
    ) -> Self {
        Self {
            parking_session_repository,
            tariff_policy_repository,
        }
    }

    pub async fn perform(&self, input: CheckOutInput) -> Result<(), AppError> {
        let mut parking_session = self
            .parking_session_repository
            .find_by_plate(&input.plate)
            .await?;

        let tariff_policy = self
            .tariff_policy_repository
            .find_by_id(parking_session.tariff_policy_id)
            .await?;

        let checkout_date = Utc::now();

        let price = tariff_policy.calculate(parking_session.started_at, checkout_date);
        
        parking_session.close(checkout_date, price)?;

        self.parking_session_repository
            .update(&parking_session)
            .await?;

        Ok(())
    }
}
