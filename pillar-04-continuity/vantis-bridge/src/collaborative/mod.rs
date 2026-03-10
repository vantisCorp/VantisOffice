//! Collaborative Conversion Module - Integration with Vantis Link (Pillar 03)
//!
//! Enables real-time collaborative format conversion where multiple users
//! can participate in the conversion pipeline, review results, and
//! approve conversions through Link sessions.
//!
//! # Architecture
//!
//! ```text
//! vantis-bridge (Pillar 04: Continuity)
//!     └── collaborative module
//!             └── vantis-link (Pillar 03: Sync)
//!                     ├── Conversion session management
//!                     ├── Real-time progress sync
//!                     └── Collaborative review
//! ```
//!
//! # Features
//! - Shared conversion sessions for team review
//! - Real-time conversion progress broadcasting
//! - Collaborative approval workflow
//! - Conversion history with change tracking
//! - Batch conversion with session coordination

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use vantis_link::core::{Session as LinkSession, User as LinkUser, UserRole};

use crate::core::{ConversionConfig, ConversionResult, Document, DocumentType};

/// Collaborative conversion manager
pub struct CollaborativeConversionManager {
    /// Link session for collaboration
    link_session: LinkSession,
    /// Conversion jobs
    jobs: HashMap<String, ConversionJob>,
    /// Conversion history
    history: Vec<ConversionRecord>,
    /// Manager configuration
    config: CollaborativeConfig,
}

/// Configuration for collaborative conversions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollaborativeConfig {
    /// Whether approval is required before conversion
    pub require_approval: bool,
    /// Minimum number of approvals needed
    pub min_approvals: usize,
    /// Whether to broadcast progress to all session users
    pub broadcast_progress: bool,
    /// Maximum concurrent conversions
    pub max_concurrent: usize,
    /// Auto-convert on approval
    pub auto_convert_on_approval: bool,
}

impl Default for CollaborativeConfig {
    fn default() -> Self {
        Self {
            require_approval: true,
            min_approvals: 1,
            broadcast_progress: true,
            max_concurrent: 5,
            auto_convert_on_approval: true,
        }
    }
}

/// A conversion job in the collaborative pipeline
#[derive(Debug, Clone)]
pub struct ConversionJob {
    /// Job ID
    pub id: String,
    /// Source document name
    pub source_name: String,
    /// Source document type
    pub source_type: DocumentType,
    /// Target document type
    pub target_type: DocumentType,
    /// Job status
    pub status: JobStatus,
    /// Conversion configuration
    pub config: ConversionConfig,
    /// Who submitted the job
    pub submitted_by: String,
    /// When the job was submitted
    pub submitted_at: DateTime<Utc>,
    /// Approvals received
    pub approvals: Vec<Approval>,
    /// Conversion progress (0-100)
    pub progress: u8,
    /// Result of conversion (if completed)
    pub result_summary: Option<String>,
    /// Warnings from conversion
    pub warnings: Vec<String>,
}

/// Status of a conversion job
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum JobStatus {
    /// Waiting for approval
    PendingApproval,
    /// Approved and queued
    Queued,
    /// Currently converting
    InProgress,
    /// Conversion completed successfully
    Completed,
    /// Conversion failed
    Failed,
    /// Job was cancelled
    Cancelled,
}

/// An approval for a conversion job
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Approval {
    /// User who approved
    pub user_id: String,
    /// User name
    pub user_name: String,
    /// When approved
    pub approved_at: DateTime<Utc>,
    /// Optional comment
    pub comment: Option<String>,
}

/// A record of a completed conversion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionRecord {
    /// Job ID
    pub job_id: String,
    /// Source document name
    pub source_name: String,
    /// Source type
    pub source_type: DocumentType,
    /// Target type
    pub target_type: DocumentType,
    /// Whether conversion was successful
    pub success: bool,
    /// When completed
    pub completed_at: DateTime<Utc>,
    /// Duration in milliseconds
    pub duration_ms: u64,
    /// Number of warnings
    pub warning_count: usize,
}

impl CollaborativeConversionManager {
    /// Create a new collaborative conversion manager
    pub fn new(document_id: &str, user_id: &str, user_name: &str) -> Self {
        let mut link_session = LinkSession::new(document_id.to_string());
        let user = LinkUser::new(user_id.to_string(), user_name.to_string());
        let _ = link_session.add_user(user);

        Self {
            link_session,
            jobs: HashMap::new(),
            history: Vec::new(),
            config: CollaborativeConfig::default(),
        }
    }

