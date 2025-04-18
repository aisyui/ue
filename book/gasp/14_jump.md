# ジャンプの高さを変更する

特殊ジャンプを実装して、敵の近くに着地した場合に特別な技を繰り出す演出を目指します。

<iframe width="100%" height="415" src="https://www.youtube.com/embed/q3UcM8e7B0g?mute=1&rel=0&showinfo=0&controls=0" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>

特殊なジャンプはクールタイムが存在し、10秒間に一回実行できます。ジャンプ中は敵の攻撃を回避できます。通常ジャンプとキーを別にしてもいいですが、同じキーにしたほうがシンプルでいいと思います。

1. jump drop
2. cool time
3. jump attack

まず`skill:1`の場合はjump dropが発動します。発動すると`skill:0`になります。collisionを制御して敵の攻撃が当たらないようにします。もし着地時にboss(enemy)のcollisionにあたっていると、jump attack(ジャンプ技)が発動します。

<iframe src="https://blueprintue.com/render/r4d91x-8/1" scrolling="no" allowfullscreen style="width:100%;height:400px"></iframe>

