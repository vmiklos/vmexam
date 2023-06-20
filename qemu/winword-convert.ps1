# Copyright 2023 Miklos Vajna
#
# SPDX-License-Identifier: MIT
#
# Example usage:
# .\winword-convert.ps1 -path "z:/libreoffice/bugs/tdf123456/test.docx"
# Output format is hardcoded to be [MS-DOC] for now.

param (
    [string]$path
)

$wordApp = New-Object -ComObject Word.Application
$file = get-item -Path $path
$document = $wordApp.Documents.Open($file.FullName)
$outFile = "$($file.DirectoryName)/$($file.BaseName).doc"
$format = [Microsoft.Office.Interop.Word.WdSaveFormat]::wdFormatDocument
$document.SaveAs([ref]$outFile, [ref]$format)
$document.Close()
$wordApp.Quit()
