use chrono::{DateTime, TimeDelta, Utc};

pub struct TariffPolicy {
    pub id: i64,
    pub first_hour_rate: i64,
    pub grace_period: TimeDelta,
    pub hourly_rate: i64,
}

impl TariffPolicy {
    pub fn calculate(&self, checkin_date: DateTime<Utc>, checkout_date: DateTime<Utc>) -> i64 {
        let duration = checkout_date - checkin_date;
        if self.grace_period >= duration {
            return 0;
        }

        let parked_hours = duration.num_hours();
        return self.first_hour_rate + (parked_hours * self.hourly_rate);
    }
}
