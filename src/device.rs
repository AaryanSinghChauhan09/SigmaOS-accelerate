//! Acceleration device definitions.

/// Device type
#[derive(Debug, Clone, PartialEq)]
pub enum DeviceType {
    /// GPU device
    GPU,
    /// NPU device
    NPU,
    /// DSP device
    DSP,
    /// VPU device
    VPU,
    /// FPGA device
    FPGA,
}

/// Device state
#[derive(Debug, Clone, PartialEq)]
pub enum DeviceState {
    /// Available device
    Available,
    /// Busy device
    Busy,
    /// Error device
    Error,
    /// Disabled device
    Disabled,
}

/// Device info
#[derive(Debug, Clone, Default)]
pub struct DeviceInfo {
    /// Device name
    pub name: String,
    /// Device vendor
    pub vendor: String,
    /// Device description
    pub description: String,
    /// Device driver version
    pub driver_version: String,
    /// Device firmware version
    pub firmware_version: String,
    /// Device serial number
    pub serial_number: String,
}

/// Acceleration device
#[derive(Debug, Clone)]
pub struct AccelDevice {
    /// Device ID
    pub id: String,
    /// Device type
    pub device_type: DeviceType,
    /// Device state
    pub state: DeviceState,
    /// Device info
    pub info: DeviceInfo,
    /// Device compute capability
    pub compute_capability: Option<super::capability::ComputeCapability>,
    /// Device features
    pub features: super::capability::FeatureSet,
    /// Device clock info
    pub clock_info: Option<super::capability::ClockInfo>,
    /// Device memory info
    pub memory_info: Option<super::memory::MemoryInfo>,
    /// Device created timestamp
    pub created_at: u64,
    /// Device last used timestamp
    pub last_used: u64,
    /// Device utilization percentage
    pub utilization: f64,
    /// Device temperature in Celsius
    pub temperature: f64,
}

impl AccelDevice {
    /// Create new acceleration device
    pub fn new(id: String, device_type: DeviceType) -> Self {
        Self {
            id,
            device_type,
            state: DeviceState::Available,
            info: DeviceInfo::default(),
            compute_capability: None,
            features: super::capability::FeatureSet::new(),
            clock_info: None,
            memory_info: None,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            last_used: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            utilization: 0.0,
            temperature: 0.0,
        }
    }
    
    /// Set device info
    pub fn set_info(&mut self, info: DeviceInfo) {
        self.info = info;
        self.touch();
    }
    
    /// Set compute capability
    pub fn set_compute_capability(&mut self, cap: super::capability::ComputeCapability) {
        self.compute_capability = Some(cap);
        self.touch();
    }
    
    /// Set clock info
    pub fn set_clock_info(&mut self, clock: super::capability::ClockInfo) {
        self.clock_info = Some(clock);
        self.touch();
    }
    
    /// Set memory info
    pub fn set_memory_info(&mut self, memory: super::memory::MemoryInfo) {
        self.memory_info = Some(memory);
        self.touch();
    }
    
    /// Set state
    pub fn set_state(&mut self, state: DeviceState) {
        self.state = state;
        self.touch();
    }
    
    /// Set utilization
    pub fn set_utilization(&mut self, utilization: f64) {
        self.utilization = utilization.clamp(0.0, 100.0);
        self.touch();
    }
    
    /// Set temperature
    pub fn set_temperature(&mut self, temperature: f64) {
        self.temperature = temperature;
        self.touch();
    }
    
    /// Touch device (update last used)
    pub fn touch(&mut self) {
        self.last_used = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }
    
    /// Check if device is available
    pub fn is_available(&self) -> bool {
        self.state == DeviceState::Available
    }
    
    /// Check if device is busy
    pub fn is_busy(&self) -> bool {
        self.state == DeviceState::Busy
    }
    
    /// Check if device has error
    pub fn has_error(&self) -> bool {
        self.state == DeviceState::Error
    }
    
    /// Check if device is disabled
    pub fn is_disabled(&self) -> bool {
        self.state == DeviceState::Disabled
    }
    
    /// Get device age in seconds
    pub fn get_age_seconds(&self) -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() - self.created_at
    }
    
    /// Get time since last used in seconds
    pub fn get_time_since_last_used(&self) -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() - self.last_used
    }
    
    /// Check if device has compute capability
    pub fn has_compute_capability(&self) -> bool {
        self.compute_capability.is_some()
    }
    
    /// Check if device has memory info
    pub fn has_memory_info(&self) -> bool {
        self.memory_info.is_some()
    }
    
    /// Check if device has features
    pub fn has_features(&self) -> bool {
        !self.features.features.is_empty()
    }
    
    /// Get device type name
    pub fn get_type_name(&self) -> &str {
        match self.device_type {
            DeviceType::GPU => "GPU",
            DeviceType::NPU => "NPU",
            DeviceType::DSP => "DSP",
            DeviceType::VPU => "VPU",
            DeviceType::FPGA => "FPGA",
        }
    }
}
