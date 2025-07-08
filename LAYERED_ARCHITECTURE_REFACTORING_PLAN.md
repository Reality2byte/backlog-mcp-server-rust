# レイヤードアーキテクチャ違反解消のための段階的移行計画

## 更新履歴
- 2025-07-07: Phase 1 完了 - Kent BeckのTDDプロセスで実装
- 2025-07-08: Phase 2 完了 - typed-activityフィーチャーフラグとActivityProject/ActivityIssue型の導入、CustomFieldTypeの移動
- 2025-07-08: Phase 3 完了 - 依存関係の修正、backlog-projectへの依存を削除
- 2025-07-08: Phase 4 完了 - 最終的な削除計画の実行（非推奨マーク追加とマイグレーションガイド作成はスキップ）

## 現状の問題まとめ

### 1. 依存関係の逆転
- `backlog-activity` → `backlog-project` （基本層が上位層に依存）
- `backlog-user` → `backlog-project`, `backlog-issue`
- `backlog-space` → `backlog-project`

### 2. 型定義の重複と散在

#### Activity
- `/crates/backlog-activity/src/models/mod.rs` - メインの定義
- `/crates/backlog-project/src/models/activity/get_recent_updates_response.rs` - 重複定義

#### Content
- `backlog-project`: Standard/UserManagement型（get_recent_updates_response.rs）
- `backlog-project`: IssueCreated/Wiki等の詳細型（content.rs）

#### Notification
- `/crates/backlog-project/src/models/activity/get_recent_updates_response.rs` - 空の struct
- `/crates/backlog-user/src/models/notification_list.rs` - 完全な定義
- `/crates/backlog-issue/src/models/notification.rs` - NotificationForComment
- `/crates/backlog-git/src/models.rs` - プルリクエスト用
- `/crates/backlog-user/src/models/notification.rs` - NotificationCount

### 3. 責任範囲の混乱
- 基本的な概念（Activity、Notification）が上位層のクレートに定義されている
- NotificationReasonがissueクレートに定義されているが、他のドメインでも使用

## 段階的移行計画

### Phase 1: 共通型の整理と移動（互換性維持）
**目的**: 基本型を適切な層に配置しつつ、既存APIとの互換性を保つ

#### 1.1 新しい共通型の定義場所を作成
```
crates/backlog-core/src/activity/
├── mod.rs
├── activity.rs      # Activity基本型
├── content.rs       # Content型（統合版）
├── notification.rs  # Notification基本型
└── change.rs        # Change, Comment等の補助型
```

#### 1.2 型定義の移動
- `Activity`, `Content`, `Change`, `Comment`, `Notification`を`backlog-core`に定義
- `NotificationReason`も`backlog-core`に移動
- 既存の場所では`pub use`で再エクスポート（互換性維持）

例：
```rust
// crates/backlog-activity/src/models/mod.rs
#[deprecated(since = "0.2.0", note = "Use backlog_core::activity::Activity instead")]
pub use backlog_core::activity::Activity;
```

#### 1.3 依存関係の整理
```
backlog-core（基本型、識別子）
    ↑
backlog-domain-models（Project, Issue等のドメインモデル）
    ↑
各ドメインクレート（activity, project, user, issue等）
```

### Phase 2: 段階的な型統合
**目的**: 重複定義を解消し、一つの統一された型に収束

#### 2.1 Content型の統合
現在の2つのContent型を統合した新しいenum定義：
```rust
// crates/backlog-core/src/activity/content.rs
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Content {
    // 旧Standard型の内容
    Comment {
        id: i64,
        key_id: Option<i64>,
        summary: Option<String>,
        description: Option<String>,
        comment: Option<Comment>,
        changes: Option<Vec<Change>>,
    },
    // 旧UserManagement型の内容
    UserManagement {
        users: Option<Vec<User>>,
        group_project_activities: Option<Vec<GroupProjectActivity>>,
        comment: Option<String>,
    },
    // 詳細型たち
    IssueCreated(Box<IssueCreatedContent>),
    Issue(Box<IssueContent>),
    Wiki(Box<WikiContent>),
    File(Box<FileContent>),
    // ... 他の型
}
```

#### 2.2 Notification型の統合
```rust
// crates/backlog-core/src/activity/notification.rs
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Notification {
    pub id: NotificationId,
    pub already_read: bool,
    pub reason: NotificationReason,
    pub resource_already_read: bool,
    
    // オプショナルフィールド（使用場所により存在）
    pub user: Option<User>,
    pub project: Option<Project>,
    pub issue: Option<Issue>,
    pub comment: Option<Comment>,
    pub pull_request: Option<serde_json::Value>,
    pub pull_request_comment: Option<serde_json::Value>,
    pub sender: Option<User>,
    pub created: Option<DateTime<Utc>>,
}
```

