use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::domain::error::ModelError;

pub struct ParkingSession {
    pub id: Uuid,
    pub plate: String,
    pub tariff_policy_id: i64,
    pub started_at: DateTime<Utc>,
    pub closed_at: Option<DateTime<Utc>>,
    pub price: Option<i64>,
}

impl ParkingSession {
    pub fn start(
        id: Uuid,
        plate: String,
        tariff_policy_id: i64,
        checkin_date: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            plate,
            tariff_policy_id,
            started_at: checkin_date,
            closed_at: None,
            price: None,
        }
    }

    pub fn close(&mut self, checkout_date: DateTime<Utc>, price: i64) -> Result<(), ModelError> {
        if self.closed_at.is_some() {
            return Err(ModelError::SessionAlreadyClosed);
        }
        if self.started_at > checkout_date {
            return Err(ModelError::InvalidCheckoutDate);
        }

        self.closed_at = Some(checkout_date);
        self.price = Some(price);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr as _;

    use chrono::{DateTime, Duration};
    use uuid::Uuid;

    use crate::domain::{error::ModelError, model::tariff_policy::TariffPolicy};

    use super::ParkingSession;

    #[test]
    fn creates_session_successfully() {
        let parking_session_id = Uuid::new_v4();
        let plate = String::from("XXX-0000");
        let tariff_policy_id: i64 = 1;
        let checkin_date = DateTime::from_str("2025-01-01T12:00:00.000Z").unwrap();
        let parking_session = ParkingSession::start(
            parking_session_id,
            plate.clone(),
            tariff_policy_id,
            checkin_date,
        );

        assert_eq!(parking_session.plate, plate);
        assert_eq!(parking_session.started_at, checkin_date);
    }

    #[test]
    fn returns_error_if_session_already_closed() {
        let parking_session_id = Uuid::new_v4();
        let plate = String::from("XXX-0000");
        let tariff_policy_id: i64 = 1;
        let checkin_date = DateTime::from_str("2025-01-01T12:00:00.000Z").unwrap();
        let mut parking_session = ParkingSession::start(
            parking_session_id,
            plate.clone(),
            tariff_policy_id,
            checkin_date,
        );

        let tariff_policy = TariffPolicy {
            id: 1,
            first_hour_rate: 500,
            grace_period: Duration::minutes(15),
            hourly_rate: 200,
        };

        let checkout_date = DateTime::from_str("2025-01-01T12:59:00.000Z").unwrap();
        let price = tariff_policy.calculate(parking_session.started_at, checkout_date);
        parking_session.close(checkout_date, price).unwrap();

        let result = parking_session.close(checkout_date, price);

        assert_eq!(result, Err(ModelError::SessionAlreadyClosed))
    }

    #[test]
    fn returns_error_if_checkout_before_checkin() {
        let parking_session_id = Uuid::new_v4();
        let plate = String::from("XXX-0000");
        let tariff_policy_id: i64 = 1;
        let checkin_date = DateTime::from_str("2025-01-01T12:00:00.000Z").unwrap();
        let mut parking_session = ParkingSession::start(
            parking_session_id,
            plate.clone(),
            tariff_policy_id,
            checkin_date,
        );

        let tariff_policy = TariffPolicy {
            id: 1,
            first_hour_rate: 500,
            grace_period: Duration::minutes(15),
            hourly_rate: 200,
        };

        let checkout_date = DateTime::from_str("2025-01-01T11:59:00.000Z").unwrap();
        let price = tariff_policy.calculate(parking_session.started_at, checkout_date);
        let result = parking_session.close(checkout_date, price);

        assert_eq!(result, Err(ModelError::InvalidCheckoutDate))
    }

    #[test]
    fn closes_session_and_calculates_price() {
        let parking_session_id = Uuid::new_v4();
        let plate = String::from("XXX-0000");
        let tariff_policy_id: i64 = 1;
        let checkin_date = DateTime::from_str("2025-01-01T12:00:00.000Z").unwrap();
        let mut parking_session = ParkingSession::start(
            parking_session_id,
            plate.clone(),
            tariff_policy_id,
            checkin_date,
        );

        let tariff_policy = TariffPolicy {
            id: 1,
            first_hour_rate: 500,
            grace_period: Duration::minutes(15),
            hourly_rate: 200,
        };

        let checkout_date = DateTime::from_str("2025-01-01T12:59:00.000Z").unwrap();
        let price = tariff_policy.calculate(parking_session.started_at, checkout_date);
        parking_session.close(checkout_date, price).unwrap();

        let expected_price: i64 = 500;
        assert_eq!(parking_session.price, Some(expected_price));
        assert_eq!(parking_session.closed_at, Some(checkout_date));
    }

    #[test]
    fn closes_session_and_applies_grace_period_pricing() {
        let parking_session_id = Uuid::new_v4();
        let plate = String::from("XXX-0000");
        let tariff_policy_id: i64 = 1;
        let checkin_date = DateTime::from_str("2025-01-01T12:00:00.000Z").unwrap();
        let mut parking_session = ParkingSession::start(
            parking_session_id,
            plate.clone(),
            tariff_policy_id,
            checkin_date,
        );

        let tariff_policy = TariffPolicy {
            id: 1,
            first_hour_rate: 500,
            grace_period: Duration::minutes(15),
            hourly_rate: 200,
        };

        let checkout_date = DateTime::from_str("2025-01-01T12:15:00.000Z").unwrap();
        let price = tariff_policy.calculate(parking_session.started_at, checkout_date);
        parking_session.close(checkout_date, price).unwrap();

        let expected_price: i64 = 0;
        assert_eq!(parking_session.price, Some(expected_price));
    }

    #[test]
    fn closes_session_and_calculates_price_with_extra_hours() {
        let parking_session_id = Uuid::new_v4();
        let plate = String::from("XXX-0000");
        let tariff_policy_id: i64 = 1;
        let checkin_date = DateTime::from_str("2025-01-01T12:00:00.000Z").unwrap();
        let mut parking_session = ParkingSession::start(
            parking_session_id,
            plate.clone(),
            tariff_policy_id,
            checkin_date,
        );

        let tariff_policy = TariffPolicy {
            id: 1,
            first_hour_rate: 500,
            grace_period: Duration::minutes(15),
            hourly_rate: 200,
        };

        let checkout_date = DateTime::from_str("2025-01-01T13:00:00.000Z").unwrap();
        let price = tariff_policy.calculate(parking_session.started_at, checkout_date);
        parking_session.close(checkout_date, price).unwrap();

        let expected_price: i64 = 700;
        assert_eq!(parking_session.price, Some(expected_price));
    }
}
