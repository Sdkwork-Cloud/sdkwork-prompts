#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TenantId(pub i64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OrganizationId(pub i64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UserId(pub i64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TopicId(pub i64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ReplyId(pub i64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BoardId(pub i64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId(pub i64);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cursor(pub Option<String>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PageLimit(pub u16);

impl PageLimit {
    pub const DEFAULT: u16 = 20;
    pub const MAX: u16 = 100;

    pub fn normalize(self) -> u16 {
        self.0.min(Self::MAX).max(1)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModerationStatus {
    Visible,
    Pending,
    Hidden,
    Rejected,
    Deleted,
}

impl ModerationStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Visible => "visible",
            Self::Pending => "pending",
            Self::Hidden => "hidden",
            Self::Rejected => "rejected",
            Self::Deleted => "deleted",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "visible" => Some(Self::Visible),
            "pending" => Some(Self::Pending),
            "hidden" => Some(Self::Hidden),
            "rejected" => Some(Self::Rejected),
            "deleted" => Some(Self::Deleted),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TopicType {
    Discussion,
    Question,
    Poll,
    Announcement,
    Article,
}

impl TopicType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Discussion => "discussion",
            Self::Question => "question",
            Self::Poll => "poll",
            Self::Announcement => "announcement",
            Self::Article => "article",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Visibility {
    Public,
    Members,
    Private,
    Unlisted,
}

impl Visibility {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Public => "public",
            Self::Members => "members",
            Self::Private => "private",
            Self::Unlisted => "unlisted",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NodeType {
    Category,
    Board,
}

impl NodeType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Category => "category",
            Self::Board => "board",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BodyFormat {
    Markdown,
    HtmlSanitized,
    RichTextJson,
}

impl BodyFormat {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Markdown => "markdown",
            Self::HtmlSanitized => "html_sanitized",
            Self::RichTextJson => "rich_text_json",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DataScope {
    Default,
    Private,
    Organization,
    Tenant,
    Public,
}

impl DataScope {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Private => "private",
            Self::Organization => "organization",
            Self::Tenant => "tenant",
            Self::Public => "public",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PromptsRequestContext {
    pub tenant_id: TenantId,
    pub organization_id: OrganizationId,
    pub principal_user_id: UserId,
    pub request_id: Option<String>,
}

impl PromptsRequestContext {
    pub fn new(tenant_id: i64, organization_id: i64, user_id: i64) -> Self {
        Self {
            tenant_id: TenantId(tenant_id),
            organization_id: OrganizationId(organization_id),
            principal_user_id: UserId(user_id),
            request_id: None,
        }
    }

    pub fn with_request_id(mut self, request_id: String) -> Self {
        self.request_id = Some(request_id);
        self
    }

    pub fn tenant_id_value(&self) -> i64 {
        self.tenant_id.0
    }

    pub fn organization_id_value(&self) -> i64 {
        self.organization_id.0
    }

    pub fn user_id_value(&self) -> i64 {
        self.principal_user_id.0
    }
}