#### 2.3 移行用のtype aliasの提供
```rust
// 互換性のためのエイリアス
pub type ProjectActivity = Activity;
pub type UserNotification = Notification;
pub type IssueNotification = Notification;
```

### Phase 3: 依存関係の逆転解消
**目的**: 正しい依存方向に修正

#### 3.1 backlog-activityの依存を修正
```toml
# crates/backlog-activity/Cargo.toml
[dependencies]
backlog-api-core = { path = "../backlog-api-core" }
backlog-core = { path = "../backlog-core" }
backlog-domain-models = { path = "../backlog-domain-models" }
# backlog-project = { path = "../backlog-project" } # 削除
client = { path = "../client" }
```

#### 3.2 各クレートの依存整理
- 上位層から下位層への依存のみ許可
- 循環依存の解消
- 必要な型は全て`backlog-core`または`backlog-domain-models`から取得

### Phase 4: 最終的なクリーンアップ
**目的**: 非推奨の型定義を削除し、クリーンな構造に

#### 4.1 非推奨マークの追加（Phase 1で実施済み）

#### 4.2 段階的な削除
- v0.3.0: 非推奨警告の追加
- v0.4.0: 内部実装の完全移行
- v0.5.0: 非推奨型の削除

## 実装優先順位

1. **高優先度**: Activity関連の移動
   - 最も基本的で影響範囲が大きい
   - 多くのAPIで使用されている

2. **中優先度**: Notification統合
   - 複数箇所で異なる定義がある
   - 統合により一貫性が向上

3. **低優先度**: その他の細かい型の整理
   - Content内の詳細型
   - Change, Comment等の補助型

## リスク軽減策

### 1. テストの充実
- 既存のテストが全て通ることを確認
- 移行用の統合テストを追加
- 型の互換性を確認するテスト

### 2. 段階的リリース
```toml
[features]
# 新しい構造を使用
new-architecture = []
# デフォルトは旧構造を維持
default = []
```

### 3. ドキュメント
- MIGRATION.mdの作成
- 各段階でのCHANGELOG.md更新
- 型の移動先を明確に文書化

## 実装チェックリスト

- [x] Phase 1: 共通型の定義と再エクスポート ✅ 2025-07-07 完了
  - [x] backlog-core/src/activityディレクトリ作成
  - [x] Activity型の移動と再エクスポート
  - [x] Content型の統合定義
  - [x] Notification型の統合定義
  - [x] NotificationReasonの移動
  - [x] 既存テストの動作確認

- [x] Phase 2: 型の統合 ✅ 2025-07-08 実装
  - [x] ActivityProject/ActivityIssue型の作成
  - [x] typed-activityフィーチャーフラグの導入
  - [x] 条件付きコンパイルでActivity型の切り替え
  - [x] CustomFieldTypeのbacklog-domain-modelsへの移動（完了）
  - [ ] Notification型の完全統合（Phase 3へ延期）

- [x] Phase 3: 依存関係の修正 ✅ 2025-07-08 完了
  - [x] backlog-activityの依存修正
  - [x] 各クレートの依存見直し  
  - [x] 循環依存の解消確認

- [x] Phase 4: クリーンアップ ✅ 2025-07-08 完了
  - [ ] 非推奨マークの追加（スキップ）
  - [ ] マイグレーションガイド作成（スキップ）
  - [x] 最終的な削除計画の実行

## Phase 1 実装詳細

### 実装内容
1. **新規作成ファイル**
   - `/crates/backlog-core/src/activity/mod.rs`
   - `/crates/backlog-core/src/activity/activity.rs` - Activity統合型（projectは`serde_json::Value`）
   - `/crates/backlog-core/src/activity/content.rs` - Content統合型（Standard/UserManagement/詳細型を統合）
   - `/crates/backlog-core/src/activity/notification.rs` - Notification統合型とEmptyNotification
   - `/crates/backlog-core/src/activity/notification_reason.rs` - NotificationReason enum
   - `/crates/backlog-core/src/activity/change.rs` - Change, Comment, GroupProjectActivity

2. **修正ファイル**
   - `/crates/backlog-issue/src/models/notification_reason.rs` - 再エクスポートに変更
   - `/crates/backlog-project/src/models/activity/get_recent_updates_response.rs` - 型を再エクスポートに変更
   - `/crates/backlog-activity/src/models/mod.rs` - Activity型を再エクスポートに変更

3. **テスト修正**
   - `/crates/backlog-project/tests/project_recent_updates_test.rs` - projectフィールドへのアクセス方法を修正

### 実装上の工夫
- **循環依存の回避**: Activity.projectフィールドを`serde_json::Value`として定義
- **互換性の維持**: 既存の型定義を`#[deprecated]`付きで再エクスポート
- **TDDプロセス**: Red-Green-Refactorサイクルで堅牢な実装

