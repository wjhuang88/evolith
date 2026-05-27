//! Skill registry (placeholder)

use async_trait::async_trait;
use common::error::Result;
use domain::repository::SkillRepository;
use domain::{NewSkill, Skill, SkillFilter, UpdateSkill};
use uuid::Uuid;

pub struct SkillRegistry;

#[async_trait]
impl SkillRepository for SkillRegistry {
    async fn create(&self, _skill: NewSkill, _owner_id: Uuid, _tenant_id: Uuid) -> Result<Skill> {
        todo!("Skill registry not implemented")
    }

    async fn find_by_id(&self, _id: Uuid) -> Result<Option<Skill>> {
        todo!("Skill registry not implemented")
    }

    async fn find_by_name_and_version(&self, _name: &str, _version: &str) -> Result<Option<Skill>> {
        todo!("Skill registry not implemented")
    }

    async fn find_all(&self, _filter: SkillFilter) -> Result<Vec<Skill>> {
        Ok(Vec::new())
    }

    async fn count(&self, _filter: &SkillFilter) -> Result<u32> {
        Ok(0)
    }

    async fn update(&self, _id: Uuid, _skill: UpdateSkill) -> Result<Skill> {
        todo!("Skill registry not implemented")
    }

    async fn delete(&self, _id: Uuid) -> Result<()> {
        todo!("Skill registry not implemented")
    }
}
