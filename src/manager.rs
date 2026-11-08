//! Acceleration manager.

use std::collections::HashMap;
use crate::device::{AccelDevice, DeviceType, DeviceState};
use crate::capability::{ComputeCapability, FeatureSet, ClockInfo};
use crate::context::{AccelContext, ContextConfig, ContextState};
use crate::memory::{MemoryInfo, MemoryStats};

/// Manager options
#[derive(Debug, Clone)]
pub struct ManagerOptions {
    /// Max devices
    pub max_devices: usize,
    /// Max contexts per device
    pub max_contexts_per_device: usize,
    /// Enable device discovery
    pub enable_discovery: bool,
    /// Enable auto-scheduling
    pub enable_auto_scheduling: bool,
    /// Default context config
    pub default_context_config: ContextConfig,
    /// Enable statistics
    pub enable_stats: bool,
}

impl Default for ManagerOptions {
    fn default() -> Self {
        Self {
            max_devices: 16,
            max_contexts_per_device: 8,
            enable_discovery: true,
            enable_auto_scheduling: true,
            default_context_config: ContextConfig::default(),
            enable_stats: true,
        }
    }
}

/// Manager statistics
#[derive(Debug, Clone)]
pub struct ManagerStatistics {
    /// Total devices
    pub total_devices: usize,
    /// Total GPUs
    pub total_gpus: usize,
    /// Total NPUs
    pub total_npus: usize,
    /// Total contexts
    pub total_contexts: usize,
    /// Active contexts
    pub active_contexts: usize,
    /// Available devices
    pub available_devices: usize,
    /// Busy devices
    pub busy_devices: usize,
    /// Total memory allocated in MB
    pub total_memory_allocated_mb: u64,
    /// Manager uptime in seconds
    pub uptime: u64,
}

/// Acceleration manager
#[derive(Debug, Clone)]
pub struct AccelManager {
    /// Devices
    devices: HashMap<String, AccelDevice>,
    /// Contexts
    contexts: HashMap<String, AccelContext>,
    /// Memory stats per device
    device_memory_stats: HashMap<String, MemoryStats>,
    /// Options
    options: ManagerOptions,
    /// Statistics
    statistics: ManagerStatistics,
    /// Manager created timestamp
    pub created_at: u64,
}

