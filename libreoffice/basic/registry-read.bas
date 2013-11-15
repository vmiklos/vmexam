oProvider = createUnoService("com.sun.star.configuration.ConfigurationProvider")
Dim aParams(0) As new com.sun.star.beans.PropertyValue
aParams(0).Name = "nodepath"
aParams(0).Value = "/org.openoffice.Setup/Product"
oSettings = oProvider.createInstanceWithArguments("com.sun.star.configuration.ConfigurationAccess", aParams)
xray oSettings.getByName("ooName")
