# CRでキャラクターを動かす

`control rig`は`CR_xxx`というファイル名がつけられています。

## dragon

例えば、ABPで`head_global_ctrl`の値を更新します。

<iframe src="https://blueprintue.com/render/o3glwh72/1" scrolling="no" allowfullscreen style="width:100%;height:400px"></iframe>

## mech

1. `/Content/ControlRig/Characters/Mech/Meshes/SKM_Mech`からABP, BP, SM(StaticMesh)を作成します。
2. ABPから`Control Rig`で`cannon_ctrl`などを動かせます。

```sh
- /Content/ControlRig/Characters/Mech/BP_Mech
- /Content/ControlRig/Characters/Mech/ABP_Mech
- /Content/ControlRig/Characters/Mech/Meshes/SM_Mech
```

