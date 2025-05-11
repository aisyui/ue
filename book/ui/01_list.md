# listview

1. BPでuser widgetを2つ作ります。`(1)WBP_player`, `(2)WBP_text`とします。WBP_textには`TextBlock`を追加します。
2. BPでobjectを作ります。`(3)BP_text`とします。変数にtextを作ります。スポーン時に公開します。
3. WBP_textはclassの実装インターフェイスに`UserObjectListEntry`を検索して追加します。これで`EventOnListItemObjectSet`を使えるようになります。そこから`cast BP_text`してtextを取得し、それをTextBlockにSetTextします。
4. WBP_playerには`ListView`を追加し、`WBP_text`を参照します。

これでforから値をlist化することができます。WBP_playerでEvent Constructを追加し、そこからfor -> Construct Objectに`BP_text`を参照し、textに入れいます。最後に`ListView -> Add item`します。

### WBP_text

<iframe src="https://blueprintue.com/render/__2rf1mk/2" scrolling="no" allowfullscreen style="width:100%;height:400px"></iframe>

### WBP_player

<iframe src="https://blueprintue.com/render/__2rf1mk/3" scrolling="no" allowfullscreen style="width:100%;height:400px"></iframe>

## ueは簡単なことが複雑すぎる

正直、uiにlistを表示したい場面は多いと思います。しかし、現状、複雑な方法でしか実装できません。

また、`UMG Viewmodel`というpluginがありますが、壊れていました。
