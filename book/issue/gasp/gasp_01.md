## GameplayCameraをdisableにする

GASPで使用されるGameplayCamera(plugin)が壊れているため、これを削除する人が多いです。

削除方法は簡単で`/Content/Blueprints/CBP_SandboxCharacter`の`GameplayCamera`というcomponentを削除し、err箇所のnodeを修正すると完了です。

仕組みを解説すると、このcomponentでCameraに`/Blueprints/Cameras/CameraAsset_SandboxCharacter`を指定しています。そして、`SetupCamera`という関数でenable(有効)にします。関数では`$project/Config/DefaultEngine.ini`にある`DDCVar.NewGameplayCameraSystem.Enable`の値がtrueなら使用されます。

```sh
[/Script/Engine.DataDrivenConsoleVariableSettings]
+CVarsArray=(Type=CVarBool,Name="DDCVar.NewGameplayCameraSystem.Enable",ToolTip="",DefaultValueFloat=0.000000,DefaultValueInt=0,DefaultValueBool=True)
```

`DDCVar.NewGameplayCameraSystem.Enable`をfalseにするか、あるいは`branch`の処理を削除し、false後の処理(Set View Target with Blend)につなげるかで機能をdisable(無効)にすることができます。

私はなるべく最新機能を使用したいと思っています。ですが、先に述べたように色々と壊れているため、問題がたくさん発生します。原因が`GameplayCamera`にあることを突き止めるのも苦労しますから、disableにしておくのが正解かもしれません。


