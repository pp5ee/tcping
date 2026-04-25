use std::time::Instant;

/// Statistics tracking for TCP ping sessions
#[derive(Debug, Clone, Default)]
pub struct Statistics {
    /// Total number of probes sent
    pub total_probes: u32,

    /// Number of successful probes
    pub successful_probes: u32,

    /// Number of failed probes
    pub failed_probes: u32,

    /// Minimum RTT in milliseconds
    pub min_rtt: Option<f64>,

    /// Maximum RTT in milliseconds
    pub max_rtt: Option<f64>,

    /// Average RTT in milliseconds
    pub avg_rtt: Option<f64>,

    /// Total RTT sum for calculating average
    pub total_rtt: f64,

    /// Packet loss percentage
    pub packet_loss: f64,

    /// Start time of the session
    pub start_time: Option<Instant>,

    /// Longest successful streak
    pub longest_success_streak: u32,

    /// Longest failure streak
    pub longest_failure_streak: u32,

    /// Current success/failure streak
    pub current_streak: u32,

    /// Whether current streak is success (true) or failure (false)
    pub current_streak_success: bool,
}

impl Statistics {
    /// Create new statistics tracker
    pub fn new() -> Self {
        Self::default()
    }

    /// Record a successful probe with RTT measurement
    pub fn record_success(&mut self, rtt_ms: f64) {
        self.total_probes += 1;
        self.successful_probes += 1;

        // Update min/max RTT
        self.min_rtt = Some(self.min_rtt.map_or(rtt_ms, |min| min.min(rtt_ms)));
        self.max_rtt = Some(self.max_rtt.map_or(rtt_ms, |max| max.max(rtt_ms)));

        // Update average RTT
        self.total_rtt += rtt_ms;
        self.avg_rtt = Some(self.total_rtt / self.successful_probes as f64);

        // Update streak
        if self.current_streak_success {
            self.current_streak += 1;
        } else {
            self.current_streak = 1;
            self.current_streak_success = true;
        }
        self.longest_success_streak = self.longest_success_streak.max(self.current_streak);

        // Update packet loss
        self.update_packet_loss();
    }

    /// Record a failed probe
    pub fn record_failure(&mut self) {
        self.total_probes += 1;
        self.failed_probes += 1;

        // Update streak
        if !self.current_streak_success {
            self.current_streak += 1;
        } else {
            self.current_streak = 1;
            self.current_streak_success = false;
        }
        self.longest_failure_streak = self.longest_failure_streak.max(self.current_streak);

        // Update packet loss
        self.update_packet_loss();
    }

    /// Update packet loss percentage
    fn update_packet_loss(&mut self) {
        if self.total_probes > 0 {
            self.packet_loss = (self.failed_probes as f64 / self.total_probes as f64) * 100.0;
        }
    }

    /// Get current success streak
    pub fn current_streak(&self) -> u32 {
        if self.current_streak_success {
            self.current_streak
        } else {
            0
        }
    }

    /// Get current failure streak
    pub fn current_failure_streak(&self) -> u32 {
        if !self.current_streak_success {
            self.current_streak
        } else {
            0
        }
    }

    /// Start timing the session
    pub fn start_timing(&mut self) {
        self.start_time = Some(Instant::now());
    }

    /// Get session duration in seconds
    pub fn duration_seconds(&self) -> Option<f64> {
        self.start_time.map(|start| start.elapsed().as_secs_f64())
    }
}