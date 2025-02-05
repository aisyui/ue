## ueとは

`Unreal Engine`は`epic games`という会社が作っているゲームエンジンです。よく`ue`と略されます。

最新バージョン(latest version)は`5`なので、`ue5`となります。

`ue`の他には`unity`, `godto`が有名です。`DirectX`というものがあり、例えば、`DX12`からGPUに命令を送ることができます。ゲームエンジンの多くは`DX11`, `DX12`を使用します。

|name|url|
|---|---|
|unreal engine|https://unrealengine.com/|
|github|https://github.com/EpicGames/UnrealEngine|

ソース(source)からビルド(build)して使うこともできます。一部でbuildしないと使えない機能があります。private repoなので[こちら](https://www.unrealengine.com/ja/ue-on-github)からアクセス権をもらいます。

## ueの特徴と注意

`c++`か`blueprint`で書きます。

ueの特徴はグラフィックが綺麗なことです。と言ってもきれいなグラフィックはunityなどでも実現可能です。しかし、ueは初期設定でもそれが実現できるので、主にグラフィック用途で使用されています。

ueはバグ(bug)が多く基本的に動きません。これはアップデート(update)が速く、3Dを扱うソフトウェア(software)なので仕方ありません。複雑なのです。

ueを長く使っているとわかることですが、ゲーム制作にはあまり向きません。unityのほうがおすすめです。ueはきれいな画像やシーンを作る用途におすすめです。

「ueはバグが多く基本的に壊れている」このことを最初に理解しておかないと「おかしい、動かない」と時間を無駄にしてしまいます。また、保存せず長時間コードを書き続けるのは危険です。クラッシュ(crash)すると消えてしまいます。プロジェクト(project)そのものが壊れることもよくあります。注意しましょう。

## 用語の解説

|title|short|body|
|---|---|---|
|unrealengine|ue|アンリアルエンジン|
|version|ver|バージョン|
|code||コード、プログラミング言語で書かれた文章|
|build||ビルド、osで実行できる形式にすること。windowsなら`.exe`|
|compile||コンパイル、コンピュータで実行できる形式にすること。buildと同じ意味で使われる|
|source|src|ソース、主にソースコードの略|
|server||サーバー、リクエストに応じるコンピュータ|
|deploy||serverに実行ファイルを置くこと|
|example|ex|例、uriではexample.comが有名|
|install||インストール、アプリをインストールすること|
|application|app|アプリ、ソフトウェア(software)のこと|
|library|lib|ライブラリ、softwareを構築するための部品|
|package|pkg|パッケージ、appだったり、libだったり色々。基本的にpkg managerでinstallできるものを指す|
|update|up|アップデート、5.5の`x.5`の部分を言う ex: 5.4 -> 5.5|
|upgrade||アップグレード、5.5の`5.x`の部分を言う ex: 4.0 -> 5.0|
|asset||アセット、ueでは購入できるlibやexampleを指す|
|plugin|plug|プラグイン、エンジンに直接入れる追加機能。新たなblueprintなどを使えるようになる|
|crash||クラッシュ、アプリやosが落ちること|
|cache||キャッシュ、一時ファイルのこと。tmpなども使われる|
|repository|repo|リポジトリ、主にsrc codeの一式が置いてある場所|
|project||プロジェクト、ueでは主に新しいゲームを作った時のフォルダ一式|
|blueprint|bp|ブループリント、ueのノードベースで書く形式。anim blueprintはabp、character blueprintはcbpと略される事が多く、ファイル名は`BP_XXX`, `ABP_XXX`, `CBP_XXX`となる。基本大文字が使われる|

基本的に英語で書けるものは英語で書きます。ueのmenuは英語にするのがおすすめです。様々な単語に慣れておきましょう。
