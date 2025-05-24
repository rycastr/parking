use chrono::TimeDelta;

pub struct TariffPolicy {
    pub id: i64,
    pub first_hour_rate: i64,
    pub grace_period: TimeDelta,
    pub hourly_rate: i64,
}

impl TariffPolicy {
    pub fn calculate(&self, parked_duration: TimeDelta) -> i64 {
        if self.grace_period >= parked_duration {
            return 0;
        }

        let parked_hours = parked_duration.num_hours();
        return self.first_hour_rate + (parked_hours * self.hourly_rate);
    }
}
