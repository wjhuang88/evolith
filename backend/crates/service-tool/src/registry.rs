//! Tool registry (placeholder)

use async_trait::async_trait;
use common::error::Result;
use domain::repository::ToolRepository;
use domain::{NewTool, Tool, ToolFilter, UpdateTool};
use uuid::Uuid;

pub struct ToolRegistry;

#[async_trait]
impl ToolRepository for ToolRegistry {
    async fn create(&self, _tool: NewTool, _owner_id: Uuid) -> Result<Tool> {
        todo!("Tool registry not implemented")
    }

    async fn find_by_id(&self, _id: Uuid) -> Result<Option<Tool>> {
        todo!("Tool registry not implemented")
    }

    async fn find_by_name(&self, _name: &str) -> Result<Option<Tool>> {
        todo!("Tool registry not implemented")
    }

    async fn find_all(&self, _filter: ToolFilter) -> Result<Vec<Tool>> {
        Ok(Vec::new())
    }

    async fn count(&self, _filter: &ToolFilter) -> Result<u32> {
        Ok(0)
    }

    async fn update(&self, _id: Uuid, _tool: UpdateTool) -> Result<Tool> {
        todo!("Tool registry not implemented")
    }

    async fn delete(&self, _id: Uuid) -> Result<()> {
        todo!("Tool registry not implemented")
    }
}
