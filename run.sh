mkdir -p png
mkdir -p ppm

echo "ファイル名を入力してください: "
read file_name
cargo run -- -o $file_name