# Job Runbooks

Operational runbooks for forum background jobs.

## Outbox Replay

**Job:** `PublishOutbox`
**Queue:** `forum-outbox`
**Schedule:** Continuous polling

### Purpose
Publish `prm_outbox_event` rows with retry and dead-letter handling.

### Monitoring
- Watch `prm_outbox_event` for rows with `status = 'pending'` and `next_attempt_at < now()`
- Alert if pending count exceeds 1000 or oldest pending is older than 5 minutes

### Replay Procedure
1. Identify failed events: `SELECT * FROM prm_outbox_event WHERE status = 'failed' AND publish_attempts < 5`
2. Reset for retry: `UPDATE prm_outbox_event SET status = 'pending', next_attempt_at = now() WHERE id = :id`
3. For permanently failed events (attempts >= 5): Move to dead-letter queue

### Dead-Lookup
```sql
SELECT event_type, aggregate_type, aggregate_id, publish_attempts, 
       payload_json, created_at
FROM prm_outbox_event 
WHERE status = 'failed' AND publish_attempts >= 5
ORDER BY created_at DESC;
```

## Projection Rebuild

**Job:** `RebuildSearchProjection`
**Queue:** `forum-search-repair-hourly`
**Schedule:** Hourly at :00

### Purpose
Repair failed or stale `prm_search_document` rows and push to Search SDK.

### Monitoring
- Watch `prm_search_document` for rows with `index_status = 'failed'`
- Alert if failed count exceeds 100

### Manual Rebuild
1. Scoped rebuild: Trigger via `POST /backend/v3/api/prompts/search/reindex` with `boardId`
2. Full rebuild: Trigger without `boardId` parameter
3. Monitor progress: `SELECT index_status, COUNT(*) FROM prm_search_document GROUP BY index_status`

### Stale Detection
```sql
SELECT sd.source_type, sd.source_id, sd.source_version, sd.index_status
FROM prm_search_document sd
WHERE sd.index_status = 'indexed' 
  AND sd.source_version < (
    SELECT COALESCE(t.version, r.version) 
    FROM prm_topic t FULL OUTER JOIN prm_topic_reply r 
    ON (sd.source_type = 'topic' AND t.id = sd.source_id)
    OR (sd.source_type = 'reply' AND r.id = sd.source_id)
  )
LIMIT 100;
```

## Stats Rebuild

**Job:** `RebuildStats`
**Schedule:** Nightly at 03:00 UTC

### Purpose
Recalculate topic, board, member, and tag counters from source tables.

### Monitoring
- Compare `prm_topic_stats.reply_count` with actual `COUNT(*) FROM prm_topic_reply`
- Alert if drift exceeds 5%

### Manual Rebuild
```sql
-- Rebuild topic stats
INSERT INTO prm_topic_stats (topic_id, reply_count, view_count, reaction_count, vote_score, bookmark_count, report_count, last_calculated_at)
SELECT t.id, 
       COUNT(DISTINCT r.id) FILTER (WHERE r.moderation_status = 'visible'),
       0, 0, 0, 0, 0,
       now()
FROM prm_topic t
LEFT JOIN prm_topic_reply r ON r.topic_id = t.id
GROUP BY t.id
ON CONFLICT (topic_id) DO UPDATE SET
  reply_count = EXCLUDED.reply_count,
  last_calculated_at = EXCLUDED.last_calculated_at;

-- Rebuild board stats
INSERT INTO prm_board_stats (board_id, topic_count, reply_count, member_count, last_activity_at, last_calculated_at)
SELECT n.id,
       COUNT(DISTINCT t.id) FILTER (WHERE t.moderation_status = 'visible'),
       COUNT(DISTINCT r.id) FILTER (WHERE r.moderation_status = 'visible'),
       COUNT(DISTINCT t.author_user_id),
       MAX(t.last_activity_at),
       now()
FROM prm_node n
LEFT JOIN prm_topic t ON t.board_id = n.id
LEFT JOIN prm_topic_reply r ON r.topic_id = t.id
WHERE n.node_type = 'board'
GROUP BY n.id
ON CONFLICT (board_id) DO UPDATE SET
  topic_count = EXCLUDED.topic_count,
  reply_count = EXCLUDED.reply_count,
  member_count = EXCLUDED.member_count,
  last_activity_at = EXCLUDED.last_activity_at,
  last_calculated_at = EXCLUDED.last_calculated_at;
```

## Moderation Queue Cleanup

**Job:** `EvaluateModerationPolicy`
**Queue:** `forum-moderation-policy`
**Schedule:** Continuous polling

### Purpose
Evaluate moderation policies and create queue items idempotently.

### Stuck Queue Detection
```sql
SELECT id, target_type, target_id, severity, queue_status, created_at,
       now() - created_at as age
FROM prm_moderation_queue_item
WHERE queue_status IN ('open', 'assigned')
  AND created_at < now() - INTERVAL '24 hours'
ORDER BY severity DESC, created_at ASC;
```

### Cleanup Procedure
1. Identify stuck items (open/assigned > 24h)
2. Reassign to senior moderator pool
3. Escalate severity if SLA breached
4. Log cleanup action in `prm_audit_action`
