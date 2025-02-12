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

### Gameplay Camera

例えば、camera(Gameplay)を有効にするには`$project/Config/DefaultEngine.ini`に`DDCVar.NewGameplayCameraSystem.Enable`の行を追加します。`CBP_SandboxCharacter`にある関数の`SetupCamera`を確認してください。

```sh
[/Script/Engine.DataDrivenConsoleVariableSettings]
+CVarsArray=(Type=CVarBool,Name="DDCVar.NewGameplayCameraSystem.Enable",ToolTip="",DefaultValueFloat=0.000000,DefaultValueInt=0,DefaultValueBool=True)
```

### Collision Trace Channel

Collision Trace Channelを設定するには、`Config/DefaultEngine.ini`を編集する必要があります。以下の手順で行います。GASは`traversable`を追加します。これが追加されていないと動きません。

```sh
[/Script/Engine.CollisionProfile] 
+DefaultChannelResponses=(Channel=ECC_GameTraceChannel1,DefaultResponse=ECR_Ignore,bTraceType=True,bStaticObject=False,Name="Traversable")
```

## 他のmapとの統合

mapにはactorがまとめられているものがあり、それはmapにくっついていません。移動できない場合があります。

この場合は、city sampleのほうを別の場所に動かしたほうがいいでしょう。

