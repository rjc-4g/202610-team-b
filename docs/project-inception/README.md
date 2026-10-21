# Rustサーバー監視ダッシュボード — プロジェクトドキュメント

> 作成日: 2026-10-21 / 対象リポジトリ: [`rjc-4g/202610-team-b`](https://github.com/rjc-4g/202610-team-b)
>
> 本資料は企画・実装方針・GitHub運用の検討記録です。2026-10-21時点で開発Issue 7件とProjectの作成を確認済みです。**コード例は実装予定のガイドであり、実装完了を示すものではありません。** 最新の進捗はGitHubで確認してください。

## 登録済みの管理先

- [開発Issue一覧](https://github.com/rjc-4g/202610-team-b/issues)（各Issueへのリンクは[Issue計画](03-issues-plan.md)を参照）
- [Project #7「202610-team-b」のBoard](https://github.com/orgs/rjc-4g/projects/7/views/2)（作成時の名前は「Rustサーバー監視ダッシュボード開発」）
- 確認時点の登録状況は[決定事項・未確定事項](06-decisions-and-open-questions.md)を参照。

## ドキュメント一覧

| ファイル | 内容 |
|---|---|
| [01-overview.md](01-overview.md) | 目的、スコープ、構成図、完了条件 |
| [02-implementation.md](02-implementation.md) | Rust/Axum、UI、Docker、cgroup v2の実装ガイド |
| [03-issues-plan.md](03-issues-plan.md) | 7件のIssueと15人時の見積・受け入れ条件 |
| [04-github-workflow.md](04-github-workflow.md) | GitHub Flow、PR、CI/CD、Codex CLI、Projects |
| [05-setup-and-operations.md](05-setup-and-operations.md) | 環境構築、コマンド、起動、トラブルシューティング |
| [06-decisions-and-open-questions.md](06-decisions-and-open-questions.md) | 決定事項、保留事項、今後の改善案 |
| [07-design-references.md](07-design-references.md) | UI検討時の参考サイトと画像利用方針 |

## 最短の進め方

1. [全体方針](01-overview.md)を確認する。
2. [Issue計画](03-issues-plan.md)から登録済みのIssueを確認する。重複作成しない。
3. [GitHub運用](04-github-workflow.md)に従い、Issueごとにfeatureブランチ→PR→レビュー→CI→マージを行う。
4. 最初は**固定値の監視API + ダッシュボード**を完成させる。
5. 余裕があればcgroup v2による実測値取得に挑戦する。

## UIデザインの参考資料

監視ダッシュボードの画面イメージを検討する際は、scmGalaxyの記事に掲載されたPrometheus/Grafanaのダッシュボード画像を**参考資料として閲覧**した。

- [Complete Guide of Prometheus with Grafana — scmGalaxy](https://www.scmgalaxy.com/tutorials/complete-guide-of-prometheus-with-grafana-certification-courses-tutorials-and-training/)
- 詳細は[UIデザインの参考資料と画像利用方針](07-design-references.md)を参照。

**記事中の画像は本リポジトリに複製・同梱していない。** 本プロジェクトの完成イメージ画像も新規作成しない。
