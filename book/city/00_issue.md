# issue

[issue](/default/00_issue.html)

## buildが遅い場合

もしcity sampleのmapを使わない場合は、buildが遅くなってしまいます。

ueのpackage化が遅い場合、使用するmap以外をbuildしないようにすることで、処理時間を短縮できます。以下の手順で設定を行います。

1. `プロジェクト設定`を開きます。
2. 「パッケージ化」セクションに移動します。
3. 「マップのみをクック」オプションにチェックを入れます。
4. 「パッケージ化されたビルドに含めるマップのリスト」に、必要なマップ（.umapファイル）を指定します。

この設定により、指定したmapのみがpackage化され、データサイズが小さくなり、処理時間も短縮されます。

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

1. `$(EngineDir)\Plugins\Performance\AutomatedPerfTesting\Build\Scripts`フォルダを作ります。`$(EngineDir)`はueがインストールされているディレクトリです。
2. githubの[src](https://github.com/EpicGames/UnrealEngine/tree/release/Engine/Plugins/Performance/AutomatedPerfTesting/Build/Scripts)から持ってきた`$(EngineDir)\Plugins\Performance\AutomatedPerfTesting\Build\Scripts\AutomatedPerfTestConfig.cs`, `$(EngineDir)\Plugins\Performance\AutomatedPerfTesting\Build\Scripts\AutomatedPerfTestNode.cs`を同ディレクトリに置きます。

また、`CitySampleCookedEditor.Target.cs`を削除するとbuildが通ったという報告があります。

https://forums.unrealengine.com/t/automatedperftesting-plugin-does-not-build-when-building-city-sample-in-ue5-5/2134722/9

## 問題が起こったときにresetする

`/Content/Map/Small_City_LVL.umap`と`/Content/__ExternalActors__/Map/Small_City_LVL`を上書きします。copy元は`VaultCache`からでもいいですし、新しく作った`CitySample`のprojectからでもいいです。

基本的に新しいprojectを作成するときは`VaultCache`からcopyされます。これがないとdownloadから`VaultCache`が生成されます。

## characterのcollisionが機能せず地面に埋まってしまう

GASPと統合するとcity sampleに置かれた物体に触れられません。

原因は`Gameplay Camera`というpluginです。`$project/Config/DefaultEngine.ini`に`DDCVar.NewGameplayCameraSystem.Enable`を追加し、関数である`Setup Camera`を実行している場合、characterのcollisionが機能せず地面に埋まってしまう問題があります。

- Gameplay Camera
- Setup Camera
- `DDCVar.NewGameplayCameraSystem.Enable`

```sh
[/Script/Engine.DataDrivenConsoleVariableSettings]
+CVarsArray=(Type=CVarBool,Name="DDCVar.NewGameplayCameraSystem.Enable",ToolTip="",DefaultValueFloat=0.000000,DefaultValueInt=0,DefaultValueBool=True)
```

`Setup Camera`を実行しないようにするか、`DDCVar.NewGameplayCameraSystem.Enable`をfalseにします。

