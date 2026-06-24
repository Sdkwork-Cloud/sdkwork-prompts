use crate::domain::commands::*;
use crate::domain::events::PromptsDomainEvent;
use crate::domain::models::*;
use crate::domain::results::*;
use crate::error::PromptsServiceError;
use crate::integration::drive::{PromptsDrivePort, NoopPromptsDrivePort};
use crate::integration::notifications::{PromptsNotificationPort, NoopPromptsNotificationPort};
use crate::integration::search::{PromptsSearchPort, NoopPromptsSearchPort};
use crate::ports::repository::PromptsRepository;
use crate::value_objects::PromptsRequestContext;
use serde_json::{json, Value};

mod integration_hooks;

pub struct PromptsService<R: PromptsRepository> {
    repository: R,
    drive_port: Box<dyn PromptsDrivePort>,
    search_port: Box<dyn PromptsSearchPort>,
    notification_port: Box<dyn PromptsNotificationPort>,
}

const VALID_BODY_FORMATS: &[&str] = &["markdown", "html_sanitized", "rich_text_json"];

impl<R: PromptsRepository> PromptsService<R> {
    pub fn new(repository: R) -> Self {
        Self {
            repository,
            drive_port: Box::new(NoopPromptsDrivePort),
            search_port: Box::new(NoopPromptsSearchPort),
            notification_port: Box::new(NoopPromptsNotificationPort),
        }
    }

    pub fn new_with_ports(
        repository: R,
        drive_port: Box<dyn PromptsDrivePort>,
        search_port: Box<dyn PromptsSearchPort>,
        notification_port: Box<dyn PromptsNotificationPort>,
    ) -> Self {
        Self {
            repository,
            drive_port,
            search_port,
            notification_port,
        }
    }

    pub fn list_node_tree(&self, ctx: &PromptsRequestContext, command: ListNodeTreeCommand) -> Result<NodeTreeResult, PromptsServiceError> {
        self.repository.list_node_tree(ctx, &command)
    }

    pub fn list_nodes(&self, ctx: &PromptsRequestContext, command: ListNodesCommand) -> Result<NodePageResult, PromptsServiceError> {
        let mut cmd = command;
        if cmd.limit == 0 {
            cmd.limit = 20;
        }
        if cmd.limit > 100 {
            cmd.limit = 100;
        }
        self.repository.list_nodes(ctx, &cmd)
    }

    pub fn retrieve_topic_by_slug(
        &self,
        ctx: &PromptsRequestContext,
        command: RetrieveTopicBySlugCommand,
    ) -> Result<PromptsTopic, PromptsServiceError> {
        if command.slug.trim().is_empty() {
            return Err(PromptsServiceError::validation("slug must not be empty"));
        }
        self.repository.retrieve_topic_by_slug(ctx, &command)
    }

    pub fn list_tags(&self, ctx: &PromptsRequestContext, command: ListTagsCommand) -> Result<TagPageResult, PromptsServiceError> {
        let mut cmd = command;
        if cmd.limit == 0 {
            cmd.limit = 20;
        }
        if cmd.limit > 100 {
            cmd.limit = 100;
        }
        self.repository.list_tags(ctx, &cmd)
    }

    pub fn list_topics(&self, ctx: &PromptsRequestContext, command: ListTopicsCommand) -> Result<TopicPageResult, PromptsServiceError> {
        if let Some(ref sort) = command.sort {
            let valid_sorts = ["latest", "top", "created", "most_replies"];
            if !valid_sorts.contains(&sort.as_str()) {
                return Err(PromptsServiceError::validation(format!("invalid sort: {sort}")));
            }
        }
        let mut cmd = command;
        if cmd.limit == 0 {
            cmd.limit = 20;
        }
        if cmd.limit > 100 {
            cmd.limit = 100;
        }
        self.repository.list_topics(ctx, &cmd)
    }

    pub fn create_topic(&self, ctx: &PromptsRequestContext, command: CreateTopicCommand) -> Result<PromptsTopic, PromptsServiceError> {
        if command.title.trim().is_empty() {
            return Err(PromptsServiceError::validation("title must not be empty"));
        }
        if command.title.len() > 240 {
            return Err(PromptsServiceError::validation("title must not exceed 240 characters"));
        }
        if command.body.trim().is_empty() {
            return Err(PromptsServiceError::validation("body must not be empty"));
        }
        if !VALID_BODY_FORMATS.contains(&command.body_format.as_str()) {
            return Err(PromptsServiceError::validation(format!("invalid body_format: {}", command.body_format)));
        }
        if let Some(ref topic_type) = command.topic_type {
            let valid_types = ["discussion", "question", "poll", "announcement", "article"];
            if !valid_types.contains(&topic_type.as_str()) {
                return Err(PromptsServiceError::validation(format!("invalid topic_type: {topic_type}")));
            }
        }
        if let Some(ref visibility) = command.visibility {
            let valid_vis = ["public", "members", "private", "unlisted"];
            if !valid_vis.contains(&visibility.as_str()) {
                return Err(PromptsServiceError::validation(format!("invalid visibility: {visibility}")));
            }
        }
        if command.board_id <= 0 {
            return Err(PromptsServiceError::validation("board_id must be positive"));
        }
        if !self.repository.check_board_exists(ctx, command.board_id)? {
            return Err(PromptsServiceError::not_found("board", command.board_id.to_string()));
        }
        let active_sanctions = self.repository.check_active_sanctions(ctx, ctx.user_id_value())?;
        if !active_sanctions.is_empty() {
            let has_restrict = active_sanctions.iter().any(|s| s.sanction_type == "ban" || s.sanction_type == "restrict_posting");
            if has_restrict {
                return Err(PromptsServiceError::sanctioned("user is sanctioned and cannot create topics"));
            }
        }
        self.repository.create_topic(ctx, &command).map(|topic| {
            let _ = self.record_domain_event(
                ctx,
                PromptsDomainEvent::topic_created(topic.id.to_string()),
                json!({
                    "topicId": topic.id,
                    "boardId": topic.board_id,
                    "authorUserId": topic.author_user_id,
                }),
            );
            let _ = self.repository.update_topic_stats(ctx, topic.id);
            let _ = self.repository.update_board_stats(ctx, topic.board_id);
            topic
        })
    }

