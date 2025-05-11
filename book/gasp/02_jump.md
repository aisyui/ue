# 飛びつける高さを増やす

1. `Content/Blueprints/CBP_SandboxCharacter`を開いて、関数の`TryTraversalAction`を編集します。
2. 一番下の`Max:275`を`Max:475`に変更します。
3. `/Content/Characters/UEFN_Mannequin/Animations/Traversal/CHT_TraversalAnims`を開きます。
4. `Mantles(編集)`から先程変更した値のところを全部`275 -> 475`に書き換えます。

例えば、levelにあるblockの高さを変更して飛びつけるか確認します。

1. `LevelBlock_TraversableXX`を選択します。
2. トランスフォーム(transform)の拡大(scale)のところで青色(transform-scale-z)を`4.5`にします。

