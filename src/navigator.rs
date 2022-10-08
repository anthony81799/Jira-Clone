use anyhow::{anyhow, Context, Ok, Result};
use std::rc::Rc;

use crate::{
    db::JiraDatabase,
    models::Action,
    ui::{EpicDetail, HomePage, Page, Prompts, StoryDetail},
};

pub struct Navigator {
    pages: Vec<Box<dyn Page>>,
    prompts: Prompts,
    db: Rc<JiraDatabase>,
}

impl Navigator {
    pub fn new(db: Rc<JiraDatabase>) -> Self {
        Self {
            pages: vec![Box::new(HomePage { db: Rc::clone(&db) })],
            prompts: Prompts::new(),
            db,
        }
    }

    pub fn get_current_page(&self) -> Option<&Box<dyn Page>> {
        self.pages.last()
    }

    pub fn handle_action(&mut self, action: Action) -> Result<()> {
        match action {
            Action::NavigateToEpicDetail { epic_id } => {
                self.pages.push(Box::new(EpicDetail {
                    epic_id,
                    db: Rc::clone(&self.db),
                }));
            }
            Action::NavigateToStoryDetail { epic_id, story_id } => {
                self.pages.push(Box::new(StoryDetail {
                    epic_id,
                    story_id,
                    db: Rc::clone(&self.db),
                }));
            }
            Action::NavigateToPreviousPage => {
                if !self.pages.is_empty() {
                    self.pages.pop();
                }
            }
            Action::CreateEpic => {
                let epic = (self.prompts.create_epic)();
                self.db
                    .create_epic(epic)
                    .with_context(|| anyhow!("failed to create epic!"))?;
            }
            Action::UpdateEpicStatus { epic_id } => {
                let status = (self.prompts.update_status)();

                if let Some(status) = status {
                    self.db
                        .update_epic_status(epic_id, status)
                        .with_context(|| anyhow!("failed to update epic!"))?;
                }
            }
            Action::DeleteEpic { epic_id } => {
                if (self.prompts.delete_epic)() {
                    self.db
                        .delete_epic(epic_id)
                        .with_context(|| anyhow!("failed to delete epic!"))?;

                    if !self.pages.is_empty() {
                        self.pages.pop();
                    }
                }
            }
            Action::CreateStory { epic_id } => {
                let story = (self.prompts.create_story)();
                self.db
                    .create_story(story, epic_id)
                    .with_context(|| anyhow!("failed to create story!"))?;
            }
            Action::UpdateStoryStatus { story_id } => {
                let status = (self.prompts.update_status)();

                if let Some(status) = status {
                    self.db
                        .update_story_status(story_id, status)
                        .with_context(|| anyhow!("failed to update story!"))?;
                }
            }
            Action::DeleteStory { epic_id, story_id } => {
                if (self.prompts.delete_story)() {
                    self.db
                        .delete_story(epic_id, story_id)
                        .with_context(|| anyhow!("failed to delete story!"))?;

                    if !self.pages.is_empty() {
                        self.pages.pop();
                    }
                }
            }
            Action::Exit => self.pages.clear(),
        }

        Ok(())
    }
}