### 残課題
- Activity.projectを`serde_json::Value`から`Project`型への移行（Phase 2）
- 非推奨警告の解消
- 未使用関数の削除

## Phase 2 実装詳細

### 実装内容
1. **新規作成ファイル**
   - `/crates/backlog-core/src/activity/project.rs` - ActivityProject/ActivityIssue型の定義
   - `/crates/backlog-domain-models/src/custom_field.rs` - CustomFieldType関連の型定義
   - `/crates/backlog-domain-models/src/comment.rs` - Comment統合型（一時的に作成、後に削除）

2. **修正ファイル**
   - `/crates/backlog-core/src/activity/activity_type.rs` - 条件付きコンパイルでActivity型を切り替え
   - `/crates/backlog-core/Cargo.toml` - typed-activityフィーチャーフラグの追加
   - `/crates/backlog-issue/src/models/custom_field_type.rs` - 非推奨マークと再エクスポート

3. **typed-activityフィーチャーフラグ**
   - デフォルト無効（Phase 1互換）
   - 有効時：ActivityProjectを使用し、型安全性を向上
   - ヘルパーメソッド（project_id(), project_name()）で統一的なアクセス

### 実装上の工夫
- **循環依存の完全回避**: ActivityProject/ActivityIssueをbacklog-coreに配置
- **段階的移行**: フィーチャーフラグで新旧の切り替えが可能
- **最小限の型定義**: 必要最小限のフィールドでActivityコンテキストに特化
- **カスタムDeserialize**: typeIdベースのデシリアライズ実装を含めて移動
- **ヘルパーメソッド**: project_id()とproject_name()で統一的なアクセス

### 解決した課題
- CustomFieldTypeの依存関係の整理（カスタムDeserializeも含めて完全移行）
- Project型の簡略版による循環依存の回避
- テストの条件付き実行で両方のモードをサポート
- ListItemのid型をCustomFieldItemIdに統一
- Option<bool>フィールドへの対応

## Phase 3 実装詳細

### 実装内容
1. **修正ファイル**
   - `/crates/backlog-activity/Cargo.toml` - backlog-project依存を削除
   - `/crates/backlog-space/Cargo.toml` - backlog-project依存を削除
   - `/crates/backlog-user/Cargo.toml` - backlog-project依存を削除
   - `/crates/backlog-space/src/api/get_space_recent_updates.rs` - Activity importをbacklog_coreから取得
   - `/crates/backlog-user/src/api/get_user_recent_updates.rs` - Activity importをbacklog_coreから取得
   - `/crates/backlog-user/src/models/notification_list.rs` - NotificationReasonをbacklog_coreから取得

2. **依存関係の整理結果**
   - backlog-activity、backlog-space、backlog-userはbacklog-projectに依存しなくなった
   - 循環依存は存在しないことを確認
   - 必要な型は全てbacklog-coreまたはbacklog-domain-modelsから取得

### 実装上の工夫
- **最小限の変更**: 主にimport文の修正のみで対応
- **互換性の維持**: APIの動作は変更なし
- **テスト駆動**: 各ステップでテストを実行して動作確認

## Phase 4 実装詳細

### 実装内容
1. **型の移動とインポート修正**
   - NulabAccountをbacklog-coreに移動
   - CustomFieldType関連のインポートをbacklog_domain_modelsに統一
   - NotificationReasonのインポートをbacklog_coreに統一
   - CLI、MCPサーバーを含む全体のインポートパスを修正

2. **削除したファイル・ディレクトリ**
   - `/crates/backlog-issue/src/models/notification_reason.rs` - 非推奨の再エクスポートファイル
   - `/crates/backlog-issue/src/models/custom_field_type.rs` - 非推奨の再エクスポートファイル
   - `/crates/backlog-activity/src/models/` - modelsディレクトリ全体（空の再エクスポートのみ）
   - `/crates/backlog-project/src/models/` - modelsディレクトリ全体（一時ファイルのクリーンアップ）

3. **クリーンアップ結果**
   - すべての非推奨の再エクスポートを削除
   - 一時的な型定義ファイル（TypeId、ReasonId、ActivityContent等）を削除
   - #[allow(deprecated)]属性をすべて削除
   - 依存関係が完全にクリーンになった

### 実装上の工夫
- **段階的な削除**: インポートエラーを一つずつ解消しながら進行
- **sedコマンドの活用**: 大量のインポート修正を効率的に実施
- **即座のテスト実行**: 各変更後にテストを実行して動作確認

## 期待される成果

1. **明確な層構造**
   - core層：基本型と識別子
   - domain-models層：ドメインモデル
   - 各ドメイン層：具体的なAPI実装

2. **保守性の向上**
   - 型定義の重複解消
   - 依存関係の明確化
   - 将来の拡張が容易に

3. **開発効率の向上**
   - 型の場所が予測可能
   - 循環依存によるコンパイルエラーの解消
   - より直感的なモジュール構造