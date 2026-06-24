# Prompts Moderation Runbook

## Purpose

This runbook defines the first moderation workflow contract for forum operators.

## Queue Triage

1. List queue items by severity, status, board, and creation time.
2. Retrieve the linked topic, reply, report, member profile, and prior decisions.
3. Create a moderation decision with one action: dismiss, hide, restore, lock, move, sanction, or escalate.
4. Write an outbox event in the same transaction as the decision.

## Required Audit Data

- Operator principal from backend dual-token context.
- Tenant and organization scope.
- Target type and target id.
- Decision action and reason code.
- Before/after moderation status.
- Request id and idempotency key when supplied.

## Escalation SLA

| Severity | Initial Response | Resolution Target |
|----------|-----------------|-------------------|
| critical | 15 minutes | 1 hour |
| high | 1 hour | 4 hours |
| medium | 4 hours | 24 hours |
| low | 24 hours | 72 hours |

Escalation triggers:
- Queue item exceeds SLA without assignment
- Moderator marks decision action as `escalate`
- Automated policy flags content as `critical`

Escalation path:
1. Auto-assign to senior moderator pool
2. Send notification via `PromptsNotificationPort.publish_moderation_alert()`
3. Update queue item `severity` to next level
4. Log escalation event in `prm_audit_action`

## Notification Templates

### Moderation Alert (to operators)

```
Event: prompts.moderation.alert
Subject: Moderation required: {severity} issue on {target_type}
Body:
  Case: {case_no}
  Target: {target_type}/{target_id}
  Severity: {severity}
  Source: {source_type}
  Board: {board_name}
  Action: Review in moderation queue
```

### Sanction Notice (to user)

```
Event: prompts.sanction.applied
Subject: Your forum account has been {sanction_type}
Body:
  Reason: {reason_code}
  Duration: {expires_at ? "until {expires_at}" : "indefinite"}
  Appeal: You may appeal through the forum appeal process
  Case: {case_no}
```

### Appeal Decision (to user)

```
Event: prompts.appeal.decided
Subject: Appeal decision for case {case_no}
Body:
  Decision: {appeal_status}
  Note: {resolution_note}
  Next: {appeal_status == "accepted" ? "Sanction lifted" : "Sanction remains in effect"}
```

## Appeal Routing

1. User submits appeal through app-api (future endpoint)
2. System validates:
   - User is the sanctioned party (`appellant_user_id == user_id`)
   - No active appeal exists for this sanction/case
   - Sanction is still active (not expired or lifted)
3. Appeal created with status `open`
4. Moderation queue item created with `source_type: appeal`
5. Senior moderator reviews and decides:
   - `accepted` - Lift sanction, update `prm_sanction.lifted_at`
   - `rejected` - Keep sanction, notify user
   - `withdrawn` - User withdraws appeal

## Audit Export Commands

Export moderation audit data for compliance:

```bash
# Export all decisions for a date range
SELECT mc.case_no, md.decision_action, md.reason_code, md.note,
       md.decided_by, md.before_state, md.after_state, md.created_at
FROM prm_moderation_decision md
JOIN prm_moderation_case mc ON mc.id = md.case_id
WHERE md.tenant_id = :tenant_id
  AND md.created_at BETWEEN :start_date AND :end_date
ORDER BY md.created_at;

# Export sanctions with appeal status
SELECT fs.sanction_type, fs.reason_code, fs.starts_at, fs.expires_at,
       fs.lifted_at, fa.appeal_status, fa.reviewed_at
FROM prm_sanction fs
LEFT JOIN prm_appeal fa ON fa.sanction_id = fs.id
WHERE fs.tenant_id = :tenant_id
  AND fs.created_at BETWEEN :start_date AND :end_date
ORDER BY fs.created_at;

# Export queue SLA compliance
SELECT severity, queue_status,
       AVG(EXTRACT(EPOCH FROM (updated_at - created_at))) as avg_resolution_seconds,
       COUNT(*) as item_count
FROM prm_moderation_queue_item
WHERE tenant_id = :tenant_id
  AND created_at BETWEEN :start_date AND :end_date
GROUP BY severity, queue_status;
```
