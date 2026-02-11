//! Search functionality for IdeaVault
//!
//! Provides comprehensive search across ideas, projects, and tags with filtering
//! and relevance ranking capabilities.

use crate::models::{Idea, Project, Tag};
use crate::storage::Storage;
use anyhow::{Context, Result};
use chrono::{DateTime, Utc};

/// Search result with relevance score
#[derive(Debug, Clone)]
pub struct SearchResult {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub entity_type: EntityType,
    pub status: String,
    pub relevance_score: f32,
    pub created_at: DateTime<Utc>,
    pub snippet: Option<String>,
    pub tags: Vec<String>,
}

/// Entity types that can be searched
#[derive(Debug, Clone, PartialEq)]
pub enum EntityType {
    Idea,
    Project,
    Tag,
}

impl std::fmt::Display for EntityType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EntityType::Idea => write!(f, "Idea"),
            EntityType::Project => write!(f, "Project"),
            EntityType::Tag => write!(f, "Tag"),
        }
    }
}

/// Search filters
#[derive(Debug, Clone)]
pub struct SearchFilters {
    pub entity_types: Vec<EntityType>,
    pub status_filter: Option<String>,
    pub tags_filter: Vec<String>,
    pub date_from: Option<DateTime<Utc>>,
    pub date_to: Option<DateTime<Utc>>,
}

impl Default for SearchFilters {
    fn default() -> Self {
        Self {
            entity_types: vec![EntityType::Idea, EntityType::Project, EntityType::Tag],
            status_filter: None,
            tags_filter: Vec::new(),
            date_from: None,
            date_to: None,
        }
    }
}

/// Search engine for IdeaVault entities
pub struct SearchEngine {
    storage: Storage,
}

impl SearchEngine {
    pub fn new(storage: Storage) -> Self {
        Self { storage }
    }

    /// Perform a search with the given query and filters
    pub fn search(&self, query: &str, filters: SearchFilters) -> Result<Vec<SearchResult>> {
        let mut results = Vec::new();

        // Search ideas if requested
        if filters.entity_types.contains(&EntityType::Idea) {
            let ideas = self
                .storage
                .load_ideas()
                .context("Failed to load ideas for search")?;
            for idea in ideas {
                if self.matches_idea_filters(&idea, &filters) {
                    if let Some(result) = self.search_in_idea(&idea, query) {
                        results.push(result);
                    }
                }
            }
        }

        // Search projects if requested
        if filters.entity_types.contains(&EntityType::Project) {
            let projects = self
                .storage
                .load_projects()
                .context("Failed to load projects for search")?;
            for project in projects {
                if self.matches_project_filters(&project, &filters) {
                    if let Some(result) = self.search_in_project(&project, query) {
                        results.push(result);
                    }
                }
            }
        }

        // Search tags if requested
        if filters.entity_types.contains(&EntityType::Tag) {
            let tags = self
                .storage
                .load_tags()
                .context("Failed to load tags for search")?;
            for tag in tags {
                if self.matches_tag_filters(&tag, &filters) {
                    if let Some(result) = self.search_in_tag(&tag, query) {
                        results.push(result);
                    }
                }
            }
        }

        // Sort by relevance score (descending)
        results.sort_by(|a, b| b.relevance_score.partial_cmp(&a.relevance_score).unwrap());

        Ok(results)
    }

    /// Search within an idea
    fn search_in_idea(&self, idea: &Idea, query: &str) -> Option<SearchResult> {
        let query_lower = query.to_lowercase();
        let mut score = 0.0f32;
        let mut snippet = None;

        // Check title match
        let title_lower = idea.title.to_lowercase();
        if title_lower.contains(&query_lower) {
            score += if idea.title.to_lowercase() == query_lower {
                100.0 // Exact title match
            } else if title_lower.starts_with(&query_lower) {
                80.0 // Title starts with query
            } else {
                60.0 // Title contains query
            };
            snippet = Some(self.create_snippet(&idea.title, &query_lower));
        }

        // Check description match
        if let Some(ref description) = idea.description {
            let desc_lower = description.to_lowercase();
            if desc_lower.contains(&query_lower) {
                score += 40.0; // Description match
                if snippet.is_none() {
                    snippet = Some(self.create_snippet(description, &query_lower));
                }
            }
        }

        // Check tags match
        for tag in &idea.tags {
            let tag_lower = tag.to_lowercase();
            if tag_lower.contains(&query_lower) {
                score += 20.0; // Tag match
                if snippet.is_none() {
                    snippet = Some(format!("Tag: {}", tag));
                }
            }
        }

        if score > 0.0 {
            Some(SearchResult {
                id: idea.id.to_string(),
                title: idea.title.clone(),
                description: idea.description.clone(),
                entity_type: EntityType::Idea,
                status: format!("{:?}", idea.status),
                relevance_score: score,
                created_at: idea.created_at,
                snippet,
                tags: idea.tags.clone(),
            })
        } else {
            None
        }
    }

