# animation clipをueで使う

## fbxでexportする

unityのmotionはanimation clipといいます。fbxでexportするには`fbx`, `recorder`を使います。

1. `.vrm`を読み込みます。vrmを読み込むpackage(plugin/addon)を追加してください。
2. Sceneにmodel(object)をD&Dします。
3. animation clipをmodelにD&Dします。すると、Animatorが追加されます。具体的には`Entry -> Animation Clip`になります。再生するとmotionが再生されます。
4. package-managerの`unity registry`でfbxとrecorderを検索し、packageをinstallします。
5. `Window -> General -> Recorder -> Recorder Window -> Add: FBX`を追加します。
6. GameObjectの欄にmodelをD&Dします。そして、Start Recordingを実行します。止めると`.fbx`が保存されます。

## vrmaでexportする

ueでfbxをimportしてもリターゲットが設定されていません。これは非常に面倒です。したがって、`.vrma`でexportして、ue(vrm4u)で読み込む方法があります。

今回は、原神やProject Mugenなどに出てくる通称、アラレちゃん走りを作ってみます。

https://booth.pm/ja/items/2845548

1. unityの[AnimationClipToVrmaSample](https://github.com/malaybaku/AnimationClipToVrmaSample)を使って`.vrma`にexportする。
2. `${model}/RTG_UEFN_${animation_clip}`を開く。この際、`ルートボーンに垂直オフセットを適用`します。
3. そこでグローバル設定にて`ルートを有効化`のチェックを外します。そして、animをexportします。