    pub fn retrieve_topic(&self, ctx: &PromptsRequestContext, topic_id: i64) -> Result<PromptsTopic, PromptsServiceError> {
        if topic_id <= 0 {
            return Err(PromptsServiceError::validation("topic_id must be positive"));
        }
        self.repository.retrieve_topic(ctx, topic_id)
    }

    pub fn update_topic(&self, ctx: &PromptsRequestContext, command: UpdateTopicCommand) -> Result<PromptsTopic, PromptsServiceError> {
        if let Some(ref title) = command.title {
            if title.trim().is_empty() {
                return Err(PromptsServiceError::validation("title must not be empty"));
            }
            if title.len() > 240 {
                return Err(PromptsServiceError::validation("title must not exceed 240 characters"));
            }
        }
        if let Some(ref body) = command.body {
            if body.trim().is_empty() {
                return Err(PromptsServiceError::validation("body must not be empty"));
            }
        }
        if let Some(ref body_format) = command.body_format {
            if !VALID_BODY_FORMATS.contains(&body_format.as_str()) {
                return Err(PromptsServiceError::validation(format!("invalid body_format: {body_format}")));
            }
        }
        self.repository.update_topic(ctx, &command).map(|topic| {
            let _ = self.record_domain_event(
                ctx,
                PromptsDomainEvent::topic_updated(topic.id.to_string()),
                json!({ "topicId": topic.id, "boardId": topic.board_id }),
            );
            topic
        })
    }

    pub fn delete_topic(&self, ctx: &PromptsRequestContext, command: DeleteTopicCommand) -> Result<CommandResult, PromptsServiceError> {
        if command.topic_id <= 0 {
            return Err(PromptsServiceError::validation("topic_id must be positive"));
        }
        let topic_id = command.topic_id;
        self.repository.delete_topic(ctx, &command).map(|result| {
            self.remove_search_document_best_effort("topic", &topic_id.to_string());
            let _ = self.record_domain_event(
                ctx,
                PromptsDomainEvent::topic_deleted(topic_id.to_string()),
                json!({ "topicId": topic_id }),
            );
            result
        })
    }

    pub fn list_replies(&self, ctx: &PromptsRequestContext, command: ListRepliesCommand) -> Result<ReplyPageResult, PromptsServiceError> {
        if command.topic_id <= 0 {
            return Err(PromptsServiceError::validation("topic_id must be positive"));
        }
        let mut cmd = command;
        if cmd.limit == 0 {
            cmd.limit = 20;
        }
        if cmd.limit > 100 {
            cmd.limit = 100;
        }
        self.repository.list_replies(ctx, &cmd)
    }

    pub fn create_reply(&self, ctx: &PromptsRequestContext, command: CreateReplyCommand) -> Result<PromptsReply, PromptsServiceError> {
        if command.body.trim().is_empty() {
            return Err(PromptsServiceError::validation("body must not be empty"));
        }
        if !VALID_BODY_FORMATS.contains(&command.body_format.as_str()) {
            return Err(PromptsServiceError::validation(format!("invalid body_format: {}", command.body_format)));
        }
        if command.topic_id <= 0 {
            return Err(PromptsServiceError::validation("topic_id must be positive"));
        }
        let topic = self.repository.retrieve_topic(ctx, command.topic_id)?;
        if topic.is_locked() {
            return Err(PromptsServiceError::topic_locked(command.topic_id));
        }
        let active_sanctions = self.repository.check_active_sanctions(ctx, ctx.user_id_value())?;
        if !active_sanctions.is_empty() {
            let has_restrict = active_sanctions.iter().any(|s| s.sanction_type == "ban" || s.sanction_type == "restrict_posting");
            if has_restrict {
                return Err(PromptsServiceError::sanctioned("user is sanctioned and cannot create replies"));
            }
        }
        self.repository.create_reply(ctx, &command).map(|reply| {
            let _ = self.record_domain_event(
                ctx,
                PromptsDomainEvent::reply_created(reply.id.to_string()),
                json!({
                    "replyId": reply.id,
                    "topicId": reply.topic_id,
                    "boardId": reply.board_id,
                    "authorUserId": reply.author_user_id,
                }),
            );
            let _ = self.repository.update_topic_stats(ctx, reply.topic_id);
            let _ = self.repository.update_board_stats(ctx, reply.board_id);
            reply
        })
    }

    pub fn update_reply(&self, ctx: &PromptsRequestContext, command: UpdateReplyCommand) -> Result<PromptsReply, PromptsServiceError> {
        if let Some(ref body) = command.body {
            if body.trim().is_empty() {
                return Err(PromptsServiceError::validation("body must not be empty"));
            }
        }
        if let Some(ref body_format) = command.body_format {
            if !VALID_BODY_FORMATS.contains(&body_format.as_str()) {
                return Err(PromptsServiceError::validation(format!("invalid body_format: {body_format}")));
            }
        }
        self.repository.update_reply(ctx, &command)
    }

    pub fn delete_reply(&self, ctx: &PromptsRequestContext, command: DeleteReplyCommand) -> Result<CommandResult, PromptsServiceError> {
        if command.reply_id <= 0 {
            return Err(PromptsServiceError::validation("reply_id must be positive"));
        }
        let reply_id = command.reply_id;
        self.repository.delete_reply(ctx, &command).map(|result| {
            self.remove_search_document_best_effort("reply", &reply_id.to_string());
            let _ = self.record_domain_event(
                ctx,
                PromptsDomainEvent::reply_deleted(reply_id.to_string()),
                json!({ "replyId": reply_id }),
            );
            result
        })
    }

    pub fn accept_reply(&self, ctx: &PromptsRequestContext, command: AcceptReplyCommand) -> Result<PromptsTopic, PromptsServiceError> {
        if command.topic_id <= 0 {
            return Err(PromptsServiceError::validation("topic_id must be positive"));
        }
        if command.reply_id <= 0 {
            return Err(PromptsServiceError::validation("reply_id must be positive"));
        }
        let topic = self.repository.retrieve_topic(ctx, command.topic_id)?;
        if !topic.is_question() {
            return Err(PromptsServiceError::validation("only question topics can have accepted replies"));
        }
        self.repository.accept_reply(ctx, &command)
    }