impl Default for AccelManager {
    fn default() -> Self {
        Self {
            devices: HashMap::new(),
            contexts: HashMap::new(),
            device_memory_stats: HashMap::new(),
            options: ManagerOptions::default(),
            statistics: ManagerStatistics {
                total_devices: 0,
                total_gpus: 0,
                total_npus: 0,
                total_contexts: 0,
                active_contexts: 0,
                available_devices: 0,
                busy_devices: 0,
                total_memory_allocated_mb: 0,
                uptime: 0,
            },
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
}

impl AccelManager {
    /// Create new acceleration manager
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Create manager with options
    pub fn with_options(options: ManagerOptions) -> Self {
        Self {
            devices: HashMap::new(),
            contexts: HashMap::new(),
            device_memory_stats: HashMap::new(),
            options,
            statistics: ManagerStatistics {
                total_devices: 0,
                total_gpus: 0,
                total_npus: 0,
                total_contexts: 0,
                active_contexts: 0,
                available_devices: 0,
                busy_devices: 0,
                total_memory_allocated_mb: 0,
                uptime: 0,
            },
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
    
    /// Initialize manager
    pub fn initialize(&mut self) {
        self.update_statistics();
    }
    
    // Device methods
    
    /// Add device
    pub fn add_device(&mut self, id: String, device_type: DeviceType) -> String {
        let device = AccelDevice::new(id.clone(), device_type);
        self.devices.insert(id.clone(), device);
        self.device_memory_stats.insert(id.clone(), MemoryStats::new());
        self.update_statistics();
        id
    }
    
    /// Get device
    pub fn get_device(&self, device_id: &str) -> Option<&AccelDevice> {
        self.devices.get(device_id)
    }
    
    /// Get device mutably
    pub fn get_device_mut(&mut self, device_id: &str) -> Option<&mut AccelDevice> {
        self.devices.get_mut(device_id)
    }
    
    /// Get all devices
    pub fn list_devices(&self) -> Vec<&AccelDevice> {
        self.devices.values().collect()
    }
    
    /// Remove device
    pub fn remove_device(&mut self, device_id: &str) -> bool {
        // Remove associated contexts
        let context_ids: Vec<String> = self.contexts.values()
            .filter(|c| c.device_id == device_id)
            .map(|c| c.id.clone())
            .collect();
        for ctx_id in context_ids {
            self.contexts.remove(&ctx_id);
        }
        
        if self.devices.remove(device_id).is_some() {
            self.device_memory_stats.remove(device_id);
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Set device info
    pub fn set_device_info(&mut self, device_id: &str, info: crate::device::DeviceInfo) -> bool {
        if let Some(device) = self.devices.get_mut(device_id) {
            device.set_info(info);
            return true;
        }
        false
    }
    
    /// Set device compute capability
    pub fn set_device_compute_capability(&mut self, device_id: &str, major: u32, minor: u32) -> bool {
        if let Some(device) = self.devices.get_mut(device_id) {
            device.set_compute_capability(ComputeCapability::new(major, minor));
            return true;
        }
        false
    }
    
    /// Set device features
    pub fn set_device_features(&mut self, device_id: &str, features: FeatureSet) -> bool {
        if let Some(device) = self.devices.get_mut(device_id) {
            device.features = features;
            return true;
        }
        false
    }
    
    /// Set device clock info
    pub fn set_device_clock_info(&mut self, device_id: &str, clock: ClockInfo) -> bool {
        if let Some(device) = self.devices.get_mut(device_id) {
            device.set_clock_info(clock);
            return true;
        }
        false
    }
    
    /// Set device memory info
    pub fn set_device_memory_info(&mut self, device_id: &str, memory: MemoryInfo) -> bool {
        if let Some(device) = self.devices.get_mut(device_id) {
            device.set_memory_info(memory);
            return true;
        }
        false
    }
    
    /// Set device state
    pub fn set_device_state(&mut self, device_id: &str, state: DeviceState) -> bool {
        if let Some(device) = self.devices.get_mut(device_id) {
            device.set_state(state);
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Update device utilization
    pub fn update_device_utilization(&mut self, device_id: &str, utilization: f64) -> bool {
        if let Some(device) = self.devices.get_mut(device_id) {
            device.set_utilization(utilization);
            return true;
        }
        false
    }
    
    /// Update device temperature
    pub fn update_device_temperature(&mut self, device_id: &str, temperature: f64) -> bool {
        if let Some(device) = self.devices.get_mut(device_id) {
            device.set_temperature(temperature);
            return true;
        }
        false
    }
    
    /// Get devices by type
    pub fn get_devices_by_type(&self, device_type: &DeviceType) -> Vec<&AccelDevice> {
        self.devices.values()
            .filter(|d| d.device_type == *device_type)
            .collect()
    }
    
    /// Get devices by state
    pub fn get_devices_by_state(&self, state: &DeviceState) -> Vec<&AccelDevice> {
        self.devices.values()
            .filter(|d| d.state == *state)
            .collect()
    }
    
    /// Get available devices
    pub fn get_available_devices(&self) -> Vec<&AccelDevice> {
        self.get_devices_by_state(&DeviceState::Available)
    }
    
    /// Get busy devices
    pub fn get_busy_devices(&self) -> Vec<&AccelDevice> {
        self.get_devices_by_state(&DeviceState::Busy)
    }
    
    /// Search devices
    pub fn search_devices(&self, query: &str) -> Vec<&AccelDevice> {
        let query_lower = query.to_lowercase();
        self.devices.values()
            .filter(|d| d.id.to_lowercase().contains(&query_lower) ||
                       d.info.name.to_lowercase().contains(&query_lower) ||
                       d.info.vendor.to_lowercase().contains(&query_lower))
            .collect()
    }
    
    // Context methods
    
    /// Create context
    pub fn create_context(&mut self, device_id: &str) -> Option<String> {
        let device = self.devices.get(device_id)?;
        
        // Check max contexts per device
        let context_count = self.contexts.values()
            .filter(|c| c.device_id == device_id)
            .count();
        if context_count >= self.options.max_contexts_per_device {
            return None;
        }
        
        let mut context = AccelContext::new(format!("ctx-{}", self.contexts.len() + 1));
        context.set_device_id(device_id.to_string());
        context.config = self.options.default_context_config.clone();
        
        let ctx_id = context.id.clone();
        self.contexts.insert(ctx_id.clone(), context);
        self.update_statistics();
        Some(ctx_id)
    }
    
    /// Get context
    pub fn get_context(&self, context_id: &str) -> Option<&AccelContext> {
        self.contexts.get(context_id)
    }
    
    /// Get context mutably
    pub fn get_context_mut(&mut self, context_id: &str) -> Option<&mut AccelContext> {
        self.contexts.get_mut(context_id)
    }
    
    /// Get all contexts
    pub fn list_contexts(&self) -> Vec<&AccelContext> {
        self.contexts.values().collect()
    }
    
    /// Delete context
    pub fn delete_context(&mut self, context_id: &str) -> bool {
        if self.contexts.remove(context_id).is_some() {
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Activate context
    pub fn activate_context(&mut self, context_id: &str) -> bool {
        if let Some(context) = self.contexts.get_mut(context_id) {
            if context.activate() {
                self.update_statistics();
                return true;
            }
        }
        false
    }
    
    /// Pause context
    pub fn pause_context(&mut self, context_id: &str) -> bool {
        if let Some(context) = self.contexts.get_mut(context_id) {
            return context.pause();
        }
        false
    }
    
    /// Destroy context
    pub fn destroy_context(&mut self, context_id: &str) -> bool {
        if let Some(context) = self.contexts.get_mut(context_id) {
            if context.destroy() {
                self.update_statistics();
                return true;
            }
        }
        false
    }
    
    /// Set context config
    pub fn set_context_config(&mut self, context_id: &str, config: ContextConfig) -> bool {
        if let Some(context) = self.contexts.get_mut(context_id) {
            context.set_config(config);
            return true;
        }
        false
    }
    
    /// Allocate memory to context
    pub fn allocate_context_memory(&mut self, context_id: &str, mb: u64) -> bool {
        if let Some(context) = self.contexts.get_mut(context_id) {
            if context.allocate_memory(mb) {
                // Update device memory stats
                if let Some(stats) = self.device_memory_stats.get_mut(&context.device_id) {
                    stats.record_alloc(mb * 1024 * 1024);
                }
                self.update_statistics();
                return true;
            }
        }
        false
    }
    
    /// Free memory from context
    pub fn free_context_memory(&mut self, context_id: &str, mb: u64) -> bool {
        if let Some(context) = self.contexts.get_mut(context_id) {
            if context.free_memory(mb) {
                if let Some(stats) = self.device_memory_stats.get_mut(&context.device_id) {
                    stats.record_free(mb * 1024 * 1024);
                }
                self.update_statistics();
                return true;
            }
        }
        false
    }
    
    /// Get contexts by device
    pub fn get_contexts_by_device(&self, device_id: &str) -> Vec<&AccelContext> {
        self.contexts.values()
            .filter(|c| c.device_id == device_id)
            .collect()
    }
    
    /// Get contexts by state
    pub fn get_contexts_by_state(&self, state: &ContextState) -> Vec<&AccelContext> {
        self.contexts.values()
            .filter(|c| c.state == *state)
            .collect()
    }
    
    /// Get active contexts
    pub fn get_active_contexts(&self) -> Vec<&AccelContext> {
        self.get_contexts_by_state(&ContextState::Active)
    }
    
    /// Get destroyed contexts
    pub fn get_destroyed_contexts(&self) -> Vec<&AccelContext> {
        self.get_contexts_by_state(&ContextState::Destroyed)
    }
    
    // Memory methods
    
    /// Allocate memory to device
    pub fn allocate_memory(&mut self, device_id: &str, mb: u64) -> bool {
        if let Some(device) = self.devices.get_mut(device_id) {
            if let Some(memory) = &mut device.memory_info {
                if memory.allocate(mb) {
                    if let Some(stats) = self.device_memory_stats.get_mut(device_id) {
                        stats.record_alloc(mb * 1024 * 1024);
                    }
                    self.update_statistics();
                    return true;
                }
            }
        }
        false
    }
    
    /// Free memory from device
    pub fn free_memory(&mut self, device_id: &str, mb: u64) -> bool {
        if let Some(device) = self.devices.get_mut(device_id) {
            if let Some(memory) = &mut device.memory_info {
                if memory.free(mb) {
                    if let Some(stats) = self.device_memory_stats.get_mut(device_id) {
                        stats.record_free(mb * 1024 * 1024);
                    }
                    self.update_statistics();
                    return true;
                }
            }
        }
        false
    }
    
    /// Get device memory info
    pub fn get_device_memory(&self, device_id: &str) -> Option<&MemoryInfo> {
        self.devices.get(device_id)
            .and_then(|d| d.memory_info.as_ref())
    }
    
    /// Get device memory stats
    pub fn get_device_memory_stats(&self, device_id: &str) -> Option<&MemoryStats> {
        self.device_memory_stats.get(device_id)
    }
    
    /// Get device memory stats mutably
    pub fn get_device_memory_stats_mut(&mut self, device_id: &str) -> Option<&mut MemoryStats> {
        self.device_memory_stats.get_mut(device_id)
    }
    
    /// Get all memory stats
    pub fn get_all_memory_stats(&self) -> Vec<(&String, &MemoryStats)> {
        self.device_memory_stats.iter().collect()
    }
    
    /// Reset device memory stats
    pub fn reset_device_memory_stats(&mut self, device_id: &str) -> bool {
        if let Some(stats) = self.device_memory_stats.get_mut(device_id) {
            stats.reset();
            return true;
        }
        false
    }
    
    // Statistics methods
    
    /// Get manager uptime
    pub fn get_uptime(&self) -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() - self.created_at
    }
    
    /// Update statistics
    fn update_statistics(&mut self) {
        self.statistics.total_devices = self.devices.len();
        self.statistics.total_gpus = self.devices.values()
            .filter(|d| d.device_type == DeviceType::GPU)
            .count();
        self.statistics.total_npus = self.devices.values()
            .filter(|d| d.device_type == DeviceType::NPU)
            .count();
        self.statistics.total_contexts = self.contexts.len();
        self.statistics.active_contexts = self.contexts.values()
            .filter(|c| c.state == ContextState::Active)
            .count();
        self.statistics.available_devices = self.devices.values()
            .filter(|d| d.state == DeviceState::Available)
            .count();
        self.statistics.busy_devices = self.devices.values()
            .filter(|d| d.state == DeviceState::Busy)
            .count();
        
        self.statistics.total_memory_allocated_mb = self.device_memory_stats.values()
            .map(|s| s.current_allocated_mb)
            .sum();
        
        self.statistics.uptime = self.get_uptime();
    }
    
    /// Get statistics
    pub fn get_statistics(&self) -> &ManagerStatistics {
        &self.statistics
    }
    
    /// Export manager data
    pub fn export_data(&self) -> ManagerExport {
        ManagerExport {
            devices: self.devices.values().cloned().collect(),
            contexts: self.contexts.values().cloned().collect(),
        }
    }
    
    /// Import manager data
    pub fn import_data(&mut self, export: ManagerExport) {
        self.devices.clear();
        for device in export.devices {
            let device_id = device.id.clone();
            self.devices.insert(device_id.clone(), device);
            self.device_memory_stats.insert(device_id, MemoryStats::new());
        }
        
        self.contexts.clear();
        for context in export.contexts {
            self.contexts.insert(context.id.clone(), context);
        }
        
        self.update_statistics();
    }
}

/// Manager export data
#[derive(Debug, Clone)]
pub struct ManagerExport {
    /// Devices
    pub devices: Vec<AccelDevice>,
    /// Contexts
    pub contexts: Vec<AccelContext>,
}
