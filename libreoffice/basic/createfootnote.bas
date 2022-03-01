content = ThisComponent.createInstance("com.sun.star.text.Footnote")
Dim args() as new com.sun.star.beans.PropertyValue
ThisComponent.Text.appendTextContent(content, args())
