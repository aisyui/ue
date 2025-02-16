# issue

[issue](/default/00_issue.html)

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

## cameraが急接近する

よじ登ったり、柵を超えたりするとき、camera(カメラ)が急接近することがあります。

これは`GameplayCamera`の`CameraRig_CollisionOffset`が原因です。`/Content/Blueprints/CBP_SandboxCharacter`の`SetupCamera`という関数で使われていますので、その部分だけ外しておきましょう。

ただし、床が透けて映ってしまうようになります。

## characterのcomponentが初期化される

buildすると動きやカメラがおかしくなっていることに気づくかもしれません。

これはcomponentが初期化されていることが原因です。

通常、`/Content/Blueprints/CBP_SandboxCharacter`にあるcomponentは`/Content/Blueprints/RetargetedCharacters/CBP_SandboxCharacter_${name}`と共通しますが、これが初期化されているのです。例えば、100という値が入れられていたとして、初期設定が1なら、1に戻されるということです。

原因はわかりませんが、これが起こるとcameraがおかしくなったり、`IA_Sprint`でダッシュできなくなったりすることがあります。

これを解決する方法は、`/Content/Blueprints/CBP_SandboxCharacter`の`Event BeginPlay`でcomponentの初期設定を追加することです。

<iframe src="https://blueprintue.com/render/0lt9y0_u/1" scrolling="no" allowfullscreen style="width:100%;height:400px"></iframe>

この方法でも解決しない場合があります。mapを開いて一度再生したあとにbuildするとうまくいくことがあります。

また、それ以外の方法として、初期設定を全部`CBP_SandboxCharacter_${name}`の`Construction Script`ほうに書き出すことが考えられます。

<iframe src="https://blueprintue.com/render/iikbmf9a/1" scrolling="no" allowfullscreen style="width:100%;height:400px"></iframe>

1. mapを開いて、問題の動作を実行したあとにbuildする
2. `CBP_SandboxCharacter`の`Event BeginPlay`に書いてみる
3. `CBP_SandboxCharacter_${name}`の`Construction Script`に書いてみる

## IA_Sprintのダッシュができなくなる

ダッシュは`/Content/Blueprints/RetargetedCharacters/CBP_SandboxCharacter_${name}`の`Event BeginPlay`を削除することで解消しました。

nodeをつなげていなくてもevent自体を削除しなければなりません。

## traversableが機能しなくなる

突然、traversable(トラバーサブル)、つまり、よじ登ったりする機能が使えなくなることがあります。

いくつか原因が考えられますが、project設定の`collision > trace channel : Traversable`に問題があるのかもしれません。

collision trace channelに問題が発生するときはけっこう大変です。色々とbugがあり、一度削除して同じ名前で作り直せば動作することもありますが、余計に壊れることもあります。例えば、削除したときにBPのnodeに他のtrace channelが自動で入ってしまうことも要因になります。

