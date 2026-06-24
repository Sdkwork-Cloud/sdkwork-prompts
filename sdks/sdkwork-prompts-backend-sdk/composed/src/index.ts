export interface PromptsBackendSdkDependencies {
  forum: PromptsBackendGeneratedClient;
  appbase: AppbaseClient;
  drive: DriveClient;
  search: SearchClient;
  messaging: MessagingClient;
}

export interface PromptsBackendGeneratedClient {
  nodes: {
    list(params?: { cursor?: string; limit?: number }): Promise<NodePage>;
    create(body: CreateNodeRequest): Promise<PromptsNode>;
    update(nodeId: string, body: UpdateNodeRequest): Promise<PromptsNode>;
    delete(nodeId: string): Promise<void>;
  };
  topicPrefixes: {
    list(params?: { boardId?: string; cursor?: string; limit?: number }): Promise<TopicPrefixPage>;
    create(body: CreateTopicPrefixRequest): Promise<PromptsTopicPrefix>;
  };
  topics: {
    list(params?: { cursor?: string; limit?: number }): Promise<TopicPage>;
    retrieve(topicId: string): Promise<PromptsTopic>;
    update(topicId: string, body: UpdateTopicRequest): Promise<PromptsTopic>;
    delete(topicId: string): Promise<void>;
    pin: { create(topicId: string): Promise<void>; delete(topicId: string): Promise<void> };
    feature: { create(topicId: string): Promise<void>; delete(topicId: string): Promise<void> };
    lock: { create(topicId: string): Promise<void>; delete(topicId: string): Promise<void> };
    move: { create(topicId: string, body: { targetBoardId: string }): Promise<void> };
  };
  moderation: {
    queue: { list(params?: { status?: string; severity?: string; cursor?: string; limit?: number }): Promise<ModerationQueuePage> };
    cases: {
      list(params?: { status?: string; cursor?: string; limit?: number }): Promise<ModerationCasePage>;
      create(body: CreateModerationCaseRequest): Promise<ModerationCase>;
      retrieve(caseId: string): Promise<ModerationCase>;
      decisions: { create(caseId: string, body: CreateDecisionRequest): Promise<ModerationDecision> };
    };
  };
  sanctions: {
    list(params?: { userId?: string; cursor?: string; limit?: number }): Promise<SanctionPage>;
    create(body: CreateSanctionRequest): Promise<PromptsSanction>;
    update(sanctionId: string, body: UpdateSanctionRequest): Promise<PromptsSanction>;
  };
  reputation: {
    rules: {
      list(params?: { cursor?: string; limit?: number }): Promise<ReputationRulePage>;
      create(body: CreateReputationRuleRequest): Promise<ReputationRule>;
    };
    ledger: { list(params?: { userId?: string; cursor?: string; limit?: number }): Promise<ReputationLedgerPage> };
  };
  trustLevels: {
    list(params?: { cursor?: string; limit?: number }): Promise<TrustLevelPage>;
    create(body: CreateTrustLevelRequest): Promise<TrustLevel>;
  };
  badges: {
    list(params?: { cursor?: string; limit?: number }): Promise<BadgePage>;
    create(body: CreateBadgeRequest): Promise<PromptsBadge>;
  };
  stats: {
    boards: { list(params?: { cursor?: string; limit?: number }): Promise<BoardStatsPage> };
    topics: { list(params?: { cursor?: string; limit?: number }): Promise<TopicStatsPage> };
  };
  search: { reindex: { create(body?: { scope?: string; boardId?: string }): Promise<void> } };
  audit: { actions: { list(params?: { cursor?: string; limit?: number }): Promise<AuditActionPage> } };
}

export interface AppbaseClient {
  getCurrentUser(): Promise<{ userId: string; tenantId: string; organizationId: string }>;
  getOperatorContext(): Promise<{ operatorId: string; permissions: string[] }>;
}

export interface DriveClient {
  validateMediaReference(mediaResourceId: string): Promise<boolean>;
}

export interface SearchClient {
  indexDocument(sourceType: string, sourceId: string): Promise<void>;
  deleteDocument(sourceType: string, sourceId: string): Promise<void>;
  rebuildIndex(boardId?: string): Promise<void>;
}

export interface MessagingClient {
  publishEvent(eventType: string, aggregateId: string): Promise<void>;
  publishModerationAlert(caseId: string, severity: string): Promise<void>;
}

export interface PromptsNode {
  id: string;
  uuid: string;
  parentId: string | null;
  nodeType: string;
  slug: string;
  name: string;
  description: string | null;
  levelNo: number;
  sortOrder: number;
  status: string;
}

export interface PromptsTopic {
  id: string;
  uuid: string;
  boardId: string;
  title: string;
  bodyFormat: string;
  topicType: string;
  moderationStatus: string;
  version: number;
  createdAt: string;
  updatedAt: string;
}

