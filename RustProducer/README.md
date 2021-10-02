* Before building, I had to install cmake (adding it to path too)
* And install openssl via https://slproweb.com/products/Win32OpenSSL.html
* Before building, specify openssl via `$env:OPENSSL_DIR = "C:\Program Files\OpenSSL-Win64"`
* Run via `cargo run`

By the way, the c++ version definitely performs better. Something in the rust client just triggers MsMpEng.exe (Windows Defender) and it starts using quite a bit of CPU. 