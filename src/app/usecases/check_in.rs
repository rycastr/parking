use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::{
    app::error::AppError,
    domain::{
        model::parking_session::ParkingSession,
        repository::{
            parking_session_repository::ParkingSessionRepository,
            tariff_policy_repository::TariffPolicyRepository,
        },
    },
};

pub struct CheckInInput {
    plate: String,
}

pub struct Ticket {
    pub session_id: Uuid,
    pub plate: String,
    pub checkin_date: DateTime<Utc>,
}

pub struct CheckIn {
    parking_session_repository: Box<dyn ParkingSessionRepository<Error = AppError>>,
    tariff_policy_repository: Box<dyn TariffPolicyRepository<Error = AppError>>,
}

impl CheckIn {
    pub fn new(
        parking_session_repository: Box<dyn ParkingSessionRepository<Error = AppError>>,
        tariff_policy_repository: Box<dyn TariffPolicyRepository<Error = AppError>>,
    ) -> Self {
        Self {
            parking_session_repository,
            tariff_policy_repository,
        }
    }

    pub async fn perform(&self, input: CheckInInput) -> Result<Ticket, AppError> {
        let tariff_policy = self
            .tariff_policy_repository
            .find_by_parking_session_plate(&input.plate)
            .await?;

        let parking_session_id = Uuid::new_v4();
        let checkin_date = Utc::now();
        let parking_session = ParkingSession::start(
            parking_session_id,
            input.plate,
            tariff_policy.id,
            checkin_date,
        );

        self.parking_session_repository
            .save(&parking_session)
            .await?;

        Ok(Ticket {
            session_id: parking_session_id,
            plate: parking_session.plate,
            checkin_date: parking_session.started_at,
        })
    }
}
