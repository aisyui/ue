# キャラクターを作る

ueで読み込むには`vrm4u`というpluginを使用します。

<iframe width="100%" height="415" src="https://www.youtube.com/embed/0Ig_-JSRV0M?si=Kz_jCbYTHr_OzPpP&start=0&end=23&mute=1&rel=0&showinfo=0&controls=0" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>

1. [vrm4u](https://github.com/ruyo/VRM4U/releases)をダウンロード(download)して、`$project/Plugins`に入れる。
> $project/Plugins/VRM4U/VRM4U.uplugin
2. editorのファイルエクスプローラーでモデルファイルの`.vrm`をD&Dします。色々と聞かれますが適当にokや選択します。配色タイプが聞かれます。例えば、`$project/Content/Characters`に`model`フォルダを作り`model.vrm`をimport(インポート)した場合で解説します。
> $project/Content/Characters/$model

モデル(model)は[こちら](https://hub.vroid.com/users/36144806)からdownloadすることができます。私は、`vrm 1.0`を使用ていますが、`vrm 0.0`のほうが安定しています。

重要なファイルは`$project/Content/Characters/$model/SK_$model`, `ABP_Post_$model`, `RTG_UEFN_$model`になります。

https://vrm.dev/vrm1/


