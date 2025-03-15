# Notion to Obsidian RS

NotionのページをObsidianのマークダウン形式に変換するRust製のツールです。

## 機能

- NotionのページをObsidianのマークダウン形式に変換
- フロントマターの自動生成（作成日時、タグ、URL等）
- 豊富なブロック形式のサポート：
  - 見出し（H1, H2, H3）
  - リスト（箇条書き、番号付き）
  - チェックボックス
  - コードブロック
  - 引用
  - コールアウト
  - 表
  - 画像
  - 動画
  - ブックマーク
  - 埋め込み
- Notionのタグをオブサイディアンのタグに変換

## 必要条件

- Rust 2021 Edition以降
- Notionのインテグレーショントークン
- Obsidianのバルトの設定

## 環境変数の設定

`.env`ファイルを作成し、以下の環境変数を設定してください：

```env
NOTION_TOKEN=your_notion_integration_token
OBSIDIAN_DIR=/path/to/your/obsidian/vault
ALL_DATABASE_ID=your_notion_database_id
TAG_DATABASE_ID=your_tag_database_id
```

各環境変数の説明：
- `NOTION_TOKEN`: NotionのAPIトークン
- `OBSIDIAN_DIR`: Obsidianバルトのディレクトリパス
- `ALL_DATABASE_ID`: 変換対象のNotionデータベースID
- `TAG_DATABASE_ID`: タグ管理用のNotionデータベースID

## 使用方法

### 特定のページを変換

```bash
cargo run -- --page <page_id>
```

### 複数のページを一括変換

```bash
cargo run -- --limit <number>
```

`number`は変換するページ数を指定します。

## 変換サポート

### フロントマター
- タイプ（タグ）
- URL
- 作成日時

### ブロックタイプ
- 段落
- 見出し（H1-H3）
- 箇条書きリスト
- 番号付きリスト
- チェックボックス
- トグル
- 引用
- コードブロック（言語指定対応）
- コールアウト
- 画像
- 動画
- ブックマーク
- リンクプレビュー
- 区切り線
- 表
- 埋め込み

## ライセンス

MIT

## 注意事項

- 一度に大量のページを変換する場合は、NotionのAPIレート制限に注意してください
- 変換前に必ずObsidianバルトのバックアップを取ることをお勧めします