    pub fn clear_accepted_reply(&self, ctx: &PromptsRequestContext, command: ClearAcceptedReplyCommand) -> Result<CommandResult, PromptsServiceError> {
        if command.topic_id <= 0 {
            return Err(PromptsServiceError::validation("topic_id must be positive"));
        }
        self.repository.clear_accepted_reply(ctx, &command)
    }

    pub fn create_report(&self, ctx: &PromptsRequestContext, command: CreateReportCommand) -> Result<CommandResult, PromptsServiceError> {
        if command.reason_code.trim().is_empty() {
            return Err(PromptsServiceError::validation("reason_code must not be empty"));
        }
        if command.reason_code.len() > 80 {
            return Err(PromptsServiceError::validation("reason_code must not exceed 80 characters"));
        }
        let valid_target_types = ["topic", "reply", "member", "board"];
        if !valid_target_types.contains(&command.target_type.as_str()) {
            return Err(PromptsServiceError::validation(format!("invalid target_type: {}", command.target_type)));
        }
        if command.target_id <= 0 {
            return Err(PromptsServiceError::validation("target_id must be positive"));
        }
        if let Some(ref description) = command.description {
            if description.len() > 2000 {
                return Err(PromptsServiceError::validation("description must not exceed 2000 characters"));
            }
        }
        self.repository.create_report(ctx, &command)
    }

    pub fn list_feed(&self, ctx: &PromptsRequestContext, command: ListFeedCommand) -> Result<FeedPageResult, PromptsServiceError> {
        if let Some(ref feed_type) = command.feed_type {
            let valid_types = ["home", "board", "tag", "member"];
            if !valid_types.contains(&feed_type.as_str()) {
                return Err(PromptsServiceError::validation(format!("invalid feed_type: {feed_type}")));
            }
        }
        let mut cmd = command;
        if cmd.limit == 0 {
            cmd.limit = 20;
        }
        if cmd.limit > 100 {
            cmd.limit = 100;
        }
        self.repository.list_feed(ctx, &cmd)
    }

    pub fn query_search(&self, ctx: &PromptsRequestContext, command: QuerySearchCommand) -> Result<SearchResult, PromptsServiceError> {
        if command.query.trim().is_empty() {
            return Err(PromptsServiceError::validation("query must not be empty"));
        }
        let mut cmd = command;
        if cmd.limit == 0 {
            cmd.limit = 20;
        }
        if cmd.limit > 100 {
            cmd.limit = 100;
        }
        self.repository.query_search(ctx, &cmd)
    }

    pub fn list_moderation_queue(&self, ctx: &PromptsRequestContext, command: ListModerationQueueCommand) -> Result<ModerationQueueResult, PromptsServiceError> {
        let mut cmd = command;
        if cmd.limit == 0 {
            cmd.limit = 20;
        }
        if cmd.limit > 100 {
            cmd.limit = 100;
        }
        self.repository.list_moderation_queue(ctx, &cmd)
    }

    pub fn create_moderation_decision(&self, ctx: &PromptsRequestContext, command: CreateModerationDecisionCommand) -> Result<ModerationDecisionResult, PromptsServiceError> {
        if command.case_id <= 0 {
            return Err(PromptsServiceError::validation("case_id must be positive"));
        }
        if command.reason_code.trim().is_empty() {
            return Err(PromptsServiceError::validation("reason_code must not be empty"));
        }
        if command.reason_code.len() > 80 {
            return Err(PromptsServiceError::validation("reason_code must not exceed 80 characters"));
        }
        if let Some(ref note) = command.note {
            if note.len() > 2000 {
                return Err(PromptsServiceError::validation("note must not exceed 2000 characters"));
            }
        }
        let valid_actions = ["dismiss", "hide", "restore", "lock", "move", "sanction", "escalate"];
        if !valid_actions.contains(&command.decision_action.as_str()) {
            return Err(PromptsServiceError::validation(format!("invalid decision_action: {}", command.decision_action)));
        }
        self.repository.create_moderation_decision(ctx, &command).map(|decision| {
            let _ = self.record_domain_event(
                ctx,
                PromptsDomainEvent::moderation_decision_created(command.case_id.to_string()),
                json!({
                    "caseId": command.case_id,
                    "decisionAction": decision.decision_action,
                }),
            );
            decision
        })
    }

    pub fn rebuild_search_projection(&self, ctx: &PromptsRequestContext, command: RebuildSearchProjectionCommand) -> Result<CommandResult, PromptsServiceError> {
        let result = self.repository.rebuild_search_projection(ctx, &command)?;
        if let Err(error) = self.search_port.rebuild_index(command.board_id) {
            tracing::warn!(board_id = ?command.board_id, error, "forum search rebuild failed");
        }
        Ok(result)
    }

    pub fn rebuild_stats(&self, ctx: &PromptsRequestContext, command: RebuildStatsCommand) -> Result<CommandResult, PromptsServiceError> {
        self.repository.rebuild_stats(ctx, &command)
    }

    pub fn publish_pending_outbox(&self, ctx: &PromptsRequestContext, command: PublishOutboxCommand) -> Result<CommandResult, PromptsServiceError> {
        let mut cmd = command;
        if cmd.limit == 0 {
            cmd.limit = 50;
        }
        if cmd.limit > 500 {
            cmd.limit = 500;
        }

        let events = self.repository.list_pending_outbox_events(ctx, &cmd)?;
        let mut published = 0_i64;
        for event in events {
            self.notify_prm_event_best_effort(&event.event_type, &event.aggregate_id);
            self.repository.mark_outbox_published(ctx, event.id)?;
            published += 1;
        }

        Ok(CommandResult {
            success: true,
            id: Some(published),
            uuid: None,
            status: Some("published".to_string()),
        })
    }

