// SPDX-FileCopyrightText: 2026 Parth Shailesh Thite
// SPDX-License-Identifier: MIT OR Apache-2.0

pub struct Telemetry {
    pub sync_beacons: u32,
    pub tx_successes: u32,
    pub last_tx_scheduled_timestamp_ticks: Option<u64>,
    pub rx_attempts: u32,
    pub rx_data_packets: u32,
    pub last_rx_timestamp_ticks: Option<u64>,
}

impl Telemetry {
    pub const fn new() -> Self {
        Self {
            sync_beacons: 0,
            tx_successes: 0,
            last_tx_scheduled_timestamp_ticks: None,
            rx_attempts: 0,
            rx_data_packets: 0,
            last_rx_timestamp_ticks: None,
        }
    }

    pub fn record_rx_attempt(&mut self) {
        self.rx_attempts = self.rx_attempts.saturating_add(1);
    }
}

#[cfg(test)]
mod tests {
    use super::Telemetry;

    #[test]
    fn each_receive_attempt_increments_once() {
        let mut telemetry = Telemetry::new();
        assert_eq!(telemetry.rx_attempts, 0);

        telemetry.record_rx_attempt();
        assert_eq!(telemetry.rx_attempts, 1);

        telemetry.record_rx_attempt();
        assert_eq!(telemetry.rx_attempts, 2);
    }
}

impl Telemetry {
    pub fn record_rx_data(&mut self, timestamp_ticks: u64) {
        self.rx_data_packets = self.rx_data_packets.saturating_add(1);
        self.last_rx_timestamp_ticks = Some(timestamp_ticks);
    }
}

#[cfg(test)]
mod receive_data_tests {
    use super::Telemetry;

    #[test]
    fn receive_attempt_counter_saturates() {
        let mut telemetry = Telemetry::new();
        telemetry.rx_attempts = u32::MAX;
        telemetry.record_rx_attempt();
        assert_eq!(telemetry.rx_attempts, u32::MAX);
    }

    #[test]
    fn decoded_packets_update_count_and_latest_timestamp() {
        let mut telemetry = Telemetry::new();
        assert_eq!(telemetry.last_rx_timestamp_ticks, None);

        telemetry.record_rx_data(0);
        assert_eq!(telemetry.rx_data_packets, 1);
        assert_eq!(telemetry.last_rx_timestamp_ticks, Some(0));

        telemetry.record_rx_data(u64::MAX);
        assert_eq!(telemetry.rx_data_packets, 2);
        assert_eq!(telemetry.last_rx_timestamp_ticks, Some(u64::MAX));
        assert_eq!(telemetry.rx_attempts, 0);
    }

    #[test]
    fn decoded_count_saturates_but_timestamp_updates() {
        let mut telemetry = Telemetry::new();
        telemetry.rx_data_packets = u32::MAX;

        telemetry.record_rx_data(42);

        assert_eq!(telemetry.rx_data_packets, u32::MAX);
        assert_eq!(telemetry.last_rx_timestamp_ticks, Some(42));
    }
}

impl Telemetry {
    pub fn record_sync_beacon(&mut self) {
        self.sync_beacons = self.sync_beacons.saturating_add(1);
    }

    pub fn record_tx_success(&mut self, scheduled_timestamp_ticks: u64) {
        self.tx_successes = self.tx_successes.saturating_add(1);
        self.last_tx_scheduled_timestamp_ticks = Some(scheduled_timestamp_ticks);
    }
}

#[cfg(test)]
mod beacon_and_tx_tests {
    use super::Telemetry;

    #[test]
    fn beacons_increment_without_changing_other_events() {
        let mut t = Telemetry::new();
        t.record_sync_beacon();
        assert_eq!(t.sync_beacons, 1);
        t.record_sync_beacon();
        assert_eq!(t.sync_beacons, 2);
        assert_eq!(t.rx_attempts, 0);
        assert_eq!(t.rx_data_packets, 0);
        assert_eq!(t.tx_successes, 0);
        assert_eq!(t.last_rx_timestamp_ticks, None);
        assert_eq!(t.last_tx_scheduled_timestamp_ticks, None);
    }

    #[test]
    fn beacon_counter_saturates() {
        let mut t = Telemetry::new();
        t.sync_beacons = u32::MAX;
        t.record_sync_beacon();
        assert_eq!(t.sync_beacons, u32::MAX);
    }

    #[test]
    fn successful_transmits_update_count_and_scheduled_timestamp() {
        let mut t = Telemetry::new();
        assert_eq!(t.last_tx_scheduled_timestamp_ticks, None);

        t.record_tx_success(0);
        assert_eq!(t.tx_successes, 1);
        assert_eq!(t.last_tx_scheduled_timestamp_ticks, Some(0));

        t.record_tx_success(u64::MAX);
        assert_eq!(t.tx_successes, 2);
        assert_eq!(t.last_tx_scheduled_timestamp_ticks, Some(u64::MAX));
        assert_eq!(t.rx_attempts, 0);
        assert_eq!(t.rx_data_packets, 0);
        assert_eq!(t.sync_beacons, 0);
        assert_eq!(t.last_rx_timestamp_ticks, None);
    }

    #[test]
    fn transmit_count_saturates_but_timestamp_updates() {
        let mut t = Telemetry::new();
        t.tx_successes = u32::MAX;
        t.record_tx_success(42);
        assert_eq!(t.tx_successes, u32::MAX);
        assert_eq!(t.last_tx_scheduled_timestamp_ticks, Some(42));
    }
}
