# エラーを解消する

## buildが遅い場合

もしcity sampleのmapを使わない場合は、buildが遅くなってしまいます。

ueのpackage化が遅い場合、使用するmap以外をbuildしないようにすることで、処理時間を短縮できます。以下の手順で設定を行います。

1. `プロジェクト設定`を開きます。
2. 「パッケージ化」セクションに移動します。
3. 「マップのみをクック」オプションにチェックを入れます3。
4. 「パッケージ化されたビルドに含めるマップのリスト」に、必要なマップ（.umapファイル）を指定します3。

この設定により、指定したmapのみがpackage化され、データサイズが小さくなり、処理時間も短縮されます3。

## ue5.5ではbuildが通らない

`2024-11-18`時点ではcity sampleはue5.5でbuildが通りません。ue5.4では通ります。

`Engine/Plugins/Performance/AutomatedPerfTestingにAutomatedPerfTestConfig.cs`, `AutomatedPerfTestNode.cs`が含まれていないため`${project}/Build/Script/CitySample.Automation.csproj`に記述されているcompileが通らないのです。`AutomatedPerfTesting`は5.5で追加されたpluginです。

```html
<Project Sdk="Microsoft.NET.Sdk">
  <Import Project="CitySample.Automation.csproj.props" Condition="Exists('CitySample.Automation.csproj.props')"/>
  
  <PropertyGroup>
    <TargetFramework>net8.0</TargetFramework>
  </PropertyGroup>

  <ItemGroup>
    <Compile Include="$(EngineDir)\Plugins\Performance\AutomatedPerfTesting\Build\Scripts\AutomatedPerfTestConfig.cs" />
    <Compile Include="$(EngineDir)\Plugins\Performance\AutomatedPerfTesting\Build\Scripts\AutomatedPerfTestNode.cs" />
  </ItemGroup>

</Project>
```

> Experimental release of Automated Perf Testing Plugin v0.1, providing Gauntlet Test Controllers, UAT Test Nodes, and BuildGraph macros for adding common automated performance tests to a project’s automated build and test.

- https://dev.epicgames.com/documentation/ja-jp/unreal-engine/unreal-engine-5.5-release-notes
- https://dev.epicgames.com/documentation/en-us/unreal-engine/API/Plugins/AutomatedPerfTesting

これはgithubにあるsrcから持ってくるしかありません。アクセスするにはorgに参加します。

https://github.com/EpicGames/UnrealEngine/tree/release/Engine/Plugins/Performance/AutomatedPerfTesting/Build/Scripts

## 問題が起こったときにresetする

`/Content/Map/Small_City_LVL.umap`と`/Content/__ExternalActors__/Map/Small_City_LVL`を上書きします。copy元は`VaultCache`からでもいいですし、新しく作った`CitySample`のprojectからでもいいです。

基本的に新しいprojectを作成するときは`VaultCache`からcopyされます。これがないとdownloadから`VaultCache`が生成されます。

## error II-E1001

`Epic Games Launcher`に非常によく出るerrorです。

エラーコード「II-E1001」は、主にEpic Games LauncherまたはUnreal Engineでのプロジェクト作成やコンテンツダウンロード時に発生する問題です。このエラーの原因と解決策を以下にまとめます。`Epic Games Launcher`を再インストールすることでしか治らないこともあります。

1. キャッシュの破損
> Epic Games LauncherのVault Cacheに破損したデータが残っている場合、エラーが発生することがあります7。
2. ダウンロード中のクラッシュ
> コンテンツのダウンロード中にPCがクラッシュした場合、未完了のデータが原因でエラーが発生することがあります7。
3. 外部ストレージの使用
> ライブラリが外部ハードドライブ上にある場合、ランチャーが正しく認識できないことがあります7。

### 解決策

1. Vault Cacheフォルダを確認・削除
> Epic Games Launcherの「設定」からVault Cacheの場所を確認します。
> 対象コンテンツのフォルダを削除します（フォルダが空の場合も削除可能）。
> 他のフォルダを選択して、元のフォルダに戻すと解消されることがあります。
2. Vault Cacheフォルダの属性変更
> Vault Cacheフォルダを右クリックし、「読み取り専用」のチェックを外して適用します。
3. Epic Games Launcherの再起動
> ランチャーを再起動し、問題が解消されるか確認します。
> 必要であればPC自体も再起動してください。
4. 外部ストレージの確認
> ライブラリが外部ストレージ上にある場合、ランチャー設定で適切なキャッシュフォルダが指定されているか確認します7。
5. Epic Games Launcherの再インストール
> キャッシュをクリアした後、Epic Games Launcherを再インストールします。
> これらの手順で解決しない場合は、Epic Gamesサポート（公式ヘルプページ）に問い合わせることをお勧めします。