export interface PromptsTopicPrefix {
  id: string;
  uuid: string;
  boardId: string;
  code: string;
  label: string;
  color: string | null;
  sortOrder: number;
  status: string;
}

export interface ModerationCase {
  id: string;
  uuid: string;
  caseNo: string;
  targetType: string;
  targetId: string;
  caseStatus: string;
  severity: string;
  openedBy: string;
  createdAt: string;
}

export interface ModerationDecision {
  id: string;
  uuid: string;
  caseId: string;
  decisionAction: string;
  reasonCode: string;
  note: string | null;
  decidedBy: string;
  createdAt: string;
}

export interface PromptsSanction {
  id: string;
  uuid: string;
  userId: string;
  sanctionType: string;
  reasonCode: string;
  startsAt: string;
  expiresAt: string | null;
  status: string;
}

export interface ReputationRule {
  id: string;
  uuid: string;
  code: string;
  eventType: string;
  points: number;
  status: string;
}

export interface TrustLevel {
  id: string;
  uuid: string;
  levelNo: number;
  code: string;
  name: string;
  status: string;
}

export interface PromptsBadge {
  id: string;
  uuid: string;
  code: string;
  name: string;
  description: string | null;
  grantMode: string;
  status: string;
}

export interface CreateNodeRequest {
  spaceId: string;
  parentId?: string;
  nodeType: string;
  slug: string;
  name: string;
  description?: string;
  sortOrder?: number;
}

export interface UpdateNodeRequest {
  name?: string;
  description?: string;
  sortOrder?: number;
  parentId?: string;
}

export interface CreateTopicPrefixRequest {
  boardId: string;
  code: string;
  label: string;
  color?: string;
  sortOrder?: number;
}

export interface UpdateTopicRequest {
  title?: string;
  bodyFormat?: string;
  body?: string;
  editReason?: string;
}

export interface CreateModerationCaseRequest {
  targetType: string;
  targetId: string;
  severity: string;
  summary?: string;
}

export interface CreateDecisionRequest {
  decisionAction: string;
  reasonCode: string;
  note?: string;
}

export interface CreateSanctionRequest {
  userId: string;
  caseId?: string;
  sanctionType: string;
  reasonCode: string;
  expiresAt?: string;
}

export interface UpdateSanctionRequest {
  expiresAt?: string;
}

export interface CreateReputationRuleRequest {
  code: string;
  eventType: string;
  points: number;
  dailyLimit?: number;
}

export interface CreateTrustLevelRequest {
  levelNo: number;
  code: string;
  name: string;
}

export interface CreateBadgeRequest {
  code: string;
  name: string;
  description?: string;
  grantMode: string;
}

export interface CursorPage<T> {
  items: T[];
  nextCursor: string | null;
  hasMore: boolean;
}

export type NodePage = CursorPage<PromptsNode>;
export type TopicPage = CursorPage<PromptsTopic>;
export type TopicPrefixPage = CursorPage<PromptsTopicPrefix>;
export type ModerationQueuePage = CursorPage<ModerationCase>;
export type ModerationCasePage = CursorPage<ModerationCase>;
export type SanctionPage = CursorPage<PromptsSanction>;
export type ReputationRulePage = CursorPage<ReputationRule>;
export type ReputationLedgerPage = CursorPage<{ id: string; userId: string; sourceType: string; direction: string; points: number; balanceAfter: number; reasonCode: string; createdAt: string }>;
export type TrustLevelPage = CursorPage<TrustLevel>;
export type BadgePage = CursorPage<PromptsBadge>;
export type BoardStatsPage = CursorPage<{ id: string; boardId: string; topicCount: number; replyCount: number; memberCount: number; lastActivityAt: string | null }>;
export type TopicStatsPage = CursorPage<{ id: string; topicId: string; replyCount: number; viewCount: number; voteScore: number; lastCalculatedAt: string }>;
export type AuditActionPage = CursorPage<{ id: string; action: string; targetType: string; targetId: string; operatorId: string; createdAt: string }>;

export class PromptsBackendFacade {
  constructor(private readonly deps: PromptsBackendSdkDependencies) {}

  async createModerationDecision(caseId: string, body: CreateDecisionRequest): Promise<ModerationDecision> {
    const decision = await this.deps.prompts.moderation.cases.decisions.create(caseId, body);
    await this.deps.messaging.publishModerationAlert(caseId, body.decisionAction);
    return decision;
  }

  async rebuildSearchProjection(body?: { scope?: string; boardId?: string }): Promise<void> {
    await this.deps.prompts.search.reindex.create(body);
    await this.deps.search.rebuildIndex(body?.boardId);
  }

  async listNodes(params?: { cursor?: string; limit?: number }): Promise<NodePage> {
    return this.deps.prompts.nodes.list(params);
  }

  async createNode(body: CreateNodeRequest): Promise<PromptsNode> {
    return this.deps.prompts.nodes.create(body);
  }

  async updateNode(nodeId: string, body: UpdateNodeRequest): Promise<PromptsNode> {
    return this.deps.prompts.nodes.update(nodeId, body);
  }

