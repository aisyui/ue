# customで衣装の半分が灰色になる

普段の衣装ではライトの加減で服に変な影がうっすら浮き上がる問題がありました。

これを解消するために`type:custom(MI_PostToon)`を当てましたが、customにすると衣装の半分が灰色になる問題が出ました。

これを解消するためにはcustomの`/custom/SK_${model}`に設定されているmaterialを編集し、`MI_PostToon`から`/Plugins/VRM4UContents/MaterialUtil/UE5/Material/MI_VrmMToonOptUnlitOpaque`に切り替えると解消されました。

