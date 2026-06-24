export interface PromptsAppSdkDependencies {
  forum: PromptsGeneratedClient;
  appbase: AppbaseClient;
  drive: DriveClient;
  search: SearchClient;
  messaging: MessagingClient;
}

export interface PromptsGeneratedClient {
  topics: {
    list(params: { boardId?: string; cursor?: string; limit?: number; sort?: string }): Promise<TopicPage>;
    create(body: CreateTopicRequest): Promise<PromptsTopic>;
    retrieve(topicId: string): Promise<PromptsTopic>;
    update(topicId: string, body: UpdateTopicRequest): Promise<PromptsTopic>;
    delete(topicId: string): Promise<void>;
    replies: {
      list(topicId: string, params?: { cursor?: string; limit?: number }): Promise<ReplyPage>;
      create(topicId: string, body: CreateReplyRequest): Promise<PromptsReply>;
    };
    revisions: {
      list(topicId: string, params?: { cursor?: string; limit?: number }): Promise<RevisionPage>;
    };
  };
  replies: {
    update(replyId: string, body: UpdateReplyRequest): Promise<PromptsReply>;
    delete(replyId: string): Promise<void>;
    revisions: {
      list(replyId: string, params?: { cursor?: string; limit?: number }): Promise<RevisionPage>;
    };
  };
  nodes: {
    tree(params?: { spaceId?: string; parentId?: string }): Promise<PromptsNode[]>;
  };
  questions: {
    acceptedReply: {
      update(topicId: string, body: { replyId: string }): Promise<PromptsTopic>;
      delete(topicId: string): Promise<void>;
    };
  };
  polls: {
    votes: {
      create(pollId: string, body: { optionIds: string[] }): Promise<void>;
    };
  };
  reactions: { create(body: { targetType: string; targetId: string; reactionType: string }): Promise<void> };
  votes: { create(body: { targetType: string; targetId: string; voteValue: number }): Promise<void> };
  bookmarks: { create(body: { targetType: string; targetId: string; note?: string }): Promise<void> };
  readState: {
    topics: { update(topicId: string, body?: { lastReadReplyId?: string }): Promise<void> };
  };
  reports: { create(body: CreateReportRequest): Promise<void> };
  feed: { list(params?: { feedType?: string; cursor?: string; limit?: number }): Promise<FeedPage> };
  search: { query(params: { q: string; cursor?: string; limit?: number }): Promise<SearchResultPage> };
}

export interface AppbaseClient {
  getCurrentUser(): Promise<{ userId: string; tenantId: string; organizationId: string }>;
}

export interface DriveClient {
  validateMediaReference(mediaResourceId: string): Promise<boolean>;
  createDownloadGrant(mediaResourceId: string): Promise<{ grantId: string; url: string }>;
}

export interface SearchClient {
  indexDocument(sourceType: string, sourceId: string): Promise<void>;
  deleteDocument(sourceType: string, sourceId: string): Promise<void>;
}

export interface MessagingClient {
  publishEvent(eventType: string, aggregateId: string): Promise<void>;
}

export interface PromptsTopic {
  id: string;
  uuid: string;
  boardId: string;
  title: string;
  bodyFormat: string;
  body: string;
  topicType: string;
  moderationStatus: string;
  visibility: string;
  version: number;
  createdAt: string;
  updatedAt: string;
}

export interface PromptsReply {
  id: string;
  uuid: string;
  topicId: string;
  replyNo: number;
  bodyFormat: string;
  body: string;
  moderationStatus: string;
  version: number;
  createdAt: string;
  updatedAt: string;
}

export interface PromptsNode {
  id: string;
  parentId: string | null;
  nodeType: string;
  slug: string;
  name: string;
  levelNo: number;
  sortOrder: number;
}

export interface CreateTopicRequest {
  boardId: string;
  title: string;
  bodyFormat: string;
  body: string;
  tagIds?: string[];
  prefixId?: string;
  topicType?: string;
  visibility?: string;
}

export interface UpdateTopicRequest {
  title?: string;
  bodyFormat?: string;
  body?: string;
  editReason?: string;
}

export interface CreateReplyRequest {
  parentReplyId?: string;
  bodyFormat: string;
  body: string;
}

export interface UpdateReplyRequest {
  bodyFormat?: string;
  body: string;
  editReason?: string;
}

export interface CreateReportRequest {
  targetType: string;
  targetId: string;
  reasonCode: string;
  description?: string;
}

export interface TopicPage {
  items: PromptsTopic[];
  nextCursor: string | null;
  hasMore: boolean;
}