  async deleteNode(nodeId: string): Promise<void> {
    return this.deps.prompts.nodes.delete(nodeId);
  }

  async listTopicPrefixes(params?: { boardId?: string; cursor?: string; limit?: number }): Promise<TopicPrefixPage> {
    return this.deps.prompts.topicPrefixes.list(params);
  }

  async createTopicPrefix(body: CreateTopicPrefixRequest): Promise<PromptsTopicPrefix> {
    return this.deps.prompts.topicPrefixes.create(body);
  }

  async listTopics(params?: { cursor?: string; limit?: number }): Promise<TopicPage> {
    return this.deps.prompts.topics.list(params);
  }

  async retrieveTopic(topicId: string): Promise<PromptsTopic> {
    return this.deps.prompts.topics.retrieve(topicId);
  }

  async updateTopic(topicId: string, body: UpdateTopicRequest): Promise<PromptsTopic> {
    return this.deps.prompts.topics.update(topicId, body);
  }

  async deleteTopic(topicId: string): Promise<void> {
    return this.deps.prompts.topics.delete(topicId);
  }

  async pinTopic(topicId: string): Promise<void> {
    return this.deps.prompts.topics.pin.create(topicId);
  }

  async unpinTopic(topicId: string): Promise<void> {
    return this.deps.prompts.topics.pin.delete(topicId);
  }

  async featureTopic(topicId: string): Promise<void> {
    return this.deps.prompts.topics.feature.create(topicId);
  }

  async unfeatureTopic(topicId: string): Promise<void> {
    return this.deps.prompts.topics.feature.delete(topicId);
  }

  async lockTopic(topicId: string): Promise<void> {
    return this.deps.prompts.topics.lock.create(topicId);
  }

  async unlockTopic(topicId: string): Promise<void> {
    return this.deps.prompts.topics.lock.delete(topicId);
  }

  async moveTopic(topicId: string, targetBoardId: string): Promise<void> {
    return this.deps.prompts.topics.move.create(topicId, { targetBoardId });
  }

  async listModerationQueue(params?: { status?: string; severity?: string; cursor?: string; limit?: number }): Promise<ModerationQueuePage> {
    return this.deps.prompts.moderation.queue.list(params);
  }

  async listModerationCases(params?: { status?: string; cursor?: string; limit?: number }): Promise<ModerationCasePage> {
    return this.deps.prompts.moderation.cases.list(params);
  }

  async createModerationCase(body: CreateModerationCaseRequest): Promise<ModerationCase> {
    return this.deps.prompts.moderation.cases.create(body);
  }

  async retrieveModerationCase(caseId: string): Promise<ModerationCase> {
    return this.deps.prompts.moderation.cases.retrieve(caseId);
  }

  async listSanctions(params?: { userId?: string; cursor?: string; limit?: number }): Promise<SanctionPage> {
    return this.deps.prompts.sanctions.list(params);
  }

  async createSanction(body: CreateSanctionRequest): Promise<PromptsSanction> {
    return this.deps.prompts.sanctions.create(body);
  }

  async updateSanction(sanctionId: string, body: UpdateSanctionRequest): Promise<PromptsSanction> {
    return this.deps.prompts.sanctions.update(sanctionId, body);
  }

  async listReputationRules(params?: { cursor?: string; limit?: number }): Promise<ReputationRulePage> {
    return this.deps.prompts.reputation.rules.list(params);
  }

  async createReputationRule(body: CreateReputationRuleRequest): Promise<ReputationRule> {
    return this.deps.prompts.reputation.rules.create(body);
  }

  async listReputationLedger(params?: { userId?: string; cursor?: string; limit?: number }): Promise<ReputationLedgerPage> {
    return this.deps.prompts.reputation.ledger.list(params);
  }

  async listTrustLevels(params?: { cursor?: string; limit?: number }): Promise<TrustLevelPage> {
    return this.deps.prompts.trustLevels.list(params);
  }

  async createTrustLevel(body: CreateTrustLevelRequest): Promise<TrustLevel> {
    return this.deps.prompts.trustLevels.create(body);
  }

  async listBadges(params?: { cursor?: string; limit?: number }): Promise<BadgePage> {
    return this.deps.prompts.badges.list(params);
  }

  async createBadge(body: CreateBadgeRequest): Promise<PromptsBadge> {
    return this.deps.prompts.badges.create(body);
  }

  async listBoardStats(params?: { cursor?: string; limit?: number }): Promise<BoardStatsPage> {
    return this.deps.prompts.stats.boards.list(params);
  }

  async listTopicStats(params?: { cursor?: string; limit?: number }): Promise<TopicStatsPage> {
    return this.deps.prompts.stats.topics.list(params);
  }

  async listAuditActions(params?: { cursor?: string; limit?: number }): Promise<AuditActionPage> {
    return this.deps.prompts.audit.actions.list(params);
  }
}
