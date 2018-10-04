' This Source Code Form is subject to the terms of the Mozilla Public
' License, v. 2.0. If a copy of the MPL was not distributed with this
' file, You can obtain one at http://mozilla.org/MPL/2.0/.
'
' Problem to be solved: you inserted an image to Impress and you want to cite
' the source of it. Ideally you do this for each & every image you did not create
' yourself, so it should be easy. The below macro adds a "caption" to the
' currently selected image.

Sub AddSource

oSelectedShape = ThisComponent.CurrentSelection(0)
oText = ThisComponent.createInstance("com.sun.star.drawing.TextShape")
oPosition = oSelectedShape.Position
oSize = oSelectedShape.Size
oPosition.Y = oPosition.Y + oSize.Height
oText.Position = oPosition
oSize.Height = 500 ' ~14pt in mm100
oText.Size = oSize
ThisComponent.DrawPages(0).add(oText)
oText.String = "(via )"
oText.TextHorizontalAdjust = 1 ' center
oText.CharHeight = 14 ' 60%, assuming 24pt default
' Then select the new shape, navigate after "via ", Ctrl-K to add the
' hyperlink, and you're done.

End Sub