export interface ReplyPage {
  items: PromptsReply[];
  nextCursor: string | null;
  hasMore: boolean;
}

export interface RevisionPage {
  items: Array<{
    id: string;
    revisionNo: number;
    editorUserId: string;
    title?: string;
    bodyFormat: string;
    body: string;
    editReason: string | null;
    createdAt: string;
  }>;
  nextCursor: string | null;
  hasMore: boolean;
}

export interface FeedPage {
  items: Array<{
    id: string;
    feedType: string;
    topicId: string;
    rankScore: string;
    activityAt: string;
  }>;
  nextCursor: string | null;
  hasMore: boolean;
}

export interface SearchResultPage {
  items: Array<{
    id: string;
    sourceType: string;
    sourceId: string;
    title: string | null;
    visibility: string;
  }>;
  nextCursor: string | null;
  hasMore: boolean;
}

export class PromptsAppFacade {
  constructor(private readonly deps: PromptsAppSdkDependencies) {}

  async listBoardTopics(boardId: string, params?: { cursor?: string; limit?: number; sort?: string }): Promise<TopicPage> {
    return this.deps.prompts.topics.list({ boardId, ...params });
  }

  async createTopic(body: CreateTopicRequest): Promise<PromptsTopic> {
    if (body.tagIds && body.tagIds.length > 0) {
      for (const tagId of body.tagIds) {
        await this.deps.drive.validateMediaReference(tagId);
      }
    }
    return this.deps.prompts.topics.create(body);
  }

  async createReply(topicId: string, body: CreateReplyRequest): Promise<PromptsReply> {
    const user = await this.deps.appbase.getCurrentUser();
    if (!user.userId) {
      throw new Error("Authentication required to create reply");
    }
    return this.deps.prompts.topics.replies.create(topicId, body);
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

  async listReplies(topicId: string, params?: { cursor?: string; limit?: number }): Promise<ReplyPage> {
    return this.deps.prompts.topics.replies.list(topicId, params);
  }

  async updateReply(replyId: string, body: UpdateReplyRequest): Promise<PromptsReply> {
    return this.deps.prompts.replies.update(replyId, body);
  }

  async deleteReply(replyId: string): Promise<void> {
    return this.deps.prompts.replies.delete(replyId);
  }

  async listTopicRevisions(topicId: string, params?: { cursor?: string; limit?: number }): Promise<RevisionPage> {
    return this.deps.prompts.topics.revisions.list(topicId, params);
  }

  async listReplyRevisions(replyId: string, params?: { cursor?: string; limit?: number }): Promise<RevisionPage> {
    return this.deps.prompts.replies.revisions.list(replyId, params);
  }

  async acceptReply(topicId: string, replyId: string): Promise<PromptsTopic> {
    return this.deps.prompts.questions.acceptedReply.update(topicId, { replyId });
  }

  async clearAcceptedReply(topicId: string): Promise<void> {
    return this.deps.prompts.questions.acceptedReply.delete(topicId);
  }

  async votePoll(pollId: string, optionIds: string[]): Promise<void> {
    return this.deps.prompts.polls.votes.create(pollId, { optionIds });
  }

  async createReaction(targetType: string, targetId: string, reactionType: string): Promise<void> {
    return this.deps.prompts.reactions.create({ targetType, targetId, reactionType });
  }

  async createVote(targetType: string, targetId: string, voteValue: number): Promise<void> {
    return this.deps.prompts.votes.create({ targetType, targetId, voteValue });
  }

  async updateBookmark(targetType: string, targetId: string, note?: string): Promise<void> {
    return this.deps.prompts.bookmarks.create({ targetType, targetId, note });
  }

  async updateReadState(topicId: string, lastReadReplyId?: string): Promise<void> {
    return this.deps.prompts.readState.topics.update(topicId, { lastReadReplyId });
  }

  async createReport(body: CreateReportRequest): Promise<void> {
    return this.deps.prompts.reports.create(body);
  }

  async listFeed(params?: { feedType?: string; cursor?: string; limit?: number }): Promise<FeedPage> {
    return this.deps.prompts.feed.list(params);
  }

  async search(query: string, params?: { cursor?: string; limit?: number }): Promise<SearchResultPage> {
    return this.deps.prompts.search.query({ q: query, ...params });
  }

  async listNodeTree(params?: { spaceId?: string; parentId?: string }): Promise<PromptsNode[]> {
    return this.deps.prompts.nodes.tree(params);
  }
}