    pub fn fanout_notifications(
        &self,
        ctx: &PromptsRequestContext,
        command: FanoutNotificationsCommand,
    ) -> Result<CommandResult, PromptsServiceError> {
        let mut cmd = PublishOutboxCommand {
            limit: command.limit,
        };
        if cmd.limit == 0 {
            cmd.limit = 50;
        }
        if cmd.limit > 500 {
            cmd.limit = 500;
        }

        let events = self.repository.list_pending_outbox_events(ctx, &cmd)?;
        let mut delivered = 0_i64;
        for event in events {
            let payload: Value = serde_json::from_str(&event.payload_json).unwrap_or(json!({}));
            let topic_id = payload
                .get("topicId")
                .or_else(|| payload.get("topic_id"))
                .and_then(Value::as_i64);
            if topic_id.is_none() {
                continue;
            }
            let topic_id = topic_id.unwrap_or_default();
            let subscriptions = self.repository.list_subscriptions(
                ctx,
                &ListSubscriptionsCommand {
                    target_type: Some("topic".to_string()),
                    target_id: Some(topic_id),
                    cursor: None,
                    limit: 100,
                },
            )?;
            for subscription in subscriptions.items {
                if subscription.target_id != topic_id {
                    continue;
                }
                if let Err(error) = self.notification_port.publish_subscription_notification(
                    subscription.user_id,
                    &event.event_type,
                    topic_id,
                ) {
                    tracing::warn!(
                        user_id = subscription.user_id,
                        topic_id,
                        error,
                        "forum subscription notification failed"
                    );
                } else {
                    delivered += 1;
                }
            }
        }

        Ok(CommandResult {
            success: true,
            id: Some(delivered),
            uuid: None,
            status: Some("fanout".to_string()),
        })
    }

    pub fn list_topic_revisions(&self, ctx: &PromptsRequestContext, command: ListTopicRevisionsCommand) -> Result<TopicRevisionPageResult, PromptsServiceError> {
        if command.topic_id <= 0 {
            return Err(PromptsServiceError::validation("topic_id must be positive"));
        }
        let mut cmd = command;
        if cmd.limit == 0 {
            cmd.limit = 20;
        }
        if cmd.limit > 100 {
            cmd.limit = 100;
        }
        self.repository.list_topic_revisions(ctx, &cmd)
    }

    pub fn list_reply_revisions(&self, ctx: &PromptsRequestContext, command: ListReplyRevisionsCommand) -> Result<ReplyRevisionPageResult, PromptsServiceError> {
        if command.reply_id <= 0 {
            return Err(PromptsServiceError::validation("reply_id must be positive"));
        }
        let mut cmd = command;
        if cmd.limit == 0 {
            cmd.limit = 20;
        }
        if cmd.limit > 100 {
            cmd.limit = 100;
        }
        self.repository.list_reply_revisions(ctx, &cmd)
    }

    pub fn create_poll_vote(&self, ctx: &PromptsRequestContext, command: CreatePollVoteCommand) -> Result<CommandResult, PromptsServiceError> {
        if command.option_ids.is_empty() {
            return Err(PromptsServiceError::validation("at least one option must be selected"));
        }
        if command.option_ids.len() > 50 {
            return Err(PromptsServiceError::validation("too many poll options selected"));
        }
        let mut sorted = command.option_ids.clone();
        sorted.sort();
        sorted.dedup();
        if sorted.len() != command.option_ids.len() {
            return Err(PromptsServiceError::validation("duplicate poll option ids"));
        }
        if command.poll_id <= 0 {
            return Err(PromptsServiceError::validation("poll_id must be positive"));
        }
        if !self.repository.check_poll_exists(ctx, command.poll_id)? {
            return Err(PromptsServiceError::not_found("poll", command.poll_id.to_string()));
        }
        let selection_mode = self.repository.check_poll_selection_mode(ctx, command.poll_id)?;
        if selection_mode == "single" && command.option_ids.len() > 1 {
            return Err(PromptsServiceError::validation("single-choice poll allows only one option"));
        }
        self.repository.create_poll_vote(ctx, &command)
    }

    pub fn create_reaction(&self, ctx: &PromptsRequestContext, command: CreateReactionCommand) -> Result<CommandResult, PromptsServiceError> {
        if command.target_id <= 0 {
            return Err(PromptsServiceError::validation("target_id must be positive"));
        }
        let valid_target_types = ["topic", "reply"];
        if !valid_target_types.contains(&command.target_type.as_str()) {
            return Err(PromptsServiceError::validation(format!("invalid target_type: {}", command.target_type)));
        }
        if command.reaction_type.trim().is_empty() {
            return Err(PromptsServiceError::validation("reaction_type must not be empty"));
        }
        self.repository.create_reaction(ctx, &command)
    }

    pub fn create_vote(&self, ctx: &PromptsRequestContext, command: CreateVoteCommand) -> Result<CommandResult, PromptsServiceError> {
        if command.target_id <= 0 {
            return Err(PromptsServiceError::validation("target_id must be positive"));
        }
        let valid_target_types = ["topic", "reply"];
        if !valid_target_types.contains(&command.target_type.as_str()) {
            return Err(PromptsServiceError::validation(format!("invalid target_type: {}", command.target_type)));
        }
        if command.vote_value != 1 && command.vote_value != -1 {
            return Err(PromptsServiceError::validation("vote_value must be 1 or -1"));
        }
        if let Some(ref reason_code) = command.reason_code {
            if reason_code.len() > 64 {
                return Err(PromptsServiceError::validation("reason_code must not exceed 64 characters"));
            }
        }
        if self.repository.check_active_vote(ctx, &command.target_type, command.target_id, ctx.user_id_value())? {
            return Err(PromptsServiceError::conflict("user has already voted on this target"));
        }
        self.repository.create_vote(ctx, &command)
    }

    pub fn update_bookmark(&self, ctx: &PromptsRequestContext, command: UpdateBookmarkCommand) -> Result<CommandResult, PromptsServiceError> {
        if command.target_id <= 0 {
            return Err(PromptsServiceError::validation("target_id must be positive"));
        }
        let valid_target_types = ["topic", "reply"];
        if !valid_target_types.contains(&command.target_type.as_str()) {
            return Err(PromptsServiceError::validation(format!("invalid target_type: {}", command.target_type)));
        }
        if let Some(ref note) = command.note {
            if note.len() > 500 {
                return Err(PromptsServiceError::validation("note must not exceed 500 characters"));
            }
        }
        self.repository.update_bookmark(ctx, &command)
    }

    pub fn update_read_state(&self, ctx: &PromptsRequestContext, command: UpdateReadStateCommand) -> Result<CommandResult, PromptsServiceError> {
        if command.topic_id <= 0 {
            return Err(PromptsServiceError::validation("topic_id must be positive"));
        }
        self.repository.update_read_state(ctx, &command)
    }

