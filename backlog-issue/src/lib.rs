pub mod api;
pub mod models;
pub mod requests;
pub mod responses;

pub use api::IssueApi;

#[cfg(test)]
mod tests {
    use crate::models::Issue;

    #[test]
    fn test_issue_deserialization() {
        let json_data = "{\"id\":913,\"projectId\":14165,\"issueKey\":\"MFP-1\",\"keyId\":1,\"issueType\":{\"id\":56740,\"projectId\":14165,\"name\":\"タスク\",\"color\":\"#7ea800\",\"displayOrder\":0},\"summary\":\"Star Mail Two\",\"description\":\"\",\"resolution\":{\"id\":0,\"name\":\"対応済み\"},\"priority\":{\"id\":3,\"name\":\"中\"},\"status\":{\"id\":4,\"projectId\":14165,\"name\":\"完了\",\"color\":\"#b0be3c\",\"displayOrder\":4000},\"assignee\":{\"id\":73420,\"userId\":\"kan_103\",\"name\":\"[株式会社ヌーラボ] 山本\",\"roleType\":4,\"lang\":null,\"mailAddress\":\"kan+103@nulab.com\",\"nulabAccount\":null,\"keyword\":\"[株式会社ヌーラボ] 山本 [KABUSHIKIGAISYANU-RABO] YAMAMOTO\",\"lastLoginTime\":\"2023-09-12T00:53:54Z\"},\"category\":[{\"id\":30446,\"projectId\":14165,\"name\":\" \\\"<img src=u onerror=alert(1)>\",\"displayOrder\":2147483646}],\"versions\":[],\"milestone\":[{\"id\":46108,\"projectId\":14165,\"name\":\"a\",\"description\":null,\"startDate\":\"2022-06-10T00:00:00Z\",\"releaseDueDate\":\"2022-06-24T00:00:00Z\",\"archived\":false,\"displayOrder\":0},{\"id\":108,\"projectId\":14165,\"name\":\"Milestone-1\",\"description\":null,\"startDate\":\"2022-10-12T00:00:00Z\",\"releaseDueDate\":\"2022-10-31T00:00:00Z\",\"archived\":false,\"displayOrder\":1}],\"startDate\":null,\"dueDate\":null,\"estimatedHours\":null,\"actualHours\":null,\"parentIssueId\":null,\"createdUser\":{\"id\":128,\"userId\":\"matsu_viewer\",\"name\":\"Mac viewer\",\"roleType\":4,\"lang\":null,\"mailAddress\":\"matsumoto+deleted@nulab.co.jp\",\"nulabAccount\":null,\"keyword\":\"Mac viewer Mac viewer\",\"lastLoginTime\":\"2021-09-24T00:59:23Z\"},\"created\":\"2019-05-14T07:37:40Z\",\"updatedUser\":{\"id\":99,\"userId\":\"matsumoto\",\"name\":\"山田太朗\",\"roleType\":1,\"lang\":\"ja\",\"mailAddress\":\"matsumoto+comdev@nulab.com\",\"nulabAccount\":null,\"keyword\":\"山田太朗 YAMADAYAMATATAROU\",\"lastLoginTime\":\"2025-06-10T14:15:00Z\"},\"updated\":\"2025-04-04T14:29:59Z\",\"customFields\":[{\"id\":318302,\"fieldTypeId\":8,\"name\":\"Foobar\",\"value\":null,\"otherValue\":null},{\"id\":338041,\"fieldTypeId\":8,\"name\":\"addable\",\"value\":null,\"otherValue\":null},{\"id\":652666,\"fieldTypeId\":3,\"name\":\"必須じゃない\",\"value\":null}],\"attachments\":[{\"id\":6317,\"name\":\"ほげほげ.png\",\"size\":181715,\"createdUser\":{\"id\":99,\"userId\":\"matsumoto\",\"name\":\"山田太朗\",\"roleType\":1,\"lang\":\"ja\",\"mailAddress\":\"matsumoto+comdev@nulab.com\",\"nulabAccount\":null,\"keyword\":\"山田太朗 YAMADAYAMATATAROU\",\"lastLoginTime\":\"2025-06-10T14:15:00Z\"},\"created\":\"2020-02-14T05:26:25Z\"}],\"sharedFiles\":[],\"externalFileLinks\":[],\"stars\":[{\"id\":166,\"comment\":null,\"url\":\"https://nulab.dev.backlog.com/view/MFP-1\",\"title\":\"[MFP-1] Star Mail Two\",\"presenter\":{\"id\":99,\"userId\":\"matsumoto\",\"name\":\"山田太朗\",\"roleType\":1,\"lang\":\"ja\",\"mailAddress\":\"matsumoto+comdev@nulab.com\",\"nulabAccount\":null,\"keyword\":\"山田太朗 YAMADAYAMATATAROU\",\"lastLoginTime\":\"2025-06-10T14:15:00Z\"},\"created\":\"2019-05-14T07:38:26Z\"}]}"; let issue: Result<Issue, _> = serde_json::from_str(json_data);
        if let Err(e) = &issue {
            println!("Deserialization error: {}", e);
        }
        assert!(issue.is_ok());
        let issue = issue.unwrap();
        assert_eq!(issue.custom_fields.len(), 3);
        assert_eq!(issue.attachments.len(), 1);
        assert_eq!(issue.attachments[0].name, "ほげほげ.png");
        assert_eq!(issue.shared_files.len(), 0);
        assert_eq!(issue.external_file_links.len(), 0);
        assert_eq!(issue.stars.len(), 1);
        assert_eq!(issue.stars[0].id.to_string(), "166");
    }
}
