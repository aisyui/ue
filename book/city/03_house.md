# 家を作る

1. [twinmotion](https://www.twinmotion.com/)をinstallして、建造物を作ります。template(テンプレート)を編集しても構いません。
2. できたらexportしてueの$projectを選択します。`$project/${name}_Assets`が作成されます。`datasmith`の置き場所は変更しても構いません。
3. $projectを開いて、pluginの`datasmith`を入れます。そして、`$name (Datasmith シーン)`のファイルをmapにD&Dします。
4. すり抜け問題を解消するには作成された`mesh`を全選択して、右クリックで`アセットアクション -> プロパティマトリクスで選択内容を... -> collision complexity(use complex collision as simple...)`を選択します。
> 私の場合はmeshが`/Content/Twinmotion/room/Geometries/`にあります。

<iframe width="100%" height="415" src="https://www.youtube.com/embed/BsLOlAr-wBY?si=jahPUVD8YMMfefvm&start=152&end=160&mute=1&rel=0&showinfo=0&controls=0" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>
