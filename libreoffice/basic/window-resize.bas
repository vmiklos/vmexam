' Provided that the current window's height is not 800, trigger exactly one
' resize event.
' This helps debugging vs resizing by mouse.

oWindow = ThisComponent.CurrentController.Frame.ContainerWindow
oPosSize = oWindow.PosSize
oPosSize.Height = 800
oWindow.setPosSize(oPosSize.X, oPosSize.Y, oPosSize.Width, oPosSize.Height, 15)
