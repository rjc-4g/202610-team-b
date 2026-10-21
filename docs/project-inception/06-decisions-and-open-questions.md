# 06. 決定事項・未確定事項・改善提案

## 決定事項

| 項目 | 方針 |
|---|---|
| テーマ | Rustサーバー監視ダッシュボード |
| 対象リポジトリ | `rjc-4g/202610-team-b` |
| チーム | 開発1名、管理・レビュー1名 |
| 工数 | 合計15人時（開発10、管理5） |
| 学習優先度 | Issues / GitHub Flow / PRレビュー / GitHub Actions / AI支援 |
| 技術 | Rust/Axum、HTML/CSS/JS、Docker Compose |
| MVP | 固定値のCPU・メモリAPIとダッシュボード |
| 追加課題 | cgroup v2実測、グラフ、GHCR公開 |
| ボード | Todo / In Progress / In Review / Done（設定済み） |
| ドキュメント | 既存`docs/`の下に`project-inception/`を追加 |

## 初期検討時の記録

- 対象リポジトリをGitHub連携で検索できた。
- GitHub連携によるIssue新規作成は403エラーとなった。
- 途中の検索で`test`というIssueが1件存在した。
- GitHub Appの個人設定では組織のリポジトリを選択できない画面が共有された。
- GitHub Projectsの空画面が共有された。
- IssueとProjectの作成には`gh`/Codex CLIを利用する案を提示した。

## GitHubで確認済みの状態（2026-10-21）

- 開発Issue 7件（#2〜#8）を`gh issue list --state all`で確認した。[一覧と各Issueへのリンク](03-issues-plan.md)を参照。
- テスト用Issue #1はクローズ済みで、Projectには追加していない。
- [Project #7「202610-team-b」](https://github.com/orgs/rjc-4g/projects/7)が存在し、対象リポジトリに紐づいている。作成時の名前は「Rustサーバー監視ダッシュボード開発」。
- [Boardビュー](https://github.com/orgs/rjc-4g/projects/7/views/2)と4つのStatusを確認した。登録は7件で、確認時点ではSetupがIn Review、APIがIn Progress、残り5件がTodoだった。
- mainの`75a2763`時点では、最小限のフォルダ構成、README、MIT Licenseが存在する。`Cargo.toml`、実装コード、Docker設定、CIはまだmainにない。作業ブランチでの実装完了を示すものではない。

上記は確認時点の記録であり、日々の進捗はIssue・PR・Projectを正とする。

## 未確定・確認したいこと

1. **開発PCのOS:** Windows（WSL2）、macOS、Linuxのどれか。
2. **作業ブランチの実装状況:** 開始前に担当Issue・PRを確認し、進行中の作業を重複させない。
3. **実測値の扱い:** チャレンジを実施するか、MVP優先で後日に回すか。CPU使用率の基準、取得不能・準備中のAPI表現は着手時に確定する。
4. **運用設定:** ブランチ保護やStatusの自動更新を導入するか。現在の手順は手動更新を前提とする。

## 改善提案

- **実装とCIを優先:** 実測値取得に時間を使いすぎない。
- **Issue番号を推測しない:** 作成時はGitHubの応答・一覧で番号を確認し、登録後は確認済みURLで参照する。
- **1 Issue = 1 PRを基本にする:** 小さい単位でレビューを体験できる。
- **MVPのAPI仕様を固定する:** UIとAPIを並行して実装しやすい。
- **ProjectとIssueの状態を同期:** PR作成でIn Review、マージ後Doneに移動する。
- **GitHub Actionsの失敗体験を残す:** 学習成果として説明しやすい。
- **参考画像はリンクで紹介:** 第三者の記事画像をリポジトリに複製せず、参考元URLと利用方針を記録する。

## 変更履歴

- 2026-10-21: これまでの会話内容から初版作成。実際のGitHub登録状況は未検証。
- 2026-10-21: 参考画像の出典をscmGalaxyの記事として明記。第三者画像とGitHub操作画面の添付を除外。
- 2026-10-21: ドキュメントレビューを実施。配置パス、登録済みIssue・Project、mainの構成を照合し、セットアップの前提と実測値計算の基準を補足した。
