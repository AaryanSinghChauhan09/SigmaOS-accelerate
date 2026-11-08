//! Compute capability and feature definitions.

use std::collections::HashSet;

/// Compute capability
#[derive(Debug, Clone, PartialEq)]
pub struct ComputeCapability {
    /// Major version
    pub major: u32,
    /// Minor version
    pub minor: u32,
}

impl ComputeCapability {
    /// Create new compute capability
    pub fn new(major: u32, minor: u32) -> Self {
        Self { major, minor }
    }
    
    /// Get version string
    pub fn version_string(&self) -> String {
        format!("{}.{}", self.major, self.minor)
    }
    
    /// Check if capability is at least the given version
    pub fn is_at_least(&self, major: u32, minor: u32) -> bool {
        self.major > major || (self.major == major && self.minor >= minor)
    }
}

/// Feature set
#[derive(Debug, Clone)]
pub struct FeatureSet {
    /// Feature names
    pub features: HashSet<String>,
}

impl FeatureSet {
    /// Create new feature set
    pub fn new() -> Self {
        Self {
            features: HashSet::new(),
        }
    }
    
    /// Add feature
    pub fn add(&mut self, feature: &str) {
        self.features.insert(feature.to_string());
    }
    
    /// Remove feature
    pub fn remove(&mut self, feature: &str) -> bool {
        self.features.remove(feature)
    }
    
    /// Check if feature exists
    pub fn has(&self, feature: &str) -> bool {
        self.features.contains(feature)
    }
    
    /// Get feature count
    pub fn get_count(&self) -> usize {
        self.features.len()
    }
    
    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.features.is_empty()
    }
    
    /// Clear all features
    pub fn clear(&mut self) {
        self.features.clear();
    }
    
    /// Get all features
    pub fn get_all(&self) -> &HashSet<String> {
        &self.features
    }
}

impl Default for FeatureSet {
    fn default() -> Self {
        Self::new()
    }
}

/// Clock info
#[derive(Debug, Clone)]
pub struct ClockInfo {
    /// Base clock in MHz
    pub base_mhz: u32,
    /// Boost clock in MHz
    pub boost_mhz: u32,
    /// Current clock in MHz
    pub current_mhz: u32,
    /// Memory clock in MHz
    pub memory_mhz: u32,
    /// Created timestamp
    pub created_at: u64,
}

impl ClockInfo {
    /// Create new clock info
    pub fn new(base_mhz: u32, boost_mhz: u32, current_mhz: u32) -> Self {
        Self {
            base_mhz,
            boost_mhz,
            current_mhz,
            memory_mhz: 0,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
    
    /// Set memory clock
    pub fn set_memory_clock(&mut self, memory_mhz: u32) {
        self.memory_mhz = memory_mhz;
    }
    
    /// Update current clock
    pub fn update_current(&mut self, current_mhz: u32) {
        self.current_mhz = current_mhz;
    }
    
    /// Get clock ratio (current/base)
    pub fn get_clock_ratio(&self) -> f64 {
        if self.base_mhz == 0 {
            0.0
        } else {
            self.current_mhz as f64 / self.base_mhz as f64
        }
    }
    
    /// Check if boosting
    pub fn is_boosting(&self) -> bool {
        self.current_mhz > self.base_mhz
    }
    
    /// Get clock info age in seconds
    pub fn get_age_seconds(&self) -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() - self.created_at
    }
}
