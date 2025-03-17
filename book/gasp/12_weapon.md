# 武器を装備する

武器を手に持つ動作を追加します。

<iframe width="100%" height="415" src="https://www.youtube.com/embed/VronsCH11oo?mute=1&rel=0&showinfo=0&controls=0" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>

1. SKで`装備する箇所`と`手に持つ箇所`のソケットを追加します。
2. そこに武器のasset(mesh)を追加し、previewします。ここでは剣(sword)を使います。
3. animを作ります。例えば、装備場所から手に持つ動作の中で剣が重なり合うようにします。
4. 重なる場所には通知で`DrawnWeapon`, `RestWeapon`を作ります。
5. それをCBP_SandboxCharacterのIAに設定します。`BP_WeaponSword`を作って武器のmeshを入れます。

## 他のキャラクターでズレてしまう

これはManny用のboneに設定されていますので、他のキャラではズレてしまいます。

対処法を考えます。

1. まず同じ名前のソケットをSKのboneに追加します。そして、位置を調整します。
2. event begin playでキャラのmeshを`Character_Mesh`の変数に入れます。それをparentに使います。

<iframe src="https://blueprintue.com/render/ml3h546e/1" scrolling="no" allowfullscreen style="width:100%;height:400px"></iframe>

## niagaraがズレてしまう

animにniagaraを設定している場合もズレてしまいます。これは解決法がありません。

slash(スラッシュ)などは面倒くさいのでそのまま当てていますが、大きくなりすぎています。

他のniagaraは、`BP_WeaponSword`に入れて、都度使用するときに呼び出すようにしています。

面倒ですが体型に合わせたboneを作り、そこにniagaraを付けるといいかもしれませんが、良い解決策は見つかっていません。

