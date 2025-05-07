# worldscape plugin

[worldscape plugin](https://www.fab.com/listings/0ef85bf8-a0be-4b74-87f8-b66bb44d6ae2)

本格的な惑星を構築することを目指すpluginです。なお、現状では完璧に動作するわけではありませんので注意が必要です。現在、ue5.5では様々な問題が発生します。リリースされていません。

[良かった点]
1. 広い範囲でlandscapeが自動生成され大気圏から見える
2. 大気圏からは地平線のアーチも描写されている
3. 風の音がスピードに基づき再生される
4. 地球を横から入っても後ろから入っても雲が描写される

[issue]
1. [地上のちらつき](/issue/wsp/wsp_01.html)
2. 惑星地球に横から入った場合、坂道になるので、多分、下からも着陸はできないと思う。重力システムが必要です
3. カスタマイズが複雑でチュートリアルがわからない

<iframe width="100%" height="415" src="https://www.youtube.com/embed/LbjAgOLDkA0?rel=0&showinfo=0&controls=0" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>

![](https://bsky.syu.is/img/feed_fullsize/plain/did:plc:vzsvtbtbnwn22xjqhcu3vd6y/bafkreig5qs3ixzcb2zwivw3o3flpyifmeo6md6psihu67ys5mc75y3ex4i@jpeg)

toolbar(ツールバー)を使うには[docs](https://iolacorp-1.gitbook.io/worldscape-plugin/getting-started/ws-tools/how-to-get-toolbar-running)を参照してください。

## worldscapeとはなにか

worldscapeとは、landscapeのplanet版です。惑星形式の地上を表現します。

ただし、完璧に動作するわけではありません。重力システムが備わっていないので、例えば、真上から惑星内外に行き来するのは可能ですが、ある範囲を超えて横や後ろから惑星に入り、地上に降り立つことはできません。

これを求めていたユーザーは多いと思いますが、現時点ではそれを含めて完璧に実装するのは難しいのだと思います。

では、worldscapeを購入する対象はどういったユーザーになるのでしょう。大気圏から見て地上っぽいものがしっかり表現されることを求めるユーザーです。また、色々なものを改造して統合するのが面倒なユーザーですね。

## support 5.5

現在、worldscapeはue5.5に対応していません。購入者は[discord](https://discord.gg/JD8jvKpmkh)でベータ版の提供を受けることが可能です。`#supoort-access`

> @benjacorp discordID + OrderID

`#version5-5`

<iframe width="100%" height="415" src="https://www.youtube.com/embed/kteb98FM9a4?mute=1&rel=0&showinfo=0&controls=0" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>

1. ue5.4をinstallして、pluginを入れる。
2. `Epic Games/UE_5.4/Engine/Plugins/Marketplace/WorldScape`を`$project/Plugins/`にcopyする。
3. Sourceを`#version5-5`にあるものに入れ替える。`WorldScape.uplugin`のversionを5.5.0に書き換える。
4. `$project`を起動して、pluginをrebuildする。

## UDSとの統合

UDSの統合はかなり大変だと思われます。

まず下記の動画を見てください。

<iframe width="100%" height="415" src="https://www.youtube.com/embed/RkpVzzTAVhw?mute=1&rel=0&showinfo=0&controls=0" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>

[@ArghanionsPuzzlebox]
> 現在の Epic ソリューションでは問題を起こさないようにするのは非常に困難です。私は独自のカスタム Atmosphere マテリアルに取り組んでおり、それが動き回るように機能します。

それをworldscapeに対応させた例です。

<iframe width="100%" height="415" src="https://www.youtube.com/embed/DZeaIJJy1Tg?mute=1&rel=0&showinfo=0&controls=0" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>

チュートリアル動画には、いくつか抜けている点があるようです。

主にplanetとudsを小さくして、それを合わせます。

> "Sky Atmosphere" in the UDS (Self) and turning off `Keep Planet Top at Camera XY Location`

1. `$project/Plugins/WorldScape/Levels/Planets_Levels/Demonstration_EarthLikePlanet`を開いきます。
2. Planet以下にあるWindVolume以外を削除します。例えば、SkyAtmosphereやVolumetricCloudなどです。
3. `Ultra_Dynamic_Sky`, `Ultra_Dynamic_Weather`を置きます。locationなどを0, 0, 0の位置します。
4. `Ultra_Dynamic_Sky -> SkyAtmophere`で以下を設定します。Transform Mode: Planet Center, Ground Radius: 100
5. PlanetでDistance to Freeze Generation: 10000000, Planet Scale: 10000000にします。
6. PlanetBp2でDistance to Freeze Generation: 10000000, Planet Scale: 5000000にします。
7. `Ultra_Dynamic_Sky`でKeep Planet Top at Camera XY Location: falseにします。

`WindVolume`は移動速度に応じて風の音を出します。

惑星は小さくなりますが、これでudsと連携することができました。

<iframe width="100%" height="415" src="https://www.youtube.com/embed/fT1rX3YLF9Q?mute=1&rel=0&showinfo=0&controls=0" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>