    pub fn pin_topic(&self, ctx: &PromptsRequestContext, command: PinTopicCommand) -> Result<CommandResult, PromptsServiceError> {
        if command.topic_id <= 0 {
            return Err(PromptsServiceError::validation("topic_id must be positive"));
        }
        self.repository.pin_topic(ctx, &command)
    }

    pub fn unpin_topic(&self, ctx: &PromptsRequestContext, command: PinTopicCommand) -> Result<CommandResult, PromptsServiceError> {
        if command.topic_id <= 0 {
            return Err(PromptsServiceError::validation("topic_id must be positive"));
        }
        self.repository.unpin_topic(ctx, &command)
    }

    pub fn feature_topic(&self, ctx: &PromptsRequestContext, command: FeatureTopicCommand) -> Result<CommandResult, PromptsServiceError> {
        if command.topic_id <= 0 {
            return Err(PromptsServiceError::validation("topic_id must be positive"));
        }
        self.repository.feature_topic(ctx, &command)
    }

    pub fn unfeature_topic(&self, ctx: &PromptsRequestContext, command: FeatureTopicCommand) -> Result<CommandResult, PromptsServiceError> {
        if command.topic_id <= 0 {
            return Err(PromptsServiceError::validation("topic_id must be positive"));
        }
        self.repository.unfeature_topic(ctx, &command)
    }

    pub fn lock_topic(&self, ctx: &PromptsRequestContext, command: LockTopicCommand) -> Result<CommandResult, PromptsServiceError> {
        if command.topic_id <= 0 {
            return Err(PromptsServiceError::validation("topic_id must be positive"));
        }
        self.repository.lock_topic(ctx, &command)
    }

    pub fn unlock_topic(&self, ctx: &PromptsRequestContext, command: LockTopicCommand) -> Result<CommandResult, PromptsServiceError> {
        if command.topic_id <= 0 {
            return Err(PromptsServiceError::validation("topic_id must be positive"));
        }
        self.repository.unlock_topic(ctx, &command)
    }

    pub fn move_topic(&self, ctx: &PromptsRequestContext, command: MoveTopicCommand) -> Result<CommandResult, PromptsServiceError> {
        if command.topic_id <= 0 {
            return Err(PromptsServiceError::validation("topic_id must be positive"));
        }
        if command.target_board_id <= 0 {
            return Err(PromptsServiceError::validation("target_board_id must be positive"));
        }
        self.repository.move_topic(ctx, &command)
    }

    pub fn create_node(&self, ctx: &PromptsRequestContext, command: CreateNodeCommand) -> Result<PromptsNode, PromptsServiceError> {
        if command.slug.trim().is_empty() {
            return Err(PromptsServiceError::validation("slug must not be empty"));
        }
        if command.slug.len() > 120 {
            return Err(PromptsServiceError::validation("slug must not exceed 120 characters"));
        }
        let normalized_slug = command.slug.to_lowercase().replace(' ', "-");
        if normalized_slug != command.slug {
            return Err(PromptsServiceError::validation("slug must be lowercase with hyphens, no spaces"));
        }
        if command.name.trim().is_empty() {
            return Err(PromptsServiceError::validation("name must not be empty"));
        }
        if command.name.len() > 160 {
            return Err(PromptsServiceError::validation("name must not exceed 160 characters"));
        }
        if let Some(ref description) = command.description {
            if description.len() > 1000 {
                return Err(PromptsServiceError::validation("description must not exceed 1000 characters"));
            }
        }
        let valid_node_types = ["category", "board"];
        if !valid_node_types.contains(&command.node_type.as_str()) {
            return Err(PromptsServiceError::validation(format!("invalid node_type: {}", command.node_type)));
        }
        if command.space_id <= 0 {
            return Err(PromptsServiceError::validation("space_id must be positive"));
        }
        self.repository.create_node(ctx, &command)
    }

    pub fn update_node(&self, ctx: &PromptsRequestContext, command: UpdateNodeCommand) -> Result<PromptsNode, PromptsServiceError> {
        if let Some(ref name) = command.name {
            if name.trim().is_empty() {
                return Err(PromptsServiceError::validation("name must not be empty"));
            }
            if name.len() > 160 {
                return Err(PromptsServiceError::validation("name must not exceed 160 characters"));
            }
        }
        if let Some(ref description) = command.description {
            if description.len() > 1000 {
                return Err(PromptsServiceError::validation("description must not exceed 1000 characters"));
            }
        }
        if command.node_id <= 0 {
            return Err(PromptsServiceError::validation("node_id must be positive"));
        }
        if let Some(parent_id) = command.parent_id {
            if parent_id == command.node_id {
                return Err(PromptsServiceError::validation("node cannot be its own parent"));
            }
            if parent_id < 0 {
                return Err(PromptsServiceError::validation("parent_id must be non-negative"));
            }
            // [REPO] Deep cycle detection requires repository tree query
            if self.repository.check_node_cycle(ctx, command.node_id, parent_id)? {
                return Err(PromptsServiceError::validation("moving node would create a cycle"));
            }
        }
        self.repository.update_node(ctx, &command)
    }

    pub fn delete_node(&self, ctx: &PromptsRequestContext, command: DeleteNodeCommand) -> Result<CommandResult, PromptsServiceError> {
        if command.node_id <= 0 {
            return Err(PromptsServiceError::validation("node_id must be positive"));
        }
        self.repository.delete_node(ctx, &command)
    }

    pub fn list_moderation_cases(&self, ctx: &PromptsRequestContext, command: ListModerationCasesCommand) -> Result<ModerationCasePageResult, PromptsServiceError> {
        let mut cmd = command;
        if cmd.limit == 0 {
            cmd.limit = 20;
        }
        if cmd.limit > 100 {
            cmd.limit = 100;
        }
        self.repository.list_moderation_cases(ctx, &cmd)
    }

