# tips

この章は読み飛ばしてください。まとめやすいように重要項目を上にしています。

最初はわからないかもしれませんが、最後まで読んでまた来ると、わかるようになっているかもしれません。

## variable(var)

まずは変数の紹介です。変数はローカル変数(local)、グローバル変数(global)があります。最初はprojectで読み込むところに作ります。

1. フォルダの`/Content/Blueprints/`で右クリックして、ブループリントクラス、`GameInstance`を作ります。例えば、`GM_xxx`にします。(名前はなんでもいいです)
2. `設定 -> プロジェクト設定 -> マップ&モード`の`Game Instance:GameInstance Class`に`GM_xxx`を読み込みます。

なお、名前に`Default`, `Test`, `Config`などを使う場合は予約されていることがあります。注意してください。

これを開いて、変数のところで`UserTest`という名前で`string`型の変数を作ります。コンパイル(compile)してください。デフォルト値は`World`とでも入れておきましょう。

次に、`/Content/Blueprints/CBP_SandboxCharacter`を開いて、EventGraphで右クリックし、`Cast To GM_xxx`を選択します。

インプット(input)のpin(ピン)にあるObjectには`Get Game Instance`をつなぎます。そして、アウトプット(output)の青線は`Get UserTest`と検索し、それを`Print String`につなぎます。

さて、ここにkeyを設定して再生すると`World`と表示されるはずです。

特に重要なのが`変数の型`です。種類もそうですが、変数の型の右側にある色アイコンをクリックしてみると、たくさんの形式があることがわかります。

- 単一
- 配列
- 設定
- マップ

配列を使ってみましょう。`UserTestList`という名前で`string`の配列を作成します。

ここで、デフォルト値に追加できることに気づくはずです。

例えば、単語を3つ追加してみます。

- [0]ai
- [1]burst
- [2]can

`Cast To GM_xxx`から`Get UserTestList`を引っ張り出し、`Length`につなげます。それをprintしてみると、`2`と表示されるはずです。

`length(len)`はその配列に追加された数を知ることができます。

では、全部の値を取り出すため`for`を書きましょう。`UserTestList`から`For Each Loop`を伸ばして`Loop Body`からprintにつなぎます。値は`Array Element`から取ります。

もし文字列をつなぎたいなら`Append`が使えます。

変数にはObjectをいれることもできます。たくさんのObjectを入れておき、`Find`で検索して取り出すこともできます。

では、characterを全部まとめて検索できる変数を作ってみます。

名前は`UserCharacterObj`でstringを選択し、マップを選択します。右側のマップは`オブジェクト(object)`と検索し、object参照を選択します。

```sh
str obj
--- ---
```

`cbp character`と検索し、characterのcbpを入れていきます。名前はわかりやすいものにしてください。例えば、`manny`, `quinn`など。

では繋いでいきます。

```sh
[Get Game Instance] --> [Cast To GM_Defaultconfig] --> [UserCharacterObj]

--> [Find(manny)] --> [Get Object Name] --> [Print String]
```

<iframe src="https://blueprintue.com/render/_q-q_ffz/" scrolling="no" allowfullscreen style="width:100%;height:400px"></iframe>

基本的にこれを利用して、他のBPと値をやり取りすることになります。例えば、`ABP_SandboxCharacter`で条件を満たしたとき実行してほしいアニメーション(anim)があったとします。

この場合、まず`GM_xxx`の変数で`UserCharacterAnim`をboolean型で作り、実行してほしいタイミングで`CBP_SandboxCharacter`に書いた`Cast To GM_Defaultconfig`から`UserCharacterAnim`を`true`に変更します。`Set UserCharacterAnim`で検索してください。そして、ABPには`UserCharacterAnim`がtrueならanimを再生する処理を書きます。

## function(func)

次は関数です。関数は簡単で、何度も繰り返す部分を再利用可能な形で残します。それが関数です。

新しい関数を作ってみましょう。左バーの関数(+)を押せば作成されます。名前は`FuncTest`にします。

関数を表す紫色のボックス(box)を選択した状態でinputとoutputの見てもらって、inputにboolという名前でboolean型を作ります。outputにはstrという名前でstring型を作ります。

処理の内容はboolがtrueなら`[yes]`というstringを出力し、falseなら`[no]`にします。

<iframe src="https://blueprintue.com/render/pvn4lao9/2" scrolling="no" allowfullscreen style="width:100%;height:400px"></iframe>

確認してみましょう。`[9]`を押すと`[yes][no]`が表示されます。

<iframe src="https://blueprintue.com/render/pvn4lao9/1" scrolling="no" allowfullscreen style="width:100%;height:400px"></iframe>

