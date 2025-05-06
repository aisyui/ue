# worldscape plugin

[worldscape plugin](https://www.fab.com/listings/0ef85bf8-a0be-4b74-87f8-b66bb44d6ae2)

本格的な惑星を構築することを目指すpluginです。なお、現状では完璧に動作するわけではありませんので注意が必要です。

## worldscapeとはなにか

worldscapeとは、landscapeのplanet版です。惑星形式の地上を表現します。

ただし、完璧に動作するわけではありません。重力システムが備わっていないので、例えば、真上から惑星内外に行き来するのは可能ですが、ある範囲を超えて横や後ろから惑星に入り、地上に降り立つことはできません。

これを求めていたユーザーは多いと思いますが、現時点ではそれを含めて完璧に実装するのは難しいのだと思います。

では、worldscapeを購入する対象はどういったユーザーになるのでしょう。大気圏から見て地上っぽいものがしっかり表現されることを求めるユーザーです。また、色々なものを改造してplanet systemのようなものを作るのが面倒なユーザーということになります。

私のケースで言うと、worldscapeはあまり必要ではありませんでした。私はUDSを改造し、ocean wavesで海上を作り、地上としてlandscapeを置いて、planet systemを作っていました。

ですから、ほとんどworldscapeと変わらない見栄えになっています。

このplanet systemは5.4ではちゃんと表示されていた地球が5.5でぼやけて表示される問題が発生しています。worldscapeも5.5では地上が点滅するなどの問題が発生しているようです。

## support 5.5

現在、ue5.5をsupportしていませんので、まだ使用できていません。購入者は[discord](https://discord.gg/JD8jvKpmkh)でベータ版の提供を受けることが可能です。`#supoort-access`

> @benjacorp discordID + OrderID

`#version5-5`

<iframe width="100%" height="415" src="https://www.youtube.com/embed/kteb98FM9a4?mute=1&rel=0&showinfo=0&controls=0" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>

1. ue5.4をinstallして、pluginを入れる。
2. `Epic Games/UE_5.4/Engine/Plugins/Marketplace/WorldScape`を`$project/Plugins/`にcopyする。
3. Sourceを`#version5-5`にあるものに入れ替える。`WorldScape.uplugin`のversionを5.5.0に書き換える。
4. `$project`を起動して、pluginをrebuildする。

## UDSとの統合

<iframe width="100%" height="415" src="https://www.youtube.com/embed/pAneS7jCE0U?mute=1&rel=0&showinfo=0&controls=0" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>

## city sampleとの統合

わかりません。