    pub fn create_moderation_case(&self, ctx: &PromptsRequestContext, command: CreateModerationCaseCommand) -> Result<PromptsModerationCase, PromptsServiceError> {
        let valid_target_types = ["topic", "reply", "member", "board"];
        if !valid_target_types.contains(&command.target_type.as_str()) {
            return Err(PromptsServiceError::validation(format!("invalid target_type: {}", command.target_type)));
        }
        let valid_severities = ["low", "medium", "high", "critical"];
        if !valid_severities.contains(&command.severity.as_str()) {
            return Err(PromptsServiceError::validation(format!("invalid severity: {}", command.severity)));
        }
        if command.target_id <= 0 {
            return Err(PromptsServiceError::validation("target_id must be positive"));
        }
        if let Some(ref summary) = command.summary {
            if summary.len() > 1000 {
                return Err(PromptsServiceError::validation("summary must not exceed 1000 characters"));
            }
        }
        self.repository.create_moderation_case(ctx, &command).map(|case| {
            self.notify_moderation_alert_best_effort(case.id, &case.severity);
            let _ = self.record_domain_event(
                ctx,
                PromptsDomainEvent::moderation_case_created(case.id.to_string()),
                json!({
                    "caseId": case.id,
                    "targetType": case.target_type,
                    "targetId": case.target_id,
                    "severity": case.severity,
                }),
            );
            case
        })
    }

    pub fn retrieve_moderation_case(&self, ctx: &PromptsRequestContext, command: RetrieveModerationCaseCommand) -> Result<PromptsModerationCase, PromptsServiceError> {
        if command.case_id <= 0 {
            return Err(PromptsServiceError::validation("case_id must be positive"));
        }
        self.repository.retrieve_moderation_case(ctx, &command)
    }

    pub fn list_sanctions(&self, ctx: &PromptsRequestContext, command: ListSanctionsCommand) -> Result<SanctionPageResult, PromptsServiceError> {
        let mut cmd = command;
        if cmd.limit == 0 {
            cmd.limit = 20;
        }
        if cmd.limit > 100 {
            cmd.limit = 100;
        }
        self.repository.list_sanctions(ctx, &cmd)
    }

    pub fn create_sanction(&self, ctx: &PromptsRequestContext, command: CreateSanctionCommand) -> Result<PromptsSanction, PromptsServiceError> {
        let valid_types = ["mute", "suspend", "restrict_posting", "ban"];
        if !valid_types.contains(&command.sanction_type.as_str()) {
            return Err(PromptsServiceError::validation(format!("invalid sanction_type: {}", command.sanction_type)));
        }
        if command.reason_code.trim().is_empty() {
            return Err(PromptsServiceError::validation("reason_code must not be empty"));
        }
        if command.reason_code.len() > 80 {
            return Err(PromptsServiceError::validation("reason_code must not exceed 80 characters"));
        }
        if command.user_id <= 0 {
            return Err(PromptsServiceError::validation("user_id must be positive"));
        }
        if command.starts_at.trim().is_empty() {
            return Err(PromptsServiceError::validation("starts_at must not be empty"));
        }
        if let Some(case_id) = command.case_id {
            if case_id <= 0 {
                return Err(PromptsServiceError::validation("case_id must be positive"));
            }
        }
        if let Some(decision_id) = command.decision_id {
            if decision_id <= 0 {
                return Err(PromptsServiceError::validation("decision_id must be positive"));
            }
        }
        if let Some(ref expires_at) = command.expires_at {
            if expires_at.trim().is_empty() {
                return Err(PromptsServiceError::validation("expires_at must not be empty string"));
            }
        }
        self.repository.create_sanction(ctx, &command)
    }

    pub fn update_sanction(&self, ctx: &PromptsRequestContext, command: UpdateSanctionCommand) -> Result<PromptsSanction, PromptsServiceError> {
        if command.sanction_id <= 0 {
            return Err(PromptsServiceError::validation("sanction_id must be positive"));
        }
        self.repository.update_sanction(ctx, &command)
    }

    pub fn list_reputation_rules(&self, ctx: &PromptsRequestContext, command: ListReputationRulesCommand) -> Result<ReputationRulePageResult, PromptsServiceError> {
        let mut cmd = command;
        if cmd.limit == 0 {
            cmd.limit = 20;
        }
        if cmd.limit > 100 {
            cmd.limit = 100;
        }
        self.repository.list_reputation_rules(ctx, &cmd)
    }

    pub fn create_reputation_rule(&self, ctx: &PromptsRequestContext, command: CreateReputationRuleCommand) -> Result<PromptsReputationRule, PromptsServiceError> {
        if command.code.trim().is_empty() {
            return Err(PromptsServiceError::validation("code must not be empty"));
        }
        if command.code.len() > 80 {
            return Err(PromptsServiceError::validation("code must not exceed 80 characters"));
        }
        if command.event_type.trim().is_empty() {
            return Err(PromptsServiceError::validation("event_type must not be empty"));
        }
        if command.event_type.len() > 120 {
            return Err(PromptsServiceError::validation("event_type must not exceed 120 characters"));
        }
        if let Some(daily_limit) = command.daily_limit {
            if daily_limit < 0 {
                return Err(PromptsServiceError::validation("daily_limit must be non-negative"));
            }
        }
        self.repository.create_reputation_rule(ctx, &command)
    }

    pub fn list_reputation_ledger(&self, ctx: &PromptsRequestContext, command: ListReputationLedgerCommand) -> Result<ReputationLedgerPageResult, PromptsServiceError> {
        let mut cmd = command;
        if cmd.limit == 0 {
            cmd.limit = 20;
        }
        if cmd.limit > 100 {
            cmd.limit = 100;
        }
        self.repository.list_reputation_ledger(ctx, &cmd)
    }

    pub fn list_trust_levels(&self, ctx: &PromptsRequestContext, command: ListTrustLevelsCommand) -> Result<TrustLevelPageResult, PromptsServiceError> {
        let mut cmd = command;
        if cmd.limit == 0 {
            cmd.limit = 20;
        }
        if cmd.limit > 100 {
            cmd.limit = 100;
        }
        self.repository.list_trust_levels(ctx, &cmd)
    }

