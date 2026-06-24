export interface PromptsOpenSdkDependencies {
  forum: PromptsOpenGeneratedClient;
}

export interface PromptsOpenGeneratedClient {
  boards: {
    list(siteSlug: string, params?: { cursor?: string; limit?: number }): Promise<BoardPage>;
    topics: {
      list(siteSlug: string, boardId: string, params?: { cursor?: string; limit?: number }): Promise<PublicTopicPage>;
    };
  };
  topics: {
    list(siteSlug: string, params?: { cursor?: string; limit?: number }): Promise<PublicTopicPage>;
    retrieve(siteSlug: string, topicId: string): Promise<PublicTopic>;
    bySlug: { retrieve(siteSlug: string, topicSlug: string): Promise<PublicTopic> };
    replies: {
      list(siteSlug: string, topicId: string, params?: { cursor?: string; limit?: number }): Promise<PublicReplyPage>;
    };
  };
  tags: {
    list(siteSlug: string, params?: { cursor?: string; limit?: number }): Promise<TagPage>;
  };
  search: {
    query(siteSlug: string, params: { q: string; cursor?: string; limit?: number }): Promise<PublicSearchResultPage>;
  };
}

export interface PublicBoard {
  id: string;
  slug: string;
  name: string;
  description: string | null;
  topicCount: number;
  replyCount: number;
  lastActivityAt: string | null;
}

export interface PublicTopic {
  id: string;
  uuid: string;
  boardId: string;
  title: string;
  slug: string | null;
  excerpt: string | null;
  authorDisplayName: string;
  tagSlugs: string[];
  replyCount: number;
  viewCount: number;
  lastActivityAt: string;
  createdAt: string;
}

export interface PublicReply {
  id: string;
  uuid: string;
  topicId: string;
  replyNo: number;
  bodyFormat: string;
  body: string;
  authorDisplayName: string;
  createdAt: string;
}

export interface PublicTag {
  id: string;
  slug: string;
  name: string;
  description: string | null;
  color: string | null;
  usageCount: number;
}

export interface CursorPage<T> {
  items: T[];
  nextCursor: string | null;
  hasMore: boolean;
}

export type BoardPage = CursorPage<PublicBoard>;
export type PublicTopicPage = CursorPage<PublicTopic>;
export type PublicReplyPage = CursorPage<PublicReply>;
export type TagPage = CursorPage<PublicTag>;
export type PublicSearchResultPage = CursorPage<{
  id: string;
  sourceType: string;
  sourceId: string;
  title: string | null;
  excerpt: string | null;
  authorDisplayName: string;
  boardId: string;
  rankScore: string;
}>;

export class PromptsOpenFacade {
  constructor(private readonly deps: PromptsOpenSdkDependencies) {}

  async listPublicTopics(siteSlug: string, params?: { cursor?: string; limit?: number }): Promise<PublicTopicPage> {
    return this.deps.prompts.topics.list(siteSlug, params);
  }

  async listBoards(siteSlug: string, params?: { cursor?: string; limit?: number }): Promise<BoardPage> {
    return this.deps.prompts.boards.list(siteSlug, params);
  }

  async listBoardTopics(siteSlug: string, boardId: string, params?: { cursor?: string; limit?: number }): Promise<PublicTopicPage> {
    return this.deps.prompts.boards.topics.list(siteSlug, boardId, params);
  }

  async retrieveTopic(siteSlug: string, topicId: string): Promise<PublicTopic> {
    return this.deps.prompts.topics.retrieve(siteSlug, topicId);
  }

  async retrieveTopicBySlug(siteSlug: string, topicSlug: string): Promise<PublicTopic> {
    return this.deps.prompts.topics.bySlug.retrieve(siteSlug, topicSlug);
  }

  async listTopicReplies(siteSlug: string, topicId: string, params?: { cursor?: string; limit?: number }): Promise<PublicReplyPage> {
    return this.deps.prompts.topics.replies.list(siteSlug, topicId, params);
  }

  async listTags(siteSlug: string, params?: { cursor?: string; limit?: number }): Promise<TagPage> {
    return this.deps.prompts.tags.list(siteSlug, params);
  }

  async search(siteSlug: string, query: string, params?: { cursor?: string; limit?: number }): Promise<PublicSearchResultPage> {
    return this.deps.prompts.search.query(siteSlug, { q: query, ...params });
  }
}
