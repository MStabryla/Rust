$pathToRs = $Args[0] + ".rs"
$pathToExe = ".\" + $Args[0] + ".exe"
rustc $pathToRs
& $pathToExe