    pub fn create_trust_level(&self, ctx: &PromptsRequestContext, command: CreateTrustLevelCommand) -> Result<PromptsTrustLevel, PromptsServiceError> {
        if command.code.trim().is_empty() {
            return Err(PromptsServiceError::validation("code must not be empty"));
        }
        if command.code.len() > 64 {
            return Err(PromptsServiceError::validation("code must not exceed 64 characters"));
        }
        if command.name.trim().is_empty() {
            return Err(PromptsServiceError::validation("name must not be empty"));
        }
        if command.name.len() > 120 {
            return Err(PromptsServiceError::validation("name must not exceed 120 characters"));
        }
        if command.level_no < 0 {
            return Err(PromptsServiceError::validation("level_no must be non-negative"));
        }
        if command.level_no > 10 {
            return Err(PromptsServiceError::validation("level_no must not exceed 10"));
        }
        self.repository.create_trust_level(ctx, &command)
    }

    pub fn list_badges(&self, ctx: &PromptsRequestContext, command: ListBadgesCommand) -> Result<BadgePageResult, PromptsServiceError> {
        let mut cmd = command;
        if cmd.limit == 0 {
            cmd.limit = 20;
        }
        if cmd.limit > 100 {
            cmd.limit = 100;
        }
        self.repository.list_badges(ctx, &cmd)
    }

    pub fn create_badge(&self, ctx: &PromptsRequestContext, command: CreateBadgeCommand) -> Result<PromptsBadge, PromptsServiceError> {
        if command.code.trim().is_empty() {
            return Err(PromptsServiceError::validation("code must not be empty"));
        }
        if command.code.len() > 64 {
            return Err(PromptsServiceError::validation("code must not exceed 64 characters"));
        }
        if command.name.trim().is_empty() {
            return Err(PromptsServiceError::validation("name must not be empty"));
        }
        if command.name.len() > 120 {
            return Err(PromptsServiceError::validation("name must not exceed 120 characters"));
        }
        let valid_grant_modes = ["manual", "rule", "external"];
        if !valid_grant_modes.contains(&command.grant_mode.as_str()) {
            return Err(PromptsServiceError::validation(format!("invalid grant_mode: {}", command.grant_mode)));
        }
        self.repository.create_badge(ctx, &command)
    }

    pub fn list_board_stats(&self, ctx: &PromptsRequestContext, command: ListBoardStatsCommand) -> Result<BoardStatsPageResult, PromptsServiceError> {
        let mut cmd = command;
        if cmd.limit == 0 {
            cmd.limit = 20;
        }
        if cmd.limit > 100 {
            cmd.limit = 100;
        }
        self.repository.list_board_stats(ctx, &cmd)
    }

    pub fn list_topic_stats(&self, ctx: &PromptsRequestContext, command: ListTopicStatsCommand) -> Result<TopicStatsPageResult, PromptsServiceError> {
        let mut cmd = command;
        if cmd.limit == 0 {
            cmd.limit = 20;
        }
        if cmd.limit > 100 {
            cmd.limit = 100;
        }
        self.repository.list_topic_stats(ctx, &cmd)
    }

    pub fn create_audit_action(&self, ctx: &PromptsRequestContext, command: CreateAuditActionCommand) -> Result<PromptsAuditAction, PromptsServiceError> {
        if command.action.trim().is_empty() {
            return Err(PromptsServiceError::validation("action must not be empty"));
        }
        if command.action.len() > 160 {
            return Err(PromptsServiceError::validation("action must not exceed 160 characters"));
        }
        if command.target_type.trim().is_empty() {
            return Err(PromptsServiceError::validation("target_type must not be empty"));
        }
        let valid_target_types = ["topic", "reply", "member", "board", "node", "sanction", "case"];
        if !valid_target_types.contains(&command.target_type.as_str()) {
            return Err(PromptsServiceError::validation(format!("invalid target_type: {}", command.target_type)));
        }
        if command.target_id <= 0 {
            return Err(PromptsServiceError::validation("target_id must be positive"));
        }
        self.repository.create_audit_action(ctx, &command)
    }

    pub fn list_audit_actions(
        &self,
        ctx: &PromptsRequestContext,
        command: ListAuditActionsCommand,
    ) -> Result<AuditActionPageResult, PromptsServiceError> {
        let mut cmd = command;
        if cmd.limit == 0 {
            cmd.limit = 20;
        }
        if cmd.limit > 100 {
            cmd.limit = 100;
        }
        self.repository.list_audit_actions(ctx, &cmd)
    }

    pub fn list_topic_prefixes(&self, ctx: &PromptsRequestContext, command: ListTopicPrefixesCommand) -> Result<TopicPrefixPageResult, PromptsServiceError> {
        let mut cmd = command;
        if cmd.limit == 0 {
            cmd.limit = 20;
        }
        if cmd.limit > 100 {
            cmd.limit = 100;
        }
        self.repository.list_topic_prefixes(ctx, &cmd)
    }

    pub fn create_topic_prefix(&self, ctx: &PromptsRequestContext, command: CreateTopicPrefixCommand) -> Result<PromptsTopicPrefix, PromptsServiceError> {
        if command.code.trim().is_empty() {
            return Err(PromptsServiceError::validation("code must not be empty"));
        }
        if command.code.len() > 64 {
            return Err(PromptsServiceError::validation("code must not exceed 64 characters"));
        }
        if command.label.trim().is_empty() {
            return Err(PromptsServiceError::validation("label must not be empty"));
        }
        if command.label.len() > 80 {
            return Err(PromptsServiceError::validation("label must not exceed 80 characters"));
        }
        if command.board_id <= 0 {
            return Err(PromptsServiceError::validation("board_id must be positive"));
        }
        // [REPO] Board existence check requires repository query
        if !self.repository.check_board_exists(ctx, command.board_id)? {
            return Err(PromptsServiceError::validation("board does not exist"));
        }
        self.repository.create_topic_prefix(ctx, &command)
    }

