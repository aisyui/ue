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


