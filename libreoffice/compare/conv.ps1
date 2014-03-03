# http://stackoverflow.com/questions/16534292/basic-powershell-batch-convert-word-docx-to-pdf
# http://www.howtogeek.com/106273/how-to-allow-the-execution-of-powershell-scripts-on-windows-7/
$documents_path = 'z:\tmp\lo\conv'

$word_app = New-Object -ComObject Word.Application

# This filter will find .doc as well as .docx documents
Get-ChildItem -Path $documents_path -Filter *.doc? | ForEach-Object {

    $document = $word_app.Documents.Open($_.FullName)

    $pdf_filename = "$($_.DirectoryName)\$($_.BaseName).pdf"

    $document.SaveAs([ref] $pdf_filename, [ref] 17)

    $document.Close()
}

$word_app.Quit()
