## traversableが機能しなくなる

突然、traversable(トラバーサブル)、つまり、よじ登ったりする機能が使えなくなることがあります。

いくつか原因が考えられますが、project設定の`collision > trace channel : Traversable`に問題があるのかもしれません。

collision trace channelに問題が発生するときはけっこう大変です。色々とbugがあり、一度削除して同じ名前で作り直せば動作することもありますが、余計に壊れることもあります。例えば、削除したときにBPのnodeに他のtrace channelが自動で入ってしまうことも要因になります。

