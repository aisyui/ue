# nice interaction system

[nice interaction system](https://www.fab.com/ja/listings/63b61e4a-dc11-4ee0-a6b1-f4860bd29198)

1. [docs](https://niceshadow.gitbook.io/nice-interaction-system)があります。基本的にGame Modeの設定に入っているPlayer Controller, Game Stateのファイルを開きます。GASは`GM_Snadbox`などがそれにあたります。これらのBPにcomponentを追加します。
2. Player Controller: `AC_PC_Interaction`
> The only thing you need to do now is open your Player Controller and add the AC_PC_Interaction component:
3. Game State: `AC_GS_Interaction`
> Now just add the AC_GS_Interaction component to your game state:
4. project設定にてcollisionのtrace channelに`LookAtTrace`を`ignore`で追加します。
