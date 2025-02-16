# 階段を登る

階段でのめり込む問題があります。これを解消します。

1. `/Blueprints/ABP_SandboxCharacter`でAnimGraphの中からFoot Placementを探す。 
2. 骨盤(pelvis)の悪態道補正モードを`Sudden Motion Only`から`Component Space`にする。

また、階段の段差(Step Height)は小さくしてください。のめり込まなくなった反面、スムーズさがなくなり画面がガタガタ揺れます。

## 階段を作る

1. map(level)を開く
2. `編集モード -> モデリング -> 作成 -> 階段`
3. `Step Height: 10.0`