    /// Create with custom configuration
    pub fn with_config(
        document_id: &str,
        user_id: &str,
        user_name: &str,
        config: CollaborativeConfig,
    ) -> Self {
        let mut manager = Self::new(document_id, user_id, user_name);
        manager.config = config;
        manager
    }

    /// Get the configuration
    pub fn config(&self) -> &CollaborativeConfig {
        &self.config
    }

    /// Get the Link session
    pub fn session(&self) -> &LinkSession {
        &self.link_session
    }

    /// Add a collaborator to the conversion session
    pub fn add_collaborator(
        &mut self,
        user_id: &str,
        user_name: &str,
    ) -> Result<(), String> {
        let user = LinkUser::new(user_id.to_string(), user_name.to_string());
        self.link_session.add_user(user)
    }

    /// Submit a conversion job
    pub fn submit_job(
        &mut self,
        source_name: &str,
        source_type: DocumentType,
        target_type: DocumentType,
        submitted_by: &str,
        config: ConversionConfig,
    ) -> Result<String, String> {
        // Check concurrent limit
        let active_count = self
            .jobs
            .values()
            .filter(|j| {
                j.status == JobStatus::InProgress || j.status == JobStatus::Queued
            })
            .count();

        if active_count >= self.config.max_concurrent {
            return Err(format!(
                "Maximum concurrent conversions ({}) reached",
                self.config.max_concurrent
            ));
        }

        let job_id = uuid::Uuid::new_v4().to_string();
        let status = if self.config.require_approval {
            JobStatus::PendingApproval
        } else {
            JobStatus::Queued
        };

        let job = ConversionJob {
            id: job_id.clone(),
            source_name: source_name.to_string(),
            source_type,
            target_type,
            status,
            config,
            submitted_by: submitted_by.to_string(),
            submitted_at: Utc::now(),
            approvals: Vec::new(),
            progress: 0,
            result_summary: None,
            warnings: Vec::new(),
        };

        self.jobs.insert(job_id.clone(), job);
        Ok(job_id)
    }

    /// Approve a conversion job
    pub fn approve_job(
        &mut self,
        job_id: &str,
        user_id: &str,
        user_name: &str,
        comment: Option<String>,
    ) -> Result<JobStatus, String> {
        let job = self
            .jobs
            .get_mut(job_id)
            .ok_or_else(|| format!("Job {} not found", job_id))?;

        if job.status != JobStatus::PendingApproval {
            return Err(format!("Job {} is not pending approval", job_id));
        }

        // Check if user already approved
        if job.approvals.iter().any(|a| a.user_id == user_id) {
            return Err(format!("User {} already approved this job", user_id));
        }

        job.approvals.push(Approval {
            user_id: user_id.to_string(),
            user_name: user_name.to_string(),
            approved_at: Utc::now(),
            comment,
        });

        // Check if enough approvals
        if job.approvals.len() >= self.config.min_approvals {
            job.status = JobStatus::Queued;
        }

        Ok(job.status)
    }

    /// Start processing a queued job
    pub fn start_job(&mut self, job_id: &str) -> Result<(), String> {
        let job = self
            .jobs
            .get_mut(job_id)
            .ok_or_else(|| format!("Job {} not found", job_id))?;

        if job.status != JobStatus::Queued {
            return Err(format!("Job {} is not queued (status: {:?})", job_id, job.status));
        }

        job.status = JobStatus::InProgress;
        job.progress = 0;
        Ok(())
    }

    /// Update job progress
    pub fn update_progress(
        &mut self,
        job_id: &str,
        progress: u8,
    ) -> Result<(), String> {
        let job = self
            .jobs
            .get_mut(job_id)
            .ok_or_else(|| format!("Job {} not found", job_id))?;

        if job.status != JobStatus::InProgress {
            return Err(format!("Job {} is not in progress", job_id));
        }

        job.progress = progress.min(100);
        Ok(())
    }

