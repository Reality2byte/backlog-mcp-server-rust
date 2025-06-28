# カスタム属性実装計画（完全版）

## 概要
Backlog APIのカスタム属性（Custom Fields）を完全型安全に実装する計画です。

## 現状の課題
1. `CustomField.value`が`serde_json::Value`で型安全性がない
2. `AddIssueParams`/`UpdateIssueParams`でカスタム属性がサポートされていない
3. 動的なフォームキー（`customField_{id}`）の扱いが困難

## 実装方針：完全型安全アプローチ

### 1. CustomFieldValue Enum型の定義

```rust
// レスポンス用：完全な情報を保持
pub enum CustomFieldValue {
    Text(String),
    TextArea(String),
    Numeric(f64),
    Date(NaiveDate),
    SingleList {
        item: CustomFieldListItem,
        other_value: Option<String>,
    },
    MultipleList {
        items: Vec<CustomFieldListItem>,
        other_value: Option<String>,
    },
    CheckBox(Vec<CustomFieldListItem>),
    Radio {
        item: CustomFieldListItem,
        other_value: Option<String>,
    },
}

// リクエスト用：IDのみを保持
pub enum CustomFieldInput {
    Text(String),
    TextArea(String),
    Numeric(f64),
    Date(NaiveDate),
    SingleList {
        id: u32,
        other_value: Option<String>,
    },
    MultipleList {
        ids: Vec<u32>,
        other_value: Option<String>,
    },
    CheckBox(Vec<u32>),
    Radio {
        id: u32,
        other_value: Option<String>,
    },
}
```

### 2. CustomField構造体の更新

```rust
pub struct CustomField {
    pub id: CustomFieldId,
    pub field_type_id: CustomFieldTypeId,
    pub name: String,
    pub value: CustomFieldValue,  // serde_json::Value から変更
}
```

### 3. カスタムデシリアライゼーション

`CustomFieldTypeId`に基づいて適切な`CustomFieldValue`バリアントにデシリアライズする実装が必要：

```rust
impl<'de> Deserialize<'de> for CustomField {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // 1. まずRawCustomFieldにデシリアライズ
        // 2. field_type_idを見て適切なCustomFieldValueに変換
        // 3. 最終的なCustomFieldを構築
    }
}
```

### 4. AddIssueParams/UpdateIssueParamsへの統合

```rust
pub struct AddIssueParams {
    // 既存フィールド...
    
    #[form(skip)]
    pub custom_fields: Option<HashMap<CustomFieldId, CustomFieldInput>>,
}

impl IntoRequest for AddIssueParams {
    fn to_form(&self) -> impl Serialize {
        let mut params: Vec<(String, String)> = self.into();
        
        // カスタムフィールドの追加
        if let Some(fields) = &self.custom_fields {
            for (id, input) in fields {
                let (value, other) = input.to_form_value();
                
                match input {
                    CustomFieldInput::MultipleList { ids, .. } 
                    | CustomFieldInput::CheckBox(ids) => {
                        // 複数値は個別のパラメータとして追加
                        for id_value in ids {
                            params.push((
                                format!("customField_{}", id), 
                                id_value.to_string()
                            ));
                        }
                    }
                    _ => {
                        params.push((
                            format!("customField_{}", id), 
                            value
                        ));
                    }
                }
                
                if let Some(other_value) = other {
                    params.push((
                        format!("customField_{}_otherValue", id), 
                        other_value
                    ));
                }
            }
        }
        
        params
    }
}
```

### 5. 実装順序

1. **Phase 1: 基本型定義**
   - `custom_field_value.rs`ファイルの作成
   - `CustomFieldValue`と`CustomFieldInput`の定義
   - `CustomFieldListItem`構造体の定義

2. **Phase 2: デシリアライゼーション**
   - `CustomField`のカスタムデシリアライゼーション実装
   - 既存のテストケースが通ることを確認

3. **Phase 3: パラメータ統合**
   - `AddIssueParams`への`custom_fields`フィールド追加
   - フォームシリアライゼーションの実装
   - `UpdateIssueParams`への同様の実装

4. **Phase 4: テスト**
   - 各カスタムフィールドタイプのテストケース作成
   - 実際のAPIとの統合テスト

### 6. 考慮事項

**メリット**
- 完全な型安全性
- コンパイル時のエラー検出
- IDE支援の充実
- ドキュメント性の向上

**課題と対策**
- **フィールドタイプの事前把握**: プロジェクトのカスタムフィールド定義を取得するAPIを先に呼ぶ
- **型不一致エラー**: 明確なエラーメッセージとヘルパー関数の提供
- **複雑性**: 段階的な実装とわかりやすいドキュメント

### 7. 使用例

```rust
// カスタムフィールドの設定
let mut custom_fields = HashMap::new();

// テキストフィールド
custom_fields.insert(
    CustomFieldId::new(1),
    CustomFieldInput::Text("サンプルテキスト".to_string())
);

// 日付フィールド
custom_fields.insert(
    CustomFieldId::new(2),
    CustomFieldInput::Date(NaiveDate::from_ymd_opt(2024, 6, 24).unwrap())
);

// 単一選択リスト
custom_fields.insert(
    CustomFieldId::new(3),
    CustomFieldInput::SingleList {
        id: 123,
        other_value: Some("その他の説明".to_string())
    }
);

// 課題作成パラメータに設定
let params = AddIssueParamsBuilder::default()
    .project_id(ProjectId::new(1))
    .summary("テスト課題")
    .custom_fields(custom_fields)
    .build()?;
```

