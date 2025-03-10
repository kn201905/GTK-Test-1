## GTK のテスト用に作ってみたコード
以下の内容を、ほぼそのまま書いてみただけのもの。

https://gihyo.jp/article/2023/07/rust-monthly-topics-04-02

---
## GTK4 のインストール
参考 URL  
https://gtk-rs.org/gtk4-rs/stable/latest/book/installation_windows.html

1. 上記の URL では「rustup default stable-msvc」とするようだけど、僕の環境ではうまくできなかったので、代わりに「rustup default stable-x86_64-pc-windows-msvc」を実行した

2. 「gvsbuild build gtk4」で GTK4 をビルドするのは面倒だったため、以下の URL から「GTK4_Gvsbuild_2025.2.0_x64.zip」をダウンロードして、適当なフォルダに展開  
   https://github.com/wingtk/gvsbuild/releases/tag/2025.2.0

3. 上記の URL を参考にして、環境変数 PKG_CONFIG_PATH、Path、Lib に値を設定  


