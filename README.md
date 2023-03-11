# SA-IS

Suffix Array を線形時間で構築するためのアルゴリズムである，SA-IS の実装．この手法が提案された論文は [Two Efficient Algorithms for Linear Time Suffix Array Construction](https://ieeexplore.ieee.org/document/5582081) であり，このリポジトリの実装は論文のサンプルコードをほぼそのまま Rust に移植したものである．

論文のサンプルコード（C で書かれている）では配列を使いまわしてメモリ使用量を削減しているが，Rust の借用権により一部再現できないところがあったため，少し妥協をしてメモリ使用量が多くなっている．

構築速度については，文字列の長さが 50 万のときに 70 ms くらいという[結果](https://judge.yosupo.jp/submission/129338)が得られた．特に最適化などは行っていない．Rust による提出の中の最速コードが 45 ms であることを考えると，よくやっているほうだといえそう？
