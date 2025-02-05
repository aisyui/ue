## 新しいキャラを追加する

これには`IKリターゲット`が必要です。

1. `Content/Blueprints/CBP_SandboxCharacter_Manny`をcopyして`CBP_SandboxCharacter_test`を作ります。
2. `CBP_SandboxCharacter_test`を開いて、`Mesh`の下にあるSKM(スケルタルメッシュ)の`Manny`を選択します。その状態で`詳細(details) -> タグ(tags)`からインデックス(index)の0を`RTG_UEFN_to_UE5_test`に書き換えます。
3. `Content/Blueprints/RetargetedCharacters/ABP_GenericRetarget`を開き、変数の`IKRetargeter_Map`に新しく`RTG_UEFN_to_UE5_test`を追加し、ファイルは`/Content/Characters/UE5_Mannequins/Rigs/RTG_UEFN_to_UE5_Mannequin`を参照します。
4. `/Content/Widgets/GameAnimationWidget`を開いて、characterのiconのところをcopyして貼り付けます。そして、詳細からObjectで `Content/Blueprints/CBP_SandboxCharacter_test`を参照します。

これで新しいキャラを使用することが可能になります。

## 飛びつける高さを増やす

1. `Content/Blueprints/CBP_SandboxCharacter`を開いて、関数の`TryTraversalAction`を編集します。
2. 一番下の`Max:275`を`Max:475`に変更します。
3. `/Content/Characters/UEFN_Mannequin/Animations/Traversal/CHT_TraversalAnims`を開きます。
4. `Mantles(編集)`から先程変更した値のところを全部`275 -> 475`に書き換えます。

例えば、levelにあるblockの高さを変更して飛びつけるか確認します。

1. `LevelBlock_TraversableXX`を選択します。
2. トランスフォーム(transform)の拡大(scale)のところで青色(transform-scale-z)を`4.5`にします。

