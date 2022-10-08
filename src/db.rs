use std::fs;

use anyhow::{anyhow, Result};

use crate::models::{DBState, Epic, Status, Story};

pub struct JiraDatabase {
    database: Box<dyn Database>,
}

impl JiraDatabase {
    pub fn new(file_path: String) -> Self {
        Self {
            database: Box::new(JSONFileDatabase { file_path }),
        }
    }

    pub fn read_db(&self) -> Result<DBState> {
        self.database.read_db()
    }

    pub fn create_epic(&self, epic: Epic) -> Result<u32> {
        let mut parsed = self.read_db()?;
        parsed.last_item_id += 1;
        parsed.epics.insert(parsed.last_item_id, epic);
        self.database.write_db(&parsed)?;
        Ok(parsed.last_item_id)
    }

    pub fn create_story(&self, story: Story, epic_id: u32) -> Result<u32> {
        let mut parsed = self.read_db()?;
        parsed.last_item_id += 1;
        parsed.stories.insert(parsed.last_item_id, story);
        parsed
            .epics
            .get_mut(&epic_id)
            .ok_or_else(|| anyhow!("could not find epic in database!"))?
            .stories
            .push(parsed.last_item_id);
        self.database.write_db(&parsed)?;
        Ok(parsed.last_item_id)
    }

    pub fn delete_epic(&self, epic_id: u32) -> Result<()> {
        let mut parsed = self.read_db()?;

        for story_id in &parsed
            .epics
            .get(&epic_id)
            .ok_or_else(|| anyhow!("could not find epic in database!"))?
            .stories
        {
            parsed.stories.remove(story_id);
        }

        parsed.epics.remove(&epic_id);

        self.database.write_db(&parsed)?;
        Ok(())
    }

    pub fn delete_story(&self, epic_id: u32, story_id: u32) -> Result<()> {
        let mut parsed = self.read_db()?;

        let epic = parsed
            .epics
            .get_mut(&epic_id)
            .ok_or_else(|| anyhow!("could not find epic in database!"))?;

        let story_index = epic
            .stories
            .iter()
            .position(|id| id == &story_id)
            .ok_or_else(|| anyhow!("story id not found in epic stories vector"))?;
        epic.stories.remove(story_index);

        parsed.stories.remove(&story_id);

        self.database.write_db(&parsed)?;
        Ok(())
    }

    pub fn update_epic_status(&self, epic_id: u32, status: Status) -> Result<()> {
        let mut parsed = self.read_db()?;

        parsed
            .epics
            .get_mut(&epic_id)
            .ok_or_else(|| anyhow!("could not find epic in database!"))?
            .status = status;

        self.database.write_db(&parsed)?;
        Ok(())
    }

    pub fn update_story_status(&self, story_id: u32, status: Status) -> Result<()> {
        let mut parsed = self.read_db()?;

        parsed
            .stories
            .get_mut(&story_id)
            .ok_or_else(|| anyhow!("could not find story in database!"))?
            .status = status;

        self.database.write_db(&parsed)?;
        Ok(())
    }
}

trait Database {
    fn read_db(&self) -> Result<DBState>;
    fn write_db(&self, db_state: &DBState) -> Result<()>;
}

struct JSONFileDatabase {
    pub file_path: String,
}

impl Database for JSONFileDatabase {
    fn read_db(&self) -> Result<DBState> {
        let db_content = fs::read_to_string(&self.file_path)?;
        let parsed: DBState = serde_json::from_str(&db_content)?;
        Ok(parsed)
    }

    fn write_db(&self, db_state: &DBState) -> Result<()> {
        fs::write(&self.file_path, &serde_json::to_vec(db_state)?)?;
        Ok(())
    }
}