    /// Complete a job with results
    pub fn complete_job(
        &mut self,
        job_id: &str,
        success: bool,
        summary: &str,
        warnings: Vec<String>,
    ) -> Result<(), String> {
        let job = self
            .jobs
            .get_mut(job_id)
            .ok_or_else(|| format!("Job {} not found", job_id))?;

        job.status = if success {
            JobStatus::Completed
        } else {
            JobStatus::Failed
        };
        job.progress = if success { 100 } else { job.progress };
        job.result_summary = Some(summary.to_string());
        job.warnings = warnings.clone();

        // Add to history
        self.history.push(ConversionRecord {
            job_id: job_id.to_string(),
            source_name: job.source_name.clone(),
            source_type: job.source_type,
            target_type: job.target_type,
            success,
            completed_at: Utc::now(),
            duration_ms: Utc::now()
                .signed_duration_since(job.submitted_at)
                .num_milliseconds() as u64,
            warning_count: warnings.len(),
        });

        Ok(())
    }

    /// Cancel a job
    pub fn cancel_job(&mut self, job_id: &str) -> Result<(), String> {
        let job = self
            .jobs
            .get_mut(job_id)
            .ok_or_else(|| format!("Job {} not found", job_id))?;

        if job.status == JobStatus::Completed || job.status == JobStatus::Failed {
            return Err(format!("Cannot cancel a finished job"));
        }

        job.status = JobStatus::Cancelled;
        Ok(())
    }

    /// Get a job by ID
    pub fn get_job(&self, job_id: &str) -> Option<&ConversionJob> {
        self.jobs.get(job_id)
    }

    /// Get all jobs
    pub fn all_jobs(&self) -> Vec<&ConversionJob> {
        self.jobs.values().collect()
    }

    /// Get jobs by status
    pub fn jobs_by_status(&self, status: JobStatus) -> Vec<&ConversionJob> {
        self.jobs
            .values()
            .filter(|j| j.status == status)
            .collect()
    }

    /// Get conversion history
    pub fn history(&self) -> &[ConversionRecord] {
        &self.history
    }

