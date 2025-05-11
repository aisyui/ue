# キャラクターを追加する

これには`IKリターゲット`が必要です。

1. `Content/Blueprints/CBP_SandboxCharacter_Manny`をcopyして`CBP_SandboxCharacter_test`を作ります。
2. `CBP_SandboxCharacter_test`を開いて、`Mesh`の下にあるSKM(スケルタルメッシュ)の`Manny`を選択します。その状態で`詳細(details) -> タグ(tags) -> Component Tags`から`index[0]`を`RTG_UEFN_to_UE5_test`に書き換えます。
3. `Content/Blueprints/RetargetedCharacters/ABP_GenericRetarget`を開き、変数の`IKRetargeter_Map`に新しく`RTG_UEFN_to_UE5_test`を追加し、ファイルは`/Content/Characters/UE5_Mannequins/Rigs/RTG_UEFN_to_UE5_Mannequin`を参照します。
4. `/Content/Widgets/GameAnimationWidget`を開いて、characterのiconのところをcopyして貼り付けます。そして、詳細からObjectで `Content/Blueprints/CBP_SandboxCharacter_test`を参照します。

これで新しいキャラを使用することが可能になります。

