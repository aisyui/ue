# ボスのジャンプ攻撃を作る

1. control rig(CR_Mech)をlevelに配置し、シーケンサからrigを動かしながらanimを作ります。時間ごとにboneの例えば、`base_ctrl`を動かして、`+`を押していきます。
2. 作ろ終わったらsk meshである`CR_Mech`を右クリックで`アニメーションシーケンスをベイク`します。
3. 保存されたanim sequenceを開いて通知などを追加します。攻撃時のみcollisionを有効にする必要があるからです。
4. 通知から受け取る情報でcollisonをenable/disableにします。

<iframe width="100%" height="415" src="https://www.youtube.com/embed/gk28r9cm5eQ?mute=1&rel=0&showinfo=0&controls=0" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>
