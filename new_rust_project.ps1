if (!$Args[0]) {
    Write-Output "EMPTY ARGUMENT!!"
    Exit
}

$pathToSrc = $Args[0] + "/src"
$pathToTarget = $Args[0] + "/target"
$pathToGitignore = $Args[0] + "/.gitignore"

$pathToMainFile = $Args[0] + "/src/main.rs"

mkdir $Args[0]
mkdir $pathToSrc
mkdir $pathToTarget

New-Item $pathToGitignore -Value "/target"
New-Item $pathToMainFile -Value "fn main() {

}"

Set-Location $Args[0]
cargo init