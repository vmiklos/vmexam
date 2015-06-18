' Provided that the cursor is at a merged cell, unmerge it and fill all the now
' separate cells with the original value.

oString = ThisComponent.CurrentSelection.String

oDocument = ThisComponent.CurrentController.Frame
oDispatcher = createUnoService("com.sun.star.frame.DispatchHelper")
oDispatcher.executeDispatch(oDocument, ".uno:ToggleMergeCells", "", 0, Array())

oRangeAddress = ThisComponent.CurrentSelection.RangeAddress
oActiveSheet = ThisComponent.CurrentController.ActiveSheet
for i = oRangeAddress.StartColumn to oRangeAddress.EndColumn
    for j = oRangeAddress.StartRow to oRangeAddress.EndRow
        oActiveSheet.getCellByPosition(i, j).String = oString
    next j
next i
