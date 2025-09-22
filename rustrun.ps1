$pathToRs = $Args[0] + "\src\main.rs"
$pathToExe = $Args[0] + "\main.exe"
rustc $pathToRs --out-dir $Args[0]
& $pathToExe $Args[1]