    /// Search within a project
    fn search_in_project(&self, project: &Project, query: &str) -> Option<SearchResult> {
        let query_lower = query.to_lowercase();
        let mut score = 0.0f32;
        let mut snippet = None;

        // Check title match
        let title_lower = project.title.to_lowercase();
        if title_lower.contains(&query_lower) {
            score += if project.title.to_lowercase() == query_lower {
                100.0 // Exact title match
            } else if title_lower.starts_with(&query_lower) {
                80.0 // Title starts with query
            } else {
                60.0 // Title contains query
            };
            snippet = Some(self.create_snippet(&project.title, &query_lower));
        }

        // Check description match
        if let Some(ref description) = project.description {
            let desc_lower = description.to_lowercase();
            if desc_lower.contains(&query_lower) {
                score += 40.0; // Description match
                if snippet.is_none() {
                    snippet = Some(self.create_snippet(description, &query_lower));
                }
            }
        }

        // Check milestone match
        if let Some(ref milestone) = project.milestone {
            let milestone_lower = milestone.to_lowercase();
            if milestone_lower.contains(&query_lower) {
                score += 30.0; // Milestone match
                if snippet.is_none() {
                    snippet = Some(format!("Milestone: {}", milestone));
                }
            }
        }

        if score > 0.0 {
            Some(SearchResult {
                id: project.id.to_string(),
                title: project.title.clone(),
                description: project.description.clone(),
                entity_type: EntityType::Project,
                status: format!("{:?}", project.status),
                relevance_score: score,
                created_at: project.created_at,
                snippet,
                tags: Vec::new(), // Projects don't have tags in current model
            })
        } else {
            None
        }
    }

    /// Search within a tag
    fn search_in_tag(&self, tag: &Tag, query: &str) -> Option<SearchResult> {
        let query_lower = query.to_lowercase();
        let tag_lower = tag.name.to_lowercase();

        if tag_lower.contains(&query_lower) {
            let score = if tag.name.to_lowercase() == query_lower {
                100.0 // Exact tag match
            } else if tag_lower.starts_with(&query_lower) {
                80.0 // Tag starts with query
            } else {
                60.0 // Tag contains query
            };

            Some(SearchResult {
                id: "tag".to_string(), // Tags don't have IDs in current model
                title: tag.name.clone(),
                description: tag.color.clone(), // Use color as description field
                entity_type: EntityType::Tag,
                status: "Active".to_string(), // Tags don't have status in current model
                relevance_score: score,
                created_at: chrono::Utc::now(), // Use current time since tags don't have timestamps
                snippet: Some(format!("Tag: {}", tag.name)),
                tags: Vec::new(),
            })
        } else {
            None
        }
    }

    /// Create a snippet showing where the query matches in the text
    fn create_snippet(&self, text: &str, query_lower: &str) -> String {
        let text_lower = text.to_lowercase();
        if let Some(pos) = text_lower.find(query_lower) {
            let start = pos.saturating_sub(50);
            let end = std::cmp::min(pos + query_lower.len() + 50, text.len());
            let snippet = &text[start..end];

            if start > 0 {
                format!("...{}", snippet)
            } else {
                snippet.to_string()
            }
        } else {
            text.to_string()
        }
    }

    /// Check if an idea matches the search filters
    fn matches_idea_filters(&self, idea: &Idea, filters: &SearchFilters) -> bool {
        // Status filter
        if let Some(ref status_filter) = filters.status_filter {
            let idea_status = format!("{:?}", idea.status).to_lowercase();
            if !idea_status.contains(&status_filter.to_lowercase()) {
                return false;
            }
        }

        // Tags filter
        if !filters.tags_filter.is_empty() {
            let idea_tags_lower: Vec<String> = idea.tags.iter().map(|t| t.to_lowercase()).collect();
            for filter_tag in &filters.tags_filter {
                if !idea_tags_lower
                    .iter()
                    .any(|t| t.contains(&filter_tag.to_lowercase()))
                {
                    return false;
                }
            }
        }

        // Date range filter
        if let Some(date_from) = filters.date_from {
            if idea.created_at < date_from {
                return false;
            }
        }

        if let Some(date_to) = filters.date_to {
            if idea.created_at > date_to {
                return false;
            }
        }

        true
    }

