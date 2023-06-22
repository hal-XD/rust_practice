
# windows 向けクロスコンパイル

`cargo build --target=x86_64-pc-windows-msvc --release`

## 補足

ws2上での話
target=x86_64-pc-windows-gnuはTauriがサポートしていないためコンパイルしても上手く動作しない。(webview2loader.dllが見つけられない)
target=x86_64-pc-windows-msvcはサポートしているようだが、link.exe(VC)が必要になるためコンパイルできない。
