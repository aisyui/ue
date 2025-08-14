# ai.verse UE5実装ガイド

## プロジェクト概要
**ai.verse** - Unreal Engine 5.6で開発中の惑星規模3Dメタバース

### 基本情報
- **エンジン**: Unreal Engine 5.6
- **開発手法**: Blueprint中心、繰り返し再実装による改善
- **特徴**: 惑星規模のシームレスワールド、atproto統合

## 現在の実装状況

### 完成済み機能
- ✅ 惑星規模の地球・月・太陽（WorldScape + UDS）
- ✅ シームレスな宇宙移動（大気圏→宇宙が一つのマップ）
- ✅ 月面着陸・クレーター探索
- ✅ キャラクターアクション（GASP + SFA統合）
- ✅ 飛行システム

### 再実装予定機能（./book/に詳細記録）

#### atproto統合
- **ログイン機能** (`/json/02_atproto.md`)
  - syui.ai (bsky.social) / syui.syu.is認証
  - PDSLSでの情報確認
- **データ連携**
  - アイテム取得→レコード更新
  - カードシステム（画像URL管理）
  - ゲームデータ管理（ai.syui.game.system）

#### UI/UX
- **タイトル画面** （実装ドキュメント未発見）
- **ゲームUI** (`/ui/01_list.md`, `/crs/02_boss.md`)
  - ListView実装（WBP_player, WBP_text）
  - HPゲージ（ProgressBar 0.0-1.0）
  - ステータス表示

#### 戦闘システム
- **ボスバトル** (`/crs/02_boss.md`)
  - Control Rig Sample Mech使用
  - Look At bone modifier
  - ダメージ処理（Event Any Damage）
  - Niagara破壊エフェクト
- **武器システム** (`/gasp/12_weapon.md`)
  - Socket設定（武器装着）
  - 抜刀/納刀アニメーション
  - BP_WeaponSword実装
