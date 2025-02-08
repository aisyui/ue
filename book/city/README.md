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

これでGASPが機能すればokです。

他には`$project/Config`と`$project/xxx.uproject`を見比べてみましょう。必要そうなものを追記します。

```json
{
	"FileVersion": 3,
	"EngineAssociation": "5.5",
	"Category": "Samples",
	"Description": "",
	"Modules": [
		{
			"Name": "CitySample",
			"Type": "Runtime",
			"LoadingPhase": "Default",
			"AdditionalDependencies": [
				"Engine",
				"AIModule",
				"ChaosVehicles",
				"UMG",
				"MovieScene"
			]
		},
		{
			"Name": "CitySampleEditor",
			"Type": "Editor",
			"LoadingPhase": "Default",
			"AdditionalDependencies": [
				"Engine"
			]
		},
		{
			"Name": "CitySampleAnimGraphRuntime",
			"Type": "UncookedOnly",
			"LoadingPhase": "Default"
		}
	],
	"Plugins": [
		{
			"Name": "AlembicHairImporter",
			"Enabled": true
		},
		{
			"Name": "HairStrands",
			"Enabled": true
		},
		{
			"Name": "PythonScriptPlugin",
			"Enabled": true
		},
		{
			"Name": "ControlRig",
			"Enabled": true
		},
		{
			"Name": "Takes",
			"Enabled": true
		},
		{
			"Name": "D3DExternalGPUStatistics",
			"Enabled": true,
			"Optional": true,
			"SupportedTargetPlatforms": [
				"Win64"
			]
		},
		{
			"Name": "LiveLinkCurveDebugUI",
			"Enabled": true
		},
		{
			"Name": "ChaosVehiclesPlugin",
			"Enabled": true
		},
		{
			"Name": "RigLogic",
			"Enabled": true
		},
		{
			"Name": "RawInput",
			"Enabled": true
		},
		{
			"Name": "GameplayInsights",
			"Enabled": true
		},
		{
			"Name": "TraceSourceFilters",
			"Enabled": true
		},
		{
			"Name": "TraceDataFilters",
			"Enabled": true
		},
		{
			"Name": "ModelingToolsEditorMode",
			"Enabled": true
		},
		{
			"Name": "Traffic",
			"Enabled": true
		},
		{
			"Name": "CitySampleMassCrowd",
			"Enabled": true
		},
		{
			"Name": "StateTree",
			"Enabled": true
		},
		{
			"Name": "OnlineSubsystem",
			"Enabled": true
		},
		{
			"Name": "OnlineSubsystemUtils",
			"Enabled": true
		},
		{
			"Name": "MassAI",
			"Enabled": true
		},
		{
			"Name": "MassCrowd",
			"Enabled": true
		},
		{
			"Name": "MassEntity",
			"Enabled": true
		},
		{
			"Name": "MassGameplay",
			"Enabled": true
		},
		{
			"Name": "HoverDrone",
			"Enabled": true
		},
		{
			"Name": "EnhancedInput",
			"Enabled": true
		},
		{
			"Name": "Volumetrics",
			"Enabled": true
		},
		{
			"Name": "ContextualAnimation",
			"Enabled": true
		},
		{
			"Name": "ChaosCaching",
			"Enabled": true
		},
		{
			"Name": "FieldSystemPlugin",
			"Enabled": true
		},
		{
			"Name": "Paper2D",
			"Enabled": true
		},
		{
			"Name": "WinDualShock",
			"Enabled": true,
			"SupportedTargetPlatforms": [
				"Win64"
			]
		},
		{
			"Name": "ZoneGraph",
			"Enabled": true
		},
		{
			"Name": "AnimationWarping",
			"Enabled": true
		},
		{
			"Name": "ElectraPlayer",
			"Enabled": true
		},
		{
			"Name": "LiveLinkCamera",
			"Enabled": true
		},
		{
			"Name": "AudioModulation",
			"Enabled": true
		},
		{
			"Name": "Metasound",
			"Enabled": true
		},
		{
			"Name": "MovieRenderPipeline",
			"Enabled": true
		},
		{
			"Name": "Soundscape",
			"Enabled": true
		},
		{
			"Name": "FullBodyIK",
			"Enabled": true
		},
		{
			"Name": "RemoteControl",
			"Enabled": true
		},
		{
			"Name": "ImagePlate",
			"Enabled": true
		},
		{
			"Name": "LightWeightInstancesEditor",
			"Enabled": true
		},
		{
			"Name": "GameFeatures",
			"Enabled": true
		},
		{
			"Name": "ModularGameplay",
			"Enabled": true
		},
		{
			"Name": "CitySampleSensorGrid",
			"Enabled": true
		},
		{
			"Name": "IKRig",
			"Enabled": true
		},
		{
			"Name": "ColorCorrectRegions",
			"Enabled": true
		},
		{
			"Name": "Gauntlet",
			"Enabled": true
		},
		{
			"Name": "CustomizableSequencerTracks",
			"Enabled": true
		},
		{
			"Name": "NetworkPrediction",
			"Enabled": true
		},
		{
			"Name": "NiagaraFluids",
			"Enabled": true
		},
		{
			"Name": "Text3D",
			"Enabled": true
		},
		{
			"Name": "MotoSynth",
			"Enabled": true
		},
		{
			"Name": "EditorDataStorage",
			"Enabled": true
		},
		{
			"Name": "EditorDataStorageFeatures",
			"Enabled": true,
			"TargetAllowList": [
				"Editor"
			]
		},
		{
			"Name": "AnimToTexture",
			"Enabled": true
		},
		{
			"Name": "SequencerScripting",
			"Enabled": true,
			"TargetAllowList": [
				"Editor"
			]
		},
		{
			"Name": "GeometryCollectionPlugin",
			"Enabled": true,
			"TargetAllowList": [
				"Editor"
			]
		},
		{
			"Name": "AutomatedPerfTesting",
			"Enabled": true
		},
		{
			"Name": "LiveLink",
			"Enabled": true
		},
		{
			"Name": "LiveLinkControlRig",
			"Enabled": true
		},
		{
			"Name": "PoseSearch",
			"Enabled": true
		},
		{
			"Name": "AnimationLocomotionLibrary",
			"Enabled": true
		},
		{
			"Name": "MotionWarping",
			"Enabled": true
		},
		{
			"Name": "Chooser",
			"Enabled": true
		},
		{
			"Name": "Mover",
			"Enabled": true
		}
	],
	"TargetPlatforms": [
		"PS5",
		"XSX",
		"Windows"
	],
	"EpicSampleNameHash": "111"
}
```

