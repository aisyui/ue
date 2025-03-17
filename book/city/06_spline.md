# 惑星を動かす

splineで惑星を動かします。

1. `BP_Spline`を作成し、componentでsplineを追加し、右クリックで生成パネルを出す
2. 円弧を選択し、90のところを360にする。あとは大きさを変更する数値を大きくする
3. 次にsplineで動かしたいBPを開いて、`BP_Spline`を入れます。
4. 処理をTickに追加する。なお、`BP_Spline(子アクター)`から持ってくる場合は`Get Child Actor`, `Get Component by Class: Spline Component`を使用します。

<iframe src="https://blueprintue.com/render/8gfrd45h/1" scrolling="no" width="100%" height="450px" allowfullscreen=""></iframe>
