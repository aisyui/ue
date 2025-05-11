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


