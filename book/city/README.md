# city sample

[city sample](https://www.unrealengine.com/marketplace/ja/product/city-sample)

ニューヨーク(NY)をモデルに都市が作られています。人と自動車が動いています。自動車は乗ることや破壊することができます。

## AutomatedPerfTesting

`AutomatedPerfTesting`は5.5で追加されたpluginです。

> Experimental release of Automated Perf Testing Plugin v0.1, providing Gauntlet Test Controllers, UAT Test Nodes, and BuildGraph macros for adding common automated performance tests to a project’s automated build and test.

- https://dev.epicgames.com/documentation/ja-jp/unreal-engine/unreal-engine-5.5-release-notes
- https://dev.epicgames.com/documentation/en-us/unreal-engine/API/Plugins/AutomatedPerfTesting

そのうち解消されると思いますが、現在(2024-11-18)、city sampleはbuildできません。`Engine/Plugins/Performance/AutomatedPerfTestingにAutomatedPerfTestConfig.cs`, `AutomatedPerfTestNode.cs`が含まれていないため`${project}/Build/Script/CitySample.Automation.csproj`に記述されているcompileが通らないのです。

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

これはgithubにあるsrcから持ってくるしかありません。アクセスするにはorgに参加します。

https://github.com/EpicGames/UnrealEngine/tree/release/Engine/Plugins/Performance/AutomatedPerfTesting/Build/Scripts

