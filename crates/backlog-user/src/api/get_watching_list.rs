use serde::{Deserialize, Serialize};

use backlog_api_core::IntoRequest;
use backlog_core::identifier::{IssueId, UserId};

/// Sort order for watching list
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Order {
    Asc,
    Desc,
}

impl std::fmt::Display for Order {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Order::Asc => write!(f, "asc"),
            Order::Desc => write!(f, "desc"),
        }
    }
}

/// Sort attribute for watching list
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum WatchingSort {
    Created,
    Updated,
    IssueUpdated,
}

impl std::fmt::Display for WatchingSort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WatchingSort::Created => write!(f, "created"),
            WatchingSort::Updated => write!(f, "updated"),
            WatchingSort::IssueUpdated => write!(f, "issueUpdated"),
        }
    }
}

/// Parameters for getting watching list
#[derive(Debug, Clone, Default)]
pub struct GetWatchingListParams {
    pub order: Option<Order>,
    pub sort: Option<WatchingSort>,
    pub count: Option<u8>,
    pub offset: Option<u64>,
    pub resource_already_read: Option<bool>,
    pub issue_ids: Option<Vec<IssueId>>,
}

impl GetWatchingListParams {
    pub fn builder() -> GetWatchingListParamsBuilder {
        GetWatchingListParamsBuilder::default()
    }
}

impl From<&GetWatchingListParams> for Vec<(String, String)> {
    fn from(params: &GetWatchingListParams) -> Self {
        let mut seq = Vec::new();

        if let Some(order) = &params.order {
            seq.push(("order".to_string(), order.to_string()));
        }

        if let Some(sort) = &params.sort {
            seq.push(("sort".to_string(), sort.to_string()));
        }

        if let Some(count) = params.count {
            seq.push(("count".to_string(), count.to_string()));
        }

        if let Some(offset) = params.offset {
            seq.push(("offset".to_string(), offset.to_string()));
        }

        if let Some(resource_already_read) = params.resource_already_read {
            seq.push((
                "resourceAlreadyRead".to_string(),
                resource_already_read.to_string(),
            ));
        }

        if let Some(issue_ids) = &params.issue_ids {
            for id in issue_ids {
                seq.push(("issueId[]".to_string(), id.to_string()));
            }
        }

        seq
    }
}

/// Builder for GetWatchingListParams
#[derive(Debug, Default)]
pub struct GetWatchingListParamsBuilder {
    order: Option<Order>,
    sort: Option<WatchingSort>,
    count: Option<u8>,
    offset: Option<u64>,
    resource_already_read: Option<bool>,
    issue_ids: Option<Vec<IssueId>>,
}

impl GetWatchingListParamsBuilder {
    pub fn order(mut self, order: Order) -> Self {
        self.order = Some(order);
        self
    }

    pub fn sort(mut self, sort: WatchingSort) -> Self {
        self.sort = Some(sort);
        self
    }

    pub fn count(mut self, count: u8) -> Self {
        self.count = Some(count);
        self
    }

    pub fn offset(mut self, offset: u64) -> Self {
        self.offset = Some(offset);
        self
    }

    pub fn resource_already_read(mut self, resource_already_read: bool) -> Self {
        self.resource_already_read = Some(resource_already_read);
        self
    }

    pub fn issue_ids(mut self, issue_ids: Vec<IssueId>) -> Self {
        self.issue_ids = Some(issue_ids);
        self
    }

    pub fn build(self) -> Result<GetWatchingListParams, backlog_api_core::Error> {
        Ok(GetWatchingListParams {
            order: self.order,
            sort: self.sort,
            count: self.count,
            offset: self.offset,
            resource_already_read: self.resource_already_read,
            issue_ids: self.issue_ids,
        })
    }
}

/// Request wrapper for get watching list API
#[derive(Debug, Clone)]
pub struct GetWatchingListRequest {
    pub user_id: UserId,
    pub params: GetWatchingListParams,
}

impl IntoRequest for GetWatchingListRequest {
    fn path(&self) -> String {
        format!("/api/v2/users/{}/watchings", self.user_id)
    }

    fn to_query(&self) -> impl Serialize {
        let form_params: Vec<(String, String)> = (&self.params).into();
        form_params
    }
}

#[cfg(test)]
#[path = "get_watching_list_test.rs"]
mod tests;
