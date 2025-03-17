## playerが地面に埋まってしまう

GASPと統合するとcity sampleに置かれた物体に触れられません。collisionが発生しないのです。

原因は`Gameplay Camera`というpluginです。`$project/Config/DefaultEngine.ini`に`DDCVar.NewGameplayCameraSystem.Enable`を追加し、関数である`Setup Camera`を実行している場合、characterのcollisionが機能せず地面に埋まってしまう問題があります。

- Gameplay Camera
- Setup Camera
- `DDCVar.NewGameplayCameraSystem.Enable`

```sh
[/Script/Engine.DataDrivenConsoleVariableSettings]
+CVarsArray=(Type=CVarBool,Name="DDCVar.NewGameplayCameraSystem.Enable",ToolTip="",DefaultValueFloat=0.000000,DefaultValueInt=0,DefaultValueBool=True)
```

`Setup Camera`を実行しないようにするか、`DDCVar.NewGameplayCameraSystem.Enable`をfalseにします。

### Collision Trace Channel

`Collision Trace Channel`を設定するには、`Config/DefaultEngine.ini`を編集する必要があります。以下の手順で行います。GASPは`traversable`を追加します。これが追加されていないと動きません。

`ECC_GameTraceChannel${n}`に注意してください。

```sh
[/Script/Engine.CollisionProfile] 
+DefaultChannelResponses=(Channel=ECC_GameTraceChannel11,DefaultResponse=ECR_Ignore,bTraceType=True,bStaticObject=False,Name="Traversable")
```


