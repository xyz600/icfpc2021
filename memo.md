# 参加記録

## 1日目

### 21:28

問題を読んで、format を除くルールを書いて README に貼り付けた。  
やらないといけないことは、まずインフラ周りかな。

* [x] 各種構造体と json への (de)serializer
* [x] http client の実装
  * [x] 問題の crawler
  * [x] 問題の submitter 

を書いていく。

## 2日目

### 2:05

基本実装が終わった(Rust の async 周りに詳しくなかったので勉強しながらやっていたけど、かなり便利だとわかった)。
色んなバグを修正しつつ問題を見てると「Hole の頂点に対応付けるだけで dislike 0 が獲得できる問題」がそれなりにあることがわかった。
機械提出が上手くいっていそうかを判断するために、12頂点以下全探索で対応付けるだけで何とかなる問題を解いてみる。

### 3:18

機械的な提出が上手く行くことを確認。寝るまでの成果としては一旦十分!!
どんな問題があるかを一通り確認して、作戦会議しながら寝ようかな。

見てみると、どうもまだまだ Hole の頂点に割り当てるだけで解ける問題がたくさんあるらしい。
雑に書いても解けそうだし、ちょっとこれらを攻略したら寝ることにする。

### 4:14

dislike が 0 になるような解答に対して誤解をしていたことがわかった。  
dislike が 0 になることと、全ての figure の頂点が hole のどれかに対応することは話が別で、
hole 1 つの頂点につきどれか 1 つの figure の頂点が対応してさえいれば良い。  

ということで、速攻でできることは終わったので、無事寝れそう(?)

寝る前に、自由に動かせる点については考察してみた。
頂点がある自由度を持って動かせるということは、

* 次数 1 の頂点は円を描くように自由に動かせる
* 次数 2 の頂点が 2 連続で並んでいると、適当な制約条件を満たすように円っぽく動かせる
* eps がある程度大きければ、少し大きめな次数の頂点でも問題なく動かせる

ということは、適当なコスト関数をつければ、単純に焼けばいいという説はあるな…

* 1頂点をランダムに上下左右に動かす近傍
* コスト関数は、以下のように分解できる
  * [x] 穴と頂点間の距離
    * 外側にある場合だけ正のペナルティ
  * dislike の値