    pub fn create_space(&self, ctx: &PromptsRequestContext, command: CreateSpaceCommand) -> Result<PromptsSpace, PromptsServiceError> {
        if command.code.trim().is_empty() {
            return Err(PromptsServiceError::validation("code must not be empty"));
        }
        if command.code.len() > 64 {
            return Err(PromptsServiceError::validation("code must not exceed 64 characters"));
        }
        if command.slug.trim().is_empty() {
            return Err(PromptsServiceError::validation("slug must not be empty"));
        }
        if command.slug.len() > 120 {
            return Err(PromptsServiceError::validation("slug must not exceed 120 characters"));
        }
        let normalized_slug = command.slug.to_lowercase().replace(' ', "-");
        if normalized_slug != command.slug {
            return Err(PromptsServiceError::validation("slug must be lowercase with hyphens, no spaces"));
        }
        if command.name.trim().is_empty() {
            return Err(PromptsServiceError::validation("name must not be empty"));
        }
        if command.name.len() > 160 {
            return Err(PromptsServiceError::validation("name must not exceed 160 characters"));
        }
        if let Some(ref description) = command.description {
            if description.len() > 1000 {
                return Err(PromptsServiceError::validation("description must not exceed 1000 characters"));
            }
        }
        let valid_visibilities = ["public", "private", "unlisted", "archived"];
        if !valid_visibilities.contains(&command.visibility.as_str()) {
            return Err(PromptsServiceError::validation(format!("invalid visibility: {}", command.visibility)));
        }
        self.repository.create_space(ctx, &command)
    }

    pub fn update_space(&self, ctx: &PromptsRequestContext, command: UpdateSpaceCommand) -> Result<PromptsSpace, PromptsServiceError> {
        if command.space_id <= 0 {
            return Err(PromptsServiceError::validation("space_id must be positive"));
        }
        if let Some(ref name) = command.name {
            if name.trim().is_empty() {
                return Err(PromptsServiceError::validation("name must not be empty"));
            }
            if name.len() > 160 {
                return Err(PromptsServiceError::validation("name must not exceed 160 characters"));
            }
        }
        if let Some(ref description) = command.description {
            if description.len() > 1000 {
                return Err(PromptsServiceError::validation("description must not exceed 1000 characters"));
            }
        }
        if let Some(ref visibility) = command.visibility {
            let valid_visibilities = ["public", "private", "unlisted", "archived"];
            if !valid_visibilities.contains(&visibility.as_str()) {
                return Err(PromptsServiceError::validation(format!("invalid visibility: {visibility}")));
            }
        }
        self.repository.update_space(ctx, &command)
    }

    pub fn create_attachment(&self, ctx: &PromptsRequestContext, command: CreateAttachmentCommand) -> Result<PromptsAttachment, PromptsServiceError> {
        let valid_owner_types = ["topic", "reply"];
        if !valid_owner_types.contains(&command.owner_type.as_str()) {
            return Err(PromptsServiceError::validation(format!("invalid owner_type: {}", command.owner_type)));
        }
        if command.owner_id <= 0 {
            return Err(PromptsServiceError::validation("owner_id must be positive"));
        }
        if command.drive_space_id.trim().is_empty() {
            return Err(PromptsServiceError::validation("drive_space_id must not be empty"));
        }
        if command.drive_space_id.len() > 128 {
            return Err(PromptsServiceError::validation("drive_space_id must not exceed 128 characters"));
        }
        if command.drive_node_id.trim().is_empty() {
            return Err(PromptsServiceError::validation("drive_node_id must not be empty"));
        }
        if command.drive_node_id.len() > 128 {
            return Err(PromptsServiceError::validation("drive_node_id must not exceed 128 characters"));
        }
        if command.file_name.trim().is_empty() {
            return Err(PromptsServiceError::validation("file_name must not be empty"));
        }
        if command.file_name.len() > 260 {
            return Err(PromptsServiceError::validation("file_name must not exceed 260 characters"));
        }
        if command.mime_type.trim().is_empty() {
            return Err(PromptsServiceError::validation("mime_type must not be empty"));
        }
        if command.mime_type.len() > 120 {
            return Err(PromptsServiceError::validation("mime_type must not exceed 120 characters"));
        }
        if command.byte_size < 0 {
            return Err(PromptsServiceError::validation("byte_size must be non-negative"));
        }
        // [REPO] Owner existence check requires repository query
        if !self.repository.check_owner_exists(ctx, &command.owner_type, command.owner_id)? {
            return Err(PromptsServiceError::validation(format!("{} with id {} does not exist", command.owner_type, command.owner_id)));
        }
        if let Some(ref media_resource_id) = command.media_resource_id {
            self.drive_port
                .validate_media_reference(media_resource_id)
                .map_err(PromptsServiceError::validation)?;
        }
        self.repository.create_attachment(ctx, &command)
    }

    pub fn create_subscription(&self, ctx: &PromptsRequestContext, command: CreateSubscriptionCommand) -> Result<PromptsSubscription, PromptsServiceError> {
        let valid_target_types = ["board", "topic", "tag", "member"];
        if !valid_target_types.contains(&command.target_type.as_str()) {
            return Err(PromptsServiceError::validation(format!("invalid target_type: {}", command.target_type)));
        }
        if command.target_id <= 0 {
            return Err(PromptsServiceError::validation("target_id must be positive"));
        }
        let valid_notify_levels = ["watching", "tracking", "muted"];
        if !valid_notify_levels.contains(&command.notify_level.as_str()) {
            return Err(PromptsServiceError::validation(format!("invalid notify_level: {}", command.notify_level)));
        }
        self.repository.create_subscription(ctx, &command)
    }

    pub fn update_subscription(&self, ctx: &PromptsRequestContext, command: UpdateSubscriptionCommand) -> Result<PromptsSubscription, PromptsServiceError> {
        if command.subscription_id <= 0 {
            return Err(PromptsServiceError::validation("subscription_id must be positive"));
        }
        if let Some(ref notify_level) = command.notify_level {
            let valid_notify_levels = ["watching", "tracking", "muted"];
            if !valid_notify_levels.contains(&notify_level.as_str()) {
                return Err(PromptsServiceError::validation(format!("invalid notify_level: {notify_level}")));
            }
        }
        self.repository.update_subscription(ctx, &command)
    }

    pub fn list_subscriptions(&self, ctx: &PromptsRequestContext, command: ListSubscriptionsCommand) -> Result<SubscriptionPageResult, PromptsServiceError> {
        if let Some(ref target_type) = command.target_type {
            let valid_target_types = ["board", "topic", "tag", "member"];
            if !valid_target_types.contains(&target_type.as_str()) {
                return Err(PromptsServiceError::validation(format!("invalid target_type: {target_type}")));
            }
        }
        let mut cmd = command;
        if cmd.limit == 0 {
            cmd.limit = 20;
        }
        if cmd.limit > 100 {
            cmd.limit = 100;
        }
        self.repository.list_subscriptions(ctx, &cmd)
    }
}