### 8. CLI統合計画

**コマンドライン引数での指定方法**
```bash
# JSONファイルから読み込み
blg issue add --custom-fields custom_fields.json

# 個別指定（型ヒント付き）
blg issue add \
  --custom-field-text 1="テキスト値" \
  --custom-field-date 2="2024-06-24" \
  --custom-field-single 3="123:その他の説明"
```

**JSON形式の例**
```json
{
  "1": {
    "type": "text",
    "value": "テキスト値"
  },
  "2": {
    "type": "date",
    "value": "2024-06-24"
  },
  "3": {
    "type": "single_list",
    "id": 123,
    "other_value": "その他の説明"
  },
  "4": {
    "type": "multiple_list",
    "ids": [100, 101, 102],
    "other_value": "追加説明"
  }
}
```

### 9. エラーハンドリング

**型不一致エラー**
```rust
pub enum CustomFieldError {
    TypeMismatch {
        field_id: CustomFieldId,
        expected: CustomFieldTypeId,
        actual: CustomFieldTypeId,
    },
    InvalidDateFormat(String),
    InvalidNumericValue(String),
    UnknownFieldId(CustomFieldId),
}
```

**バリデーション関数**
```rust
impl CustomFieldInput {
    pub fn validate_with_type(&self, expected_type: CustomFieldTypeId) -> Result<(), CustomFieldError> {
        // 型チェックロジック
    }
}
```

### 10. 後方互換性

既存のコードとの互換性を保つため：

```rust
impl CustomField {
    /// 既存のserde_json::Valueを返すメソッド（非推奨）
    #[deprecated(note = "Use typed value field instead")]
    pub fn value_as_json(&self) -> serde_json::Value {
        // CustomFieldValueをserde_json::Valueに変換
    }
    
    /// serde_json::ValueからCustomFieldを構築（移行用）
    pub fn from_json_value(
        id: CustomFieldId,
        field_type_id: CustomFieldTypeId,
        name: String,
        value: serde_json::Value,
        other_value: Option<serde_json::Value>
    ) -> Result<Self, CustomFieldError> {
        // 変換ロジック
    }
}
```

### 11. パフォーマンス考慮事項

- デシリアライゼーション時のオーバーヘッドを最小化
- 大量のカスタムフィールドがある場合の効率的な処理
- キャッシング戦略（カスタムフィールド定義のキャッシュ）

### 12. ドキュメント更新

以下のドキュメントを更新：
- API.md: カスタムフィールドサポートの追加
- README.md: 使用例の追加
- CLAUDE.md: 実装詳細の記載
- 各関数のdocstringコメント

## 実装ステップ詳細

### Phase 1: 基本型定義（1日目）
- [x] `custom_field_value.rs`の作成
- [x] `CustomFieldValue` enumの実装
- [x] `CustomFieldInput` enumの実装
- [x] `CustomFieldListItem`構造体の実装
- [x] `mod.rs`での公開設定

### Phase 2: デシリアライゼーション（2-3日目）
- [x] `RawCustomField`構造体の定義
- [x] カスタムDeserialize実装
- [x] 各フィールドタイプのパース処理
- [x] エラーハンドリング
- [x] 既存テストの動作確認

### Phase 3: パラメータ統合（4-5日目）
- [x] `AddIssueParams`の更新
- [x] `UpdateIssueParams`の更新
- [x] フォームシリアライゼーション実装
- [x] 複数値フィールドの特殊処理
- [x] `IntoRequest`実装の更新

### Phase 4: テスト（6-7日目）
- [x] 単体テストの作成
- [x] 統合テストの作成
- [ ] 実APIでの動作確認
- [x] エッジケースのテスト
- [x] パフォーマンステスト

### Phase 5: CLI統合（8-9日目）
- [x] CLIコマンドの設計
- [x] 引数パーサーの実装
- [x] JSONファイル読み込み機能
- [ ] エラーメッセージの改善
- [x] ヘルプドキュメント

### Phase 6: ドキュメント（10日目）
- [ ] API.mdの更新
- [ ] README.mdの更新
- [ ] CLAUDE.mdの更新
- [ ] 例の追加
- [ ] 移行ガイドの作成

## リスクと対策

### 技術的リスク
1. **破壊的変更**: 既存のAPIとの互換性を保つため、移行用メソッドを提供
2. **パフォーマンス低下**: ベンチマークテストで検証、必要に応じて最適化
3. **複雑性の増加**: 明確なドキュメントとサンプルコードを充実

### スケジュールリスク
1. **予期しない技術的課題**: バッファ期間を設定（+2日）
2. **実APIでの問題**: 早期に実環境でのテストを開始
3. **レビュー期間**: 各フェーズ後にレビュー時間を確保

## 成功基準

1. すべてのカスタムフィールドタイプが型安全に扱える
2. 既存のコードが破壊されない
3. パフォーマンスの大幅な低下がない
4. CLIから簡単に使用できる
5. 十分なドキュメントとテストカバレッジ

この計画に基づいて実装を進めます。