    /// Check if a project matches the search filters
    fn matches_project_filters(&self, project: &Project, filters: &SearchFilters) -> bool {
        // Status filter
        if let Some(ref status_filter) = filters.status_filter {
            let project_status = format!("{:?}", project.status).to_lowercase();
            if !project_status.contains(&status_filter.to_lowercase()) {
                return false;
            }
        }

        // Date range filter
        if let Some(date_from) = filters.date_from {
            if project.created_at < date_from {
                return false;
            }
        }

        if let Some(date_to) = filters.date_to {
            if project.created_at > date_to {
                return false;
            }
        }

        true
    }

    /// Check if a tag matches the search filters
    fn matches_tag_filters(&self, tag: &Tag, filters: &SearchFilters) -> bool {
        // Tags filter - if specified, only show tags that match
        if !filters.tags_filter.is_empty() {
            let tag_lower = tag.name.to_lowercase();
            let matches_any_filter = filters
                .tags_filter
                .iter()
                .any(|filter| tag_lower.contains(&filter.to_lowercase()));
            if !matches_any_filter {
                return false;
            }
        }

        // Tags don't have status or date filters in current model
        true
    }
}

/// Display search results in a formatted table
pub fn display_search_results(results: &[SearchResult]) {
    if results.is_empty() {
        println!("No results found.");
        return;
    }

    println!("Found {} result(s):\n", results.len());

    // Calculate column widths
    let _type_width = 7;
    let _id_width = 8;
    let max_title_width = 30;
    let _status_width = 12;
    let max_snippet_width = 50;

    for (i, result) in results.iter().enumerate() {
        println!(
            "{}. {} [{}] (ID: {})",
            i + 1,
            truncate_string(&result.title, max_title_width),
            result.entity_type,
            &result.id[..8]
        );

        if let Some(ref description) = result.description {
            println!(
                "   Desc: {}",
                truncate_string(description, max_snippet_width)
            );
        }

        println!(
            "   Status: {} | Created: {} | Score: {:.1}",
            result.status,
            result.created_at.format("%Y-%m-%d %H:%M"),
            result.relevance_score
        );

        if let Some(ref snippet) = result.snippet {
            println!("   Match: {}", truncate_string(snippet, max_snippet_width));
        }

        if !result.tags.is_empty() {
            println!("   Tags: {}", result.tags.join(", "));
        }

        println!();
    }
}

/// Truncate string to specified length with ellipsis if needed
fn truncate_string(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len.saturating_sub(3)])
    }
}

/// Parse date string into DateTime<Utc>
pub fn parse_date(date_str: &str) -> Result<DateTime<Utc>> {
    // Try different date formats
    let formats = [
        "%Y-%m-%d",
        "%Y-%m-%d %H:%M:%S",
        "%Y/%m/%d",
        "%Y/%m/%d %H:%M:%S",
        "%m/%d/%Y",
        "%m/%d/%Y %H:%M:%S",
    ];

    for format in &formats {
        if let Ok(naive_date) = chrono::NaiveDateTime::parse_from_str(date_str, format) {
            return Ok(DateTime::from_naive_utc_and_offset(naive_date, Utc));
        }

        if let Ok(naive_date) = chrono::NaiveDate::parse_from_str(date_str, format) {
            let naive_datetime = naive_date.and_hms_opt(0, 0, 0).unwrap();
            return Ok(DateTime::from_naive_utc_and_offset(naive_datetime, Utc));
        }
    }

    Err(anyhow::anyhow!(
        "Unable to parse date: {}. Expected formats: YYYY-MM-DD, YYYY/MM/DD, MM/DD/YYYY",
        date_str
    ))
}

use crate::cli::SearchArgs;

pub fn execute_search(args: SearchArgs) -> Result<()> {
    let storage = Storage::new()?;
    let engine = SearchEngine::new(storage);

    let mut filters = SearchFilters::default();

    if args.ideas {
        filters.entity_types = vec![EntityType::Idea];
    } else if args.projects {
        filters.entity_types = vec![EntityType::Project];
    } else if args.tags {
        filters.entity_types = vec![EntityType::Tag];
    }

    if let Some(ref status) = args.status {
        filters.status_filter = Some(status.clone());
    }

    filters.tags_filter = args.with_tags;

    if let Some(ref date_from) = args.date_from {
        filters.date_from = Some(parse_date(date_from)?);
    }

    if let Some(ref date_to) = args.date_to {
        filters.date_to = Some(parse_date(date_to)?);
    }

    let results = engine.search(&args.query, filters)?;

    display_search_results(&results);

    Ok(())
}