また、`/Config/DefaultEngine.ini`に注意してください。

```sh
[/Script/Engine.CollisionProfile]
+DefaultChannelResponses=(Channel=ECC_GameTraceChannel11,DefaultResponse=ECR_Ignore,bTraceType=True,bStaticObject=False,Name="Traversable")
+DefaultChannelResponses=(Channel=ECC_GameTraceChannel12,DefaultResponse=ECR_Ignore,bTraceType=True,bStaticObject=False,Name="EnemyProjectile")
+DefaultChannelResponses=(Channel=ECC_GameTraceChannel13,DefaultResponse=ECR_Block,bTraceType=True,bStaticObject=False,Name="BulletHell")
+DefaultChannelResponses=(Channel=ECC_GameTraceChannel14,DefaultResponse=ECR_Ignore,bTraceType=True,bStaticObject=False,Name="LookAtTrace")
```

`Collision Trace Channel`は作り直さなければ機能しないことがあります。

### Gameplay Camera

例えば、camera(Gameplay)を有効にするには`$project/Config/DefaultEngine.ini`に`DDCVar.NewGameplayCameraSystem.Enable`の行を追加します。`CBP_SandboxCharacter`にある関数の`SetupCamera`を確認してください。

```sh
[/Script/Engine.DataDrivenConsoleVariableSettings]
+CVarsArray=(Type=CVarBool,Name="DDCVar.NewGameplayCameraSystem.Enable",ToolTip="",DefaultValueFloat=0.000000,DefaultValueInt=0,DefaultValueBool=True)
```

現在、[characterのcollisionが機能しない問題](/city/00_err.html)が発生します。

### Collision Trace Channel

`Collision Trace Channel`を設定するには、`Config/DefaultEngine.ini`を編集する必要があります。以下の手順で行います。GASPは`traversable`を追加します。これが追加されていないと動きません。

`ECC_GameTraceChannel${n}`に注意してください。

```sh
[/Script/Engine.CollisionProfile] 
+DefaultChannelResponses=(Channel=ECC_GameTraceChannel11,DefaultResponse=ECR_Ignore,bTraceType=True,bStaticObject=False,Name="Traversable")
```

## 他のmapとの統合

mapにはactorがまとめられているものがあり、それはmapにくっついていません。移動できない場合があります。

基本的にはA-mapとB-mapはある程度位置を完成させてからcopyしましょう。

