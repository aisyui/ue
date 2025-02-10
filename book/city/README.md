# city sample

[city sample](https://www.unrealengine.com/marketplace/ja/product/city-sample)

ニューヨーク(NY)をモデルに都市が作られています。人と自動車が動いています。自動車は乗ることや破壊することができます。

## game animation sampleとの統合

他のassetとの統合を考えるとき、city sampleをベースにします。なぜなら、city sampleは複雑すぎるためです。ここではcity sampleをinstallした上で、game animation sampleをそこにcopyします。

なお、`GameAnimationSample/Binaries/Win64/UnrealEditor.modules`だけはcopyしません。

```sh
# 以下のfileをcopy
GameAnimationSample
    Binaries
        Win64/UnrealEditor.modules #このfileだけはcopy(rewrite)しない
    Build
    Content
```

次に`CitySample/Binaries/Win64/UnrealEditor.modules`を編集します。

```json
{
    "BuildId": "xxx",
        "Modules":
        {
            "CitySample": "UnrealEditor-CitySample.dll",
            "CitySampleAnimGraphRuntime": "UnrealEditor-CitySampleAnimGraphRuntime.dll",
            "CitySampleEditor": "UnrealEditor-CitySampleEditor.dll"
        },
        {
            "GameAnimationSample": "UnrealEditor-GameAnimationSample.dll"
        }
}
```

これでGASが機能すればokです。

他には`$project/Config`と`$project/xxx.uproject`を見比べてみましょう。必要そうなものを追記します。

例えば、camera(Gameplay)を有効にするには`$project/Config/DefaultEngine.ini`に`DDCVar.NewGameplayCameraSystem.Enable`の行を追加します。`CBP_SandboxCharacter`にある関数の`SetupCamera`を確認してください。

```sh
[/Script/Engine.DataDrivenConsoleVariableSettings]
+CVarsArray=(Type=CVarBool,Name="DDCVar.NewGameplayCameraSystem.Enable",ToolTip="",DefaultValueFloat=0.000000,DefaultValueInt=0,DefaultValueBool=True)
```

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

