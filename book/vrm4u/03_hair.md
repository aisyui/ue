# 前髪の角度を調整する

前髪が少し浮き上がっていたので、角度を調整する方法を紹介します。

これは`/Content/Character/$model/VM_${model}_VrmMeta`で調整します。具体的には以下のような値にすればいいでしょう。

```json
[
  {
    "bone Name": "J_Sec_Hair1_03",
    "Hit Radius": 0
  },
  {
    "bone Name": "J_Sec_Hair2_03",
    "Hit Radius": 0.01
  },
  {
    "bone Name": "J_Sec_Hair3_03",
    "Hit Radius": 0.01
  }
],
[
  {
    "bone Name": "J_Sec_Hair1_04",
    "Hit Radius": 0
  },
  {
    "bone Name": "J_Sec_Hair2_04",
    "Hit Radius": 0.01
  },
  {
    "bone Name": "J_Sec_Hair3_04",
    "Hit Radius": 0.01
  }
],
[
  {
    "bone Name": "J_Sec_Hair1_05",
    "Hit Radius": 0
  },
  {
    "bone Name": "J_Sec_Hair2_05",
    "Hit Radius": 0.01
  },
  {
    "bone Name": "J_Sec_Hair3_05",
    "Hit Radius": 0.01
  }
]
```

飛行時に髪が爆散する問題は以下です。

```json
{
  "bone Name": "J_Sec_Hair2_03",
  "Hit Radius": 0.0
},
{
  "bone Name": "J_Sec_Hair1_09",
  "Hit Radius": 0.01
},
{
  "bone Name": "J_Sec_Hair1_10",
  "Hit Radius": 0.01
}
```