- **剣技システム** ([Blueprint実装](https://blueprintue.com/blueprint/cu104wg0/))
  - Montage切り替えによる3段攻撃
  - 複雑なBranch処理で攻撃段階管理
  - Trail effectsのNiagara実装
- **キャラスキル** ([Niagara Collision実装](https://blueprintue.com/blueprint/h2ktbf6-/))
  - Niagara CollisionでAoEダメージ
  - クールダウンシステム
  - レベル連動の変身システム（Lv1で飛行可能）
  - スキル習得アイテムドロップ

#### 移動・アクション
- **基本移動** (`/gasp/README.md`)
  - WASD移動、Space跳躍、Shift疾走
- **ジャンプシステム** (`/gasp/02_jump.md`)
  - TraversalAction高度変更（275→475）
- **段階的移動** (`/gasp/11_run.md`)
  - スプリントアニメーション切替
  - Layered blend per bone実装

#### アイテムシステム
- **ドロップシステム** (`/plan/04_item.md`, `/crs/02_boss.md`)
  - Nice Interaction System使用
  - AC_PC_Interaction / AC_GS_Interaction
  - "E"キーでのアイテム取得

## 技術スタック

### プラグイン
- **WorldScape**: 惑星地形生成
- **Ultra Dynamic Sky (UDS)**: 天候・時間システム
- **GASP**: Game Animation Sample
- **SFA**: SuperHero Flight Animation
- **VRM4U**: VRMアバター統合（予定）

### 現在直面している課題

#### 1. 重力システム
- **問題**: 惑星の横から入ると坂になり、反対側から進入不可
- **原因**: 球体重力の実装が不完全

#### 2. 天候システム（UDS）
- **Manual Rain**: 動作するが雲の上でも降り続ける
- **Random Rain**: 惑星規模では機能しない
- **惑星裏側**: Weather有効時にレンダリング異常
- **回避策**: Weather無効化で解決するが、晴れ/曇り表現が失われる

##### 解決策：CharacterBPベースの雨エフェクト
- **実装方法**: CharacterBPにSceneRain追加
- **制御**: 高度とUDS天候状態で有効/無効切替
- **利点**: UDSの雲表現を維持しつつ適切な雨表現
- **実装済み**: [雨エフェクトBlueprint](https://blueprintue.com/blueprint/8kty-cvm/)
  - SceneRain子要素をForEachで制御
  - Niagara/PostProcessを個別判定
  - 機能するが実装に改善余地あり

#### 3. ビジュアルスタイル
- **課題**: WorldScapeのデフォルト地形が現実的すぎる
- **目標**: 原神のようなアニメ調スタイル
- **検討**: So Stylized Environmentとの統合

## atproto統合アーキテクチャ

### 認証方式
**WebAPI直接連携**を採用（PixelStreamingに依存しない）

### データ構造
```
GameInstance (BP_Config)
├── 環境変数
│   ├── API設定
│   ├── 認証情報（Token等）
│   └── システムデータ（JSON）
│
├── 管理者アカウント
│   └── ai.syui.game.system
│       ├── キャラクターステータス
│       ├── アイテム効果
│       └── カード情報（ID→画像URL）
│
└── プレイヤーデータ
    └── 個人レコード（atproto collection）
```

### 実装フロー
1. GameInstance起動時にシステムデータ一括取得
2. Castによる都度参照・更新
3. プレイヤーアクション→atprotoレコード更新

## Blueprint実装の課題と解決策

### JSON処理
**課題**: UE5標準のJSON処理が複雑
**解決策**: 
- VaRest Plugin（無料）を独自ビルドで使用
  - 公式はUE5.5/5.6非対応のため自前でビルド
  - メンテナンスされていないが動作は安定
- JSON Parser Plugin（有料）も併用
  - UE5.6サポートは不明（更新が遅れる可能性）

**今後の課題**:
- プラグイン依存のリスク（UEバージョンアップ時）
- C++でのユーティリティクラス実装も検討必要

### Widget画像表示
**実装**: Widget Browserで直接URL表示（キャッシュなし）

### 非同期処理
**実装**: GameInstanceでの一元管理とCast利用

## ゲームデザイン哲学

### 開発方針
- **作り直しの効果**: 1週間の実装→数時間で改善版完成
- **積極的な再実装**: 動くコードがあっても、より良い実装のために作り直す
- **学習重視**: UEへの理解を深めながら、段階的に改善
- **技術的負債回避**: ゴチャゴチャしたコードは捨てる勇気
- **シンプル重視**: 複雑なゲームより長く続く体験
- **配信親和性**: 同じ画面でも個性が出る設計

### 作り直しのメリット
- 新バージョンの機能を最大限活用
- 以前の問題への解決策が見つかる
- コードがスマートでシンプルになる
- UEへの理解が深まるたびに品質向上

### 新機能案
- **宇宙カジノ**: 移動中の暇つぶし要素
- **釣りシステム**: レアドロップ（ドラゴン0.0001%）
- **30秒投稿**: atproto自動投稿との連携

## UE5.6デモゲーム統合計画

### 発見されたUE5.6デモゲーム
1. **Variant_Combat**: 格闘ゲーム、物理演算での吹っ飛び
2. **Variant_Shooter**: 銃ゲーム、弾痕が残る、アイテム拾得
3. **Variant_TwinStick**: 大爆発演出、スロー効果

### ai.verseへの活用方針
- **Variant_Combat → ボスシステム**: 物理演算による自然な反応
- **Variant_Shooter → アイテムシステム**: 武器収集、環境破壊
- **Variant_TwinStick → スキル演出**: 大技の派手な演出

### City Sample統合の可能性
- **自動で動く街**: Mass Entity Systemによる生きた都市
- **車両物理演算**: Chaos Physicsによるリアルな挙動
- **WorldScapeとの組み合わせ**: 惑星表面の都市環境

### 飛行システムの改良案
- **泳ぐような操作感**: 常時微細な揺れ、左右上下の自然な動き
- **加速システム**: 溜め動作による推進力、羽ばたき演出
- **物理ベース**: Air Resistance、慣性重視の操作感

### 2Dステージ統合
- **切り替えシステム**: 特定エリアで2D視点に変更
- **横スクロール**: UE5.6の2Dデモベース
- **3D世界復帰**: クリア後のポータルシステム

## 今後の実装予定

### 短期目標
1. UE5.6での基本システム再構築
2. atproto認証の再実装
3. タイトル画面・ログインUI

### 中期目標
1. OAuth認証への移行
2. カード連携システム
3. PixelStreaming統合（Web配信）

### 長期目標
1. VTuber配信プラットフォーム化
2. Play-to-Work経済システム
3. ai.gpt NPCの統合

## 技術メモ

### UE5.6変更点
- WorldScapeがEarthサイズ対応
- GASPのBP構造変更
- SFAのアニメーション改善

### パフォーマンス最適化
- 惑星規模でのUDS維持設定
- シームレス移動の最適化

### 参考リソース
- [古い実装ドキュメント](https://syui.github.io/m/post/ue/)
- [Blueprint実装記録](https://blueprintue.com/profile/ai/)
- [剣技Blueprint](https://blueprintue.com/blueprint/cu104wg0/)
- [スキルBlueprint](https://blueprintue.com/blueprint/h2ktbf6-/)
- [WorldScape公式ドキュメント](https://iolacorp-1.gitbook.io/worldscape-plugin)
- [So Stylized Environment](https://docs.google.com/document/d/147wCDvZg6-9jZNyqSxX-I_HQkE2tGINZIhyjc2QHirY)

---

*このドキュメントは開発の進捗に応じて随時更新されます*