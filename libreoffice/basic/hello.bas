Sub Main
oParaEnum = ThisComponent.getText().createEnumeration()
Do While oParaEnum.hasMoreElements()
	oPara = oParaEnum.nextElement()
'	If oPara.supportsService("com.sun.star.text.Paragraph") Then
		oRangeEnum = oPara.createEnumeration()
		Do While oRangeEnum.hasMoreElements()
			oRange = oRangeEnum.nextElement()
			MsgBox(oRange.getString())
		Loop
'	End If
	Loop
End Sub
