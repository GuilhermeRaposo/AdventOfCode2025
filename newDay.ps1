$folders = Get-ChildItem -Directory -Filter "day*"

$numbers = $folders | ForEach-Object {
    if ($_.Name -match 'day(\d+)') { [int]$matches[1] }
}

$maxNumber = ($numbers | Measure-Object -Maximum).Maximum

$nextNumber = $maxNumber + 1
$nextFolder = "day$($nextNumber.ToString('00'))"

cargo new $nextFolder