    /// Get the number of active collaborators
    pub fn collaborator_count(&self) -> usize {
        self.link_session.users.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_manager() -> CollaborativeConversionManager {
        CollaborativeConversionManager::new("bridge-session-001", "user1", "Alice")
    }

    #[test]
    fn test_create_manager() {
        let manager = create_test_manager();
        assert_eq!(manager.collaborator_count(), 1);
        assert!(manager.all_jobs().is_empty());
        assert!(manager.history().is_empty());
    }

    #[test]
    fn test_add_collaborator() {
        let mut manager = create_test_manager();
        manager.add_collaborator("user2", "Bob").unwrap();
        assert_eq!(manager.collaborator_count(), 2);
    }

    #[test]
    fn test_submit_job() {
        let mut manager = create_test_manager();
        let config = ConversionConfig::default();

        let job_id = manager
            .submit_job("report.docx", DocumentType::Docx, DocumentType::VantisWriter, "user1", config)
            .unwrap();

        let job = manager.get_job(&job_id).unwrap();
        assert_eq!(job.source_name, "report.docx");
        assert_eq!(job.status, JobStatus::PendingApproval);
    }

    #[test]
    fn test_submit_job_no_approval() {
        let config = CollaborativeConfig {
            require_approval: false,
            ..Default::default()
        };
        let mut manager = CollaborativeConversionManager::with_config(
            "session", "user1", "Alice", config,
        );

        let conv_config = ConversionConfig::default();
        let job_id = manager
            .submit_job("data.xlsx", DocumentType::Xlsx, DocumentType::VantisGrid, "user1", conv_config)
            .unwrap();

        let job = manager.get_job(&job_id).unwrap();
        assert_eq!(job.status, JobStatus::Queued);
    }

    #[test]
    fn test_approve_job() {
        let mut manager = create_test_manager();
        let config = ConversionConfig::default();

        let job_id = manager
            .submit_job("doc.docx", DocumentType::Docx, DocumentType::VantisWriter, "user1", config)
            .unwrap();

        let status = manager
            .approve_job(&job_id, "user1", "Alice", Some("Looks good".to_string()))
            .unwrap();

        assert_eq!(status, JobStatus::Queued);
    }

    #[test]
    fn test_duplicate_approval_rejected() {
        let mut manager = create_test_manager();
        let config = ConversionConfig::default();

        let job_id = manager
            .submit_job("doc.docx", DocumentType::Docx, DocumentType::VantisWriter, "user1", config)
            .unwrap();

        manager
            .approve_job(&job_id, "user1", "Alice", None)
            .unwrap();

        let result = manager.approve_job(&job_id, "user1", "Alice", None);
        assert!(result.is_err());
    }

    #[test]
    fn test_job_lifecycle() {
        let mut manager = create_test_manager();
        let config = ConversionConfig::default();

        // Submit
        let job_id = manager
            .submit_job("slides.pptx", DocumentType::Pptx, DocumentType::VantisCanvas, "user1", config)
            .unwrap();

        // Approve
        manager
            .approve_job(&job_id, "user1", "Alice", None)
            .unwrap();

        // Start
        manager.start_job(&job_id).unwrap();
        assert_eq!(manager.get_job(&job_id).unwrap().status, JobStatus::InProgress);

        // Update progress
        manager.update_progress(&job_id, 50).unwrap();
        assert_eq!(manager.get_job(&job_id).unwrap().progress, 50);

        // Complete
        manager
            .complete_job(&job_id, true, "Converted successfully", vec![])
            .unwrap();

        let job = manager.get_job(&job_id).unwrap();
        assert_eq!(job.status, JobStatus::Completed);
        assert_eq!(job.progress, 100);
        assert_eq!(manager.history().len(), 1);
    }

    #[test]
    fn test_cancel_job() {
        let mut manager = create_test_manager();
        let config = ConversionConfig::default();

        let job_id = manager
            .submit_job("doc.docx", DocumentType::Docx, DocumentType::VantisWriter, "user1", config)
            .unwrap();

        manager.cancel_job(&job_id).unwrap();
        assert_eq!(manager.get_job(&job_id).unwrap().status, JobStatus::Cancelled);
    }

    #[test]
    fn test_max_concurrent_limit() {
        let config = CollaborativeConfig {
            require_approval: false,
            max_concurrent: 2,
            ..Default::default()
        };
        let mut manager = CollaborativeConversionManager::with_config(
            "session", "user1", "Alice", config,
        );

        let conv_config = ConversionConfig::default();

        // Submit 2 jobs (both go to Queued)
        let id1 = manager
            .submit_job("a.docx", DocumentType::Docx, DocumentType::VantisWriter, "user1", conv_config.clone())
            .unwrap();
        let _id2 = manager
            .submit_job("b.docx", DocumentType::Docx, DocumentType::VantisWriter, "user1", conv_config.clone())
            .unwrap();

        // Third should fail
        let result = manager.submit_job(
            "c.docx", DocumentType::Docx, DocumentType::VantisWriter, "user1", conv_config,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_jobs_by_status() {
        let config = CollaborativeConfig {
            require_approval: false,
            ..Default::default()
        };
        let mut manager = CollaborativeConversionManager::with_config(
            "session", "user1", "Alice", config,
        );

        let conv_config = ConversionConfig::default();
        let _ = manager
            .submit_job("a.docx", DocumentType::Docx, DocumentType::VantisWriter, "user1", conv_config.clone())
            .unwrap();
        let _ = manager
            .submit_job("b.xlsx", DocumentType::Xlsx, DocumentType::VantisGrid, "user1", conv_config)
            .unwrap();

        let queued = manager.jobs_by_status(JobStatus::Queued);
        assert_eq!(queued.len(), 2);

        let pending = manager.jobs_by_status(JobStatus::PendingApproval);
        assert_eq!(pending.len(), 0);
    }

    #[test]
    fn test_failed_job_recorded_in_history() {
        let mut manager = create_test_manager();
        let config = ConversionConfig::default();

        let job_id = manager
            .submit_job("bad.docx", DocumentType::Docx, DocumentType::VantisWriter, "user1", config)
            .unwrap();

        manager.approve_job(&job_id, "user1", "Alice", None).unwrap();
        manager.start_job(&job_id).unwrap();
        manager
            .complete_job(&job_id, false, "Parse error", vec!["Corrupted file".to_string()])
            .unwrap();

        assert_eq!(manager.history().len(), 1);
        assert!(!manager.history()[0].success);
        assert_eq!(manager.history()[0].warning_count, 1);
    }
}