みたいなのが動けば、それだけで割と十分解けるようになるのでは？
という訳で、明日の方針は幾何の実装になりそう。
多角形と点の距離はわからないけど、三角形分割をして三角形と点の距離ならすぐにできそう。
三角形分割の方法は、[ここ](https://ja.wikipedia.org/wiki/%E5%A4%9A%E8%A7%92%E5%BD%A2%E3%81%AE%E4%B8%89%E8%A7%92%E5%BD%A2%E5%88%86%E5%89%B2) などが詳しそう。

### 7:02

何故か寝付けず、起きてしまった…
まぁせっかくなので実装を始めよう。

### 10:51

死ぬほど色々バグらせて、ようやっと多角形と任意の点との距離を計算するモジュールが作れた。
疲れた…けどこれが完成したら、とりあえず山登りのようなことはできるので実装する！

### 12:40

実装は出来たけど、三角形分割の理屈に穴があった…(単調多角形である性質が必要なアルゴリズムだったけど、そうではない)

### 15:15

幾何のコーナーケースが無限に多くて泣いてる。

### 18:06

直感的に理解できるように、機械的にビジュアルでわかりやすいような図を出力するスクリプトを書きながら、ついにおよそバグらしいバグを取りきることに成功したように見える。
ただ、ルールを勘違いしていたらしく、整数で出力しなきゃいけないらしい。入力だけじゃなくて出力もそうである必要があるとは…
最終的に量子化すれば大丈夫と信じて、制約を満たすように適当に丸める

### 21:05

色々試そうとしていたのだけど、全く頭が働かない…
多分試しても何も得られないので、今日は諦めて寝よう

## 3日目

### 9:03

起床。集中力がないので、起きて散歩して体操してた。

### 10:12

目覚めてきたので少し考えてみた。
よく考えなくても、実数の空間で近傍を定めるのかなりきつかったな。
とりあえず、全部整数の空間で、上下左右に1マス動くような近傍で考えてみる。

### 12:58

上下左右だけだと若干辛いケースがあったので、グラフの隣接点の swap を入れるなどをすると、11 が解けるようになった。
が、自分の制約違反チェックがかなり甘く「hole を形成する辺と平行でなく交わってはいけない」という条件があることに気づいた。
ので、実装していく

それに加えて、距離のチェックが何故かめちゃくちゃ甘くなっているので、理由を調査する。

### 13:31

距離が2乗で定義されていましたね…それはガバガバでも仕方ない。
およそバグらしいものは(上の制約を除いて)なさそうなので、
一旦 submit してみる。
→ 機能としては大丈夫そうなので、とにかく上の確認を実装しよう

### 14:10

ソルバの実行並列化すらしてなかったので、このタイミングで rayon でさくっとやる

### 14:50

とりあえず遅いやつは作った。
きちんと機能していそうで、大体 10万点くらいの点数が得られる。

ここらで、ちょっと自分が出したベストの解を保存する機能を用意しよう。
といっても、評価器は Rust にあるやつしか使えないので、グローバルな解を保存するフォルダを決めておいて
その解を読んでコストが上回っていたら上書きする、みたいな仕掛けを入れておけば良さそう。

### 15:27

ベストを保存する仕組みが実装終わった。
ついでに、問題を作って解を保存するのと、
やっと本当の戦いに入れる…

### 16:08

忘れてたけど、焼きなます時に解を書き戻す修正を入れた。
viewer を見ていたけど、例えば 3 等を見ると、1手好きなところに置いてくれと言われたらいくらでも改善点が見つかることがわかる。
そこで、とりあえず全部のマスを近傍にして、とりあえず貪欲に置く改善をやるだけやって、焼き鈍しの初期解を作ることを考えてみる。

### 16:47

多少改善するやつはあったけど、よく考えたら多くの場合は1点動かしてもどうにもならないからなぁ。
となると、次は高速化について考えてみたい。
本質的にはいい初期解を作る方法がないとどうにもならなくて、それをやるには、原理的に置ける店の候補を高速に列挙できないと話しにならない…

今、問題なのは hole の辺と figure の辺が交わるかどうかを動的に確認せざるを得ない状況になっていて、  
これをある程度緩和できる方法があるといい。

考えていたのは、ある頂点毎に hole の位置関係から設置可能な頂点の位置は既に決定しているので、そこから条件を絞れないだろうか、という話。
これは、hole 内部である点を囲む最大限に大きい可視領域が見えればいいことになる。

### 17:33

多角形内で、ある点からの可視領域をチェックするアルゴリズムを実装することにした。
多分理解できたので、何とかなりそう。

## 4日目

### 2:57

これさえ出来れば、色々と高速化されるはずなのに、可視領域のチェックが鬼門過ぎる…

### 3:23

色々試行錯誤して考えた末、どうも base になる点と各頂点の可視性の判定さえ出来れば、近似的とはいえ可視領域が列挙できる気がする。
ちょっとやってみるかと思ったけど、だいぶ眠かったので諦め。

### 9:00

起床。少しぼーっとしてるけど、続きやる。

### 10:20

めちゃくちゃ簡単に出来た。ちょっと遅い気がするけど多分大丈夫でしょう。

### 11:15

UT が落ちるテストケースを追っていたら、自分が想像していたのと全然違う幾何のコーナーケースが生まれて、修復不可能になった。
頑張ってどうしたらいいかを考えている。

### 16:17

本格的にどうしようもなくなったので、(最初からやれよという話だが)そういう問題をググり始めている。
[visibility polygon](https://en.wikipedia.org/wiki/Visibility_polygon) というのをみて、それらしいアルゴリズムがあったので勉強している。
[これ](http://www.science.smith.edu/~jorourke/books/ArtGalleryTheorems/Art_Gallery_Chapter_8.pdf) や [これ](https://citeseerx.ist.psu.edu/viewdoc/download?doi=10.1.1.760.1230&rep=rep1&type=pdf) 等を眺めているが、集中力がだいぶ厳しい。

### 18:06

あと 3 時間で本質的にできることがなくなったので、おしまいです。

# 所感

* 一人で参加する目的の一つに、普段チームメイトに色々準備してもらっている下回りを自力でもできるようになりたいなという気持ちがあった
  * 最低限の機能を有するものであれば、ちゃんと便利に使えるものが作れてよかった
  * 一人で出る分には、ここの部分は結局のボトルネックにならないかも？
* Rust でできることが増えた
  * Rust の async / await の取り扱い方、http-client、json の扱いなどを勉強できた
  * いつもよりも UT をかなりたくさん書いたので、言語でテストサポートあるとやっぱり便利でありがたい
  * 1つのライブラリに対して複数のバイナリを生やせるシステムが便利で、ちょっとした簡易ツールをたくさん作ろうという気持ちにとてもなった
* 結局ボトルネックはアルゴリズム周りだとわかった
  * 直感的にどうやったらいいのかあんまり知らないことが多かったので、調べたりコーナーケースを全列挙しながら調べたりしたら無限に時間が溶けた
  * これからも幾何が出るかもしれないコンテストに出る気なら絶対ライブラリ整備したほうがいいなと思った
    * そもそも計算幾何に強くなりたいモチベが別にあるので、少しライブラリ整備をやってみる
* ICFPC はチームで参加するのが楽しいなという気持ちになった
  * AHC が始まってからコンテストに少しずつ出るようになって、個人戦はたくさんやっているので