

#   OOoLib          2003-08-29-01
#   by Danny Brewer
#
#   A module to easily work with OpenOffice.org.
#
#   Copyright (c) 2003 Danny Brewer
#   Anyone may run this code.
#   If you wish to modify or distribute this code, then
#    you are granted a license to do so under the terms
#    of the Gnu Lesser General Public License.
#   See:  http://www.gnu.org/licenses/lgpl.html




# Python libraries
import sys
sys.path.append("/usr/lib/openoffice.org/program")
import math
import string

# Danny's libraries
import HSBConversions

# OOo's libraries
import uno


#------------------------------------------------------------
#   Uno convenience functions
#------------------------------------------------------------


# The ServiceManager of the running OOo.
oServiceManager = False

def getServiceManager( cHost="localhost", cPort="8100" ):
    """Get the ServiceManager from the running OpenOffice.org.
        Then retain it in the global variable oServiceManager for future use.
    """
    global oServiceManager
    if not oServiceManager:
        # Get the uno component context from the PyUNO runtime
        oLocalContext = uno.getComponentContext()
       
        # Create the UnoUrlResolver on the Python side.
        oLocalResolver = oLocalContext.ServiceManager.createInstanceWithContext(
                                    "com.sun.star.bridge.UnoUrlResolver", oLocalContext )
       
        # Connect to the running OpenOffice.org and get its context.
        oContext = oLocalResolver.resolve( "uno:socket,host=" + cHost + ",port=" + cPort + ";urp;StarOffice.ComponentContext" )
       
        # Get the ServiceManager object
        oServiceManager = oContext.ServiceManager
    return oServiceManager


# This is the same as ServiceManager.createInstance( ... )
def createUnoService( cClass ):
    """A handy way to create a global objects within the running OOo.
    """
    oServiceManager = getServiceManager()
    oObj = oServiceManager.createInstance( cClass )
    return oObj




# The Desktop object.
oDesktop = False

def getDesktop():
    """An easy way to obtain the Desktop object from a running OOo.
    """
    global oDesktop
    if not oDesktop:
        oDesktop = createUnoService( "com.sun.star.frame.Desktop" )
    return oDesktop



# The CoreReflection object.
oCoreReflection = False

def getCoreReflection():
    global oCoreReflection
    if not oCoreReflection:
        oCoreReflection = createUnoService( "com.sun.star.reflection.CoreReflection" )
    return oCoreReflection



def createUnoStruct( cTypeName ):
    """Create a UNO struct and return it.
    """
    oCoreReflection = getCoreReflection()

    # Get the IDL class for the type name
    oXIdlClass = oCoreReflection.forName( cTypeName )

    # Create the struct.
    oReturnValue, oStruct = oXIdlClass.createObject( None )

    return oStruct



def createPropertyValue( cName=None, uValue=None, nHandle=None, nState=None ):
    """Create a com.sun.star.beans.PropertyValue struct and return it.
    """
    oPropertyValue = createUnoStruct( "com.sun.star.beans.PropertyValue" )

    if cName != None:
        oPropertyValue.Name = cName
    if uValue != None:
        oPropertyValue.Value = uValue
    if nHandle != None:
        oPropertyValue.Handle = nHandle
    if nState != None:
        oPropertyValue.State = nState

    return oPropertyValue




#------------------------------------------------------------
#   High level general purpose functions
#------------------------------------------------------------


def openURL( cUrl, tProperties=() ):
    """Open or Create a document from it's URL.
    New documents are created from URL's such as:
        private:factory/sdraw
        private:factory/swriter
        private:factory/scalc
        private:factory/simpress
    """
    oDesktop = getDesktop()
    # TODO: it would be possible to completely hide OOo here:
    # "Did you NOT want the document to appear on screen? Well, then we need to
    # pass an array of property values to the loadComponentFromURL(), which is
    # actually inside my openURL function. The array would contain one
    # PropertyValue whose name is "Hidden" and value is True."
    # http://www.oooforum.org/forum/viewtopic.phtml?t=3451
    oDocument = oDesktop.loadComponentFromURL( cUrl, "_blank", 0, tProperties )
    return oDocument


def makeDrawDocument():
    """Create a new OOo Draw document."""
    return openURL( "private:factory/sdraw" )


def makeWriterDocument():
    """Create a new OOo Writer document."""
    return openURL( "private:factory/swriter" )


def makeCalcDocument():
    """Create a new OOo Calc document."""
    return openURL( "private:factory/scalc" )


def makeImpressDocument():
    """Create a new OOo Impress document."""
    return openURL( "private:factory/simpress" )



#------------------------------------------------------------
#   Functions for working with Draw documents.
#------------------------------------------------------------



#   Notes about some properties and constants for shape objects...

    # LineStyle can be one of...
    #   com.sun.star.drawing.LineStyle.NONE
    #   com.sun.star.drawing.LineStyle.SOLID
    #   com.sun.star.drawing.LineStyle.DASH

    # CircleKind can be one of...
    #   com.sun.star.drawing.CircleKind.FULL
    #   com.sun.star.drawing.CircleKind.SECTION   ' a circle with a cut connected by two lines
    #   com.sun.star.drawing.CircleKind.CUT ' a circle with a cut connected by a line
    #   com.sun.star.drawing.CircleKind.ARC ' a circle with an open cut
   
    # FillStyle can be one of...
    #   com.sun.star.drawing.FillStyle.NONE
    #   com.sun.star.drawing.FillStyle.SOLID
    #   com.sun.star.drawing.FillStyle.GRADIENT
    #   com.sun.star.drawing.FillStyle.HATCH
    #   com.sun.star.drawing.FillStyle.BITMAP
   
    # TextHorizontalAdjust can be one of...
    #   com.sun.star.drawing.TextHorizontalAdjust.LEFT
    #   com.sun.star.drawing.TextHorizontalAdjust.CENTER
    #   com.sun.star.drawing.TextHorizontalAdjust.RIGHT
    #   com.sun.star.drawing.TextHorizontalAdjust.BLOCK
   
    # TextVerticalAdjust can be one of...
    #   com.sun.star.drawing.TextVerticalAdjust.TOP
    #   com.sun.star.drawing.TextVerticalAdjust.CENTER
    #   com.sun.star.drawing.TextVerticalAdjust.BOTTOM
    #   com.sun.star.drawing.TextVerticalAdjust.BLOCK
   
    # TextFitToSize can be one of...
    #    com.sun.star.drawing.TextFitToSizeType.NONE
    #    com.sun.star.drawing.TextFitToSizeType.PROPORTIONAL
    #    com.sun.star.drawing.TextFitToSizeType.ALLLINES
    #    com.sun.star.drawing.TextFitToSizeType.RESIZEATTR

# Useful code snippets...

    # Accessing pages of a drawing document.
    #   oDrawPage = oDrawDoc.getDrawPages().getByIndex( 0 )
    #   oDrawPage = oDrawDoc.getDrawPages().getCount()
    #   oDrawPage = oDrawDoc.getDrawPages().insertByIndex( 1 )



#------------------------------------------------------------
#   Document functions
#------------------------------------------------------------


def setDrawPageOrientationLandscape( oDrawPage ):
    """Pass in any GenericDrawPage object, and this changes it to landscape orientation,
     in addition to swapping the height/width as you would expect.
    """
    # Save some settings
    nOldWidth = oDrawPage.Width
    nOldHeight = oDrawPage.Height
    nOldBorderTop = oDrawPage.BorderTop
    nOldBorderLeft = oDrawPage.BorderLeft
    nOldBorderRight = oDrawPage.BorderRight
    nOldBorderBottom = oDrawPage.BorderBottom
   
    # Change so that it will PRINT in landscape
    oDrawPage.Orientation = uno.getConstantByName( "com.sun.star.view.PaperOrientation.LANDSCAPE" )

    # Now change some paper dimensions to match
    oDrawPage.Width = nOldHeight
    oDrawPage.Height = nOldWidth
    oDrawPage.BorderTop = nOldBorderRight
    oDrawPage.BorderLeft = nOldBorderTop
    oDrawPage.BorderRight = nOldBorderBottom
    oDrawPage.BorderBottom = nOldBorderLeft



#------------------------------------------------------------
#   Shape functions
#------------------------------------------------------------



def makeRectangleShape( oDrawDoc, oPosition=None, oSize=None ):
    """Create a new RectangleShape with an optional position and size."""
    oShape = makeShape( oDrawDoc, "com.sun.star.drawing.RectangleShape", oPosition, oSize )
    return oShape


def makeEllipseShape( oDrawDoc, oPosition=None, oSize=None ):
    """Create a new EllipseShape with an optional position and size."""
    oShape = makeShape( oDrawDoc, "com.sun.star.drawing.EllipseShape", oPosition, oSize )
    return oShape


def makeLineShape( oDrawDoc, oPosition=None, oSize=None ):
    """Create a new LineShape with an optional position and size."""
    oShape = makeShape( oDrawDoc, "com.sun.star.drawing.LineShape", oPosition, oSize )
    return oShape


def makeTextShape( oDrawDoc, oPosition=None, oSize=None ):
    """Create a new TextShape with an optional position and size."""
    oShape = makeShape( oDrawDoc, "com.sun.star.drawing.TextShape", oPosition, oSize )
    return oShape


def makePoint( nX, nY ):
    """Create a com.sun.star.awt.Point struct."""
    oPoint = createUnoStruct( "com.sun.star.awt.Point" )
    oPoint.X = nX
    oPoint.Y = nY
    return oPoint


def makeSize( nWidth, nHeight ):
    """Create a com.sun.star.awt.Size struct."""
    oSize = createUnoStruct( "com.sun.star.awt.Size" )
    oSize.Width = nWidth
    oSize.Height = nHeight
    return oSize


def findShapeByName( oShapes, cShapeName ):
    """Find a named shape within an XShapes interface.
    oShapes can be a drawing page, which supports the XShapes interface.
    Thus, you can find a named shape within a draw page, or within a grouped shape,
     or within a selection of sseveral shapes.
    """
    nNumShapes = oShapes.getCount()
    for i in range( nNumShapes ):
        oShape = oShapes.getByIndex( i )
        cTheShapeName = oShape.getName()
        if cTheShapeName == cShapeName:
            return oShape
    return None


def makeShape( oDrawDoc, cShapeClassName, oPosition=None, oSize=None ):
    """Create a new shape of the specified class.
    Position and size arguments are optional.
    """
    oShape = oDrawDoc.createInstance( cShapeClassName )

    if oPosition != None:
        oShape.Position = oPosition
    if oSize != None:
        oShape.Size = oSize

    return oShape



#------------------------------------------------------------
#   Color manipulation
#------------------------------------------------------------


def rgbColor( nRed, nGreen, nBlue ):
    """Return an integer which repsents a color.
    The color is specified in RGB notation.
    Each of nRed, nGreen and nBlue must be a number from 0 to 255.
    """
    return (int( nRed ) & 255) << 16 | (int( nGreen ) & 255) << 8 | (int( nBlue ) & 255)


def hsbColor( nHue, nSaturation, nBrightness ):
    """Return an integer which repsents a color.
    The color is specified in HSB notation.
    Each of nHue, nSaturation and nBrightness must be a number from 0.0 to 1.0.
    """
    nRed, nGreen, nBlue = HSBConversions.HSBtoRGB( nHue, nSaturation, nBrightness )
    return rgbColor( nRed, nGreen, nBlue )


def redColor( nColor ):
    """Return the Red component of a color as an integer from 0 to 255.
    nColor is an integer representing a color.
    This function is complimentary to the rgbColor function.
    """
    return (int( nColor ) >> 16) & 255


def greenColor( nColor ):
    """Return the Green component of a color as an integer from 0 to 255.
    nColor is an integer representing a color.
    This function is complimentary to the rgbColor function.
    """
    return (int( nColor ) >> 8) & 255


def blueColor( nColor ):
    """Return the Blue component of a color as an integer from 0 to 255.
    nColor is an integer representing a color.
    This function is complimentary to the rgbColor function.
    """
    return int( nColor ) & 255





#------------------------------------------------------------
#   Drawing routines
#------------------------------------------------------------


# Multiply this number by an OOo angle in 100'ths of a degree
#  to convert to radians.
nRadiansPerHundredthDegree = math.pi / 18000


def drawLine( oDrawDoc, oDrawPage, x1,y1, x2,y2, nLineColor=None ):
    """Draw a line from x1,y1 to x2,y2.  Optionally specify line color.
    This adds the LineShape to the page.
    The LineShape is returned.
    """
    # make sure size is non-zero
    #if x1 = x2: x2 = x1 + 1
    #if y1 = y2: y2 = y1 + 1

    oPosition = makePoint( x1, y1 )
    oSize = makeSize( x2-x1, y2-y1 )

    oShape = makeLineShape( oDrawDoc, oPosition, oSize )

    if nLineColor != None:
        oShape.LineColor = nLineColor
    #oShape.LineWidth = 0

    oDrawPage.add( oShape )
    return oShape


def drawLineVector( oDrawDoc, oDrawPage, x1,y1, nAngle,nDistance, nLineColor=None ):
    """Draw a line from x1,y1 in the direction of nAngle, for a distance of nDistance.
    nAngle is measured in radians, clockwise from the 3 O'Clock (east) direction.
    nDistance is in 1000ths of a centimeter.
    This adds the LineShape to the page.
    The LineShape is returned.
    """
    nDX = math.cos( nAngle ) * nDistance
    nDY = math.sin( nAngle ) * nDistance

    return drawLine( oDrawDoc, oDrawPage, x1,y1, x1+nDX,y1+nDY, nLineColor )


def drawAutoSizingText( oDrawDoc, oDrawPage,
                        cText, nHeight, nExtraWidthPercent=40 ):
    """Create a TextShape that will automatically resize its characters
     to its shape bounding rectangle.
    The TextShape is created with a specified height.
    The width is determined based on the natural character width of the
     text.
    The initial position of the text is -10000,-10000, so that the text is not visible.
    This returns the TextShape, which has already been added to the drawing page,
     at coordinates which make it invisible.
    You must set the object's Position property to make the text visible.
    (Hint: You may look at the Size property to help you determine where
     you want to place the text, for instance if you are trying to center
     it, or place it relative to some other shape object.)
    If you don't supply nExtraWidthPercent, then a default fudge factor is used.
    This is the percentage of the average character width, which is added to the shape's
     total width.
    """
    # Create TextShape
    oShape = makeTextShape( oDrawDoc, makePoint( -10000, -10000 ), makeSize( 1, 1 ) )

    # Add it to the page.
    oDrawPage.add( oShape )

    # Make text stick to upper left corner of the shape rather than centered within the shape.
    oShape.TextHorizontalAdjust = uno.getConstantByName( "com.sun.star.drawing.TextHorizontalAdjust.LEFT" )
    oShape.TextVerticalAdjust = uno.getConstantByName( "com.sun.star.drawing.TextVerticalAdjust.TOP" )

    # Make the shape auto-grow in size, based on the text.
    # Once we set the text, in the next step, the shape will grow to some
    #  unknown size, based on the current font in use.
    #oShape.TextAutoGrowHeight = True
    oShape.TextAutoGrowWidth = True

    # Set the text of the TextShape.
    # Because of the TextAutoGrowWidth, the shape now occupies some unknown size.
    oShape.setString( cText )

    # Get the shape's current size.
    nSaveHeight = oShape.Size.Height
    nSaveWidth = oShape.Size.Width

    # Make the shape NOT auto-grow in size, based on the text.
    #oShape.TextAutoGrowHeight = False
    oShape.TextAutoGrowWidth = False

    # This next setting causes the TextShape to automatically resize its characters
    #  to fit the size of the text shape.
    oShape.TextFitToSize = uno.getConstantByName( "com.sun.star.drawing.TextFitToSizeType.PROPORTIONAL" )

    # Calculate the new width, based on the desired height,
    #  and saved width/height ratio for the current font in use.
    nWidth = nSaveWidth * (float(nHeight) / nSaveHeight)

    nAverageCharacterWidth = nWidth / len( cText )

    nExtraWidth = nAverageCharacterWidth * (nExtraWidthPercent / 100.0)

    oShape.TextLeftDistance = nExtraWidth
    oShape.TextRightDistance = nExtraWidth
    nWidth = nWidth + 2 * nExtraWidth

    # Now resize the TextShape.
    oShape.Size.Width = nWidth
    oShape.Size.Height = nHeight

    return oShape


def drawArcPath( oDrawDoc, oDrawPage,
                 nStartX, nStartY, nStartAngle,
                 nArcAngle, nArcRadius, bTurnLeft ):
    """Draw an arc of a circle from a starting position and direction.
    The arc has a certian radius and arc angle,
     and turns either to the left or to the right.
    Parameters:
    nStartX and nStartY are the starting position of the "turtle".
    nStartAngle is the angle direction that the turtle is pointing,
     in 100'ths of a degree.
    An arc is drawn by moving the turtle forward and turning either left or right as it moves.
    nArcAngle is the angle of the arc describing the turtle's path,
     in 100'ths of a degree.
    nArcRadius is the radius of the arc describing the turtle's path.
    bTurnLeft is True if the turtle turns to the left, or False to turn to the right.
    Four values are returned.
    (1) the circle shape, (2) the new X position, (3) new Y position, and (4) new angle.
    Use the last three return values as initial values to draw another arc
     which is connected to the ending position of the arc just drawn.
    """

    # Figure out how big of a circle that the arc is a part of.
    nCircleDiameter = nArcRadius + nArcRadius
    oCircleSize = makeSize( nCircleDiameter, nCircleDiameter )

    # Figure out the position of the circle (ellipse) shape object.
    #
    # Determine the angle of the center point from the starting position.
    if bTurnLeft:
        nCenterPointAngle = nStartAngle + 9000
    else:
        nCenterPointAngle = nStartAngle - 9000
    # Convert to radians.
    nCenterPointAngle = nCenterPointAngle * nRadiansPerHundredthDegree
    # Determine where the center of the circle shape should be.
    nCenterX = nStartX + (nArcRadius * math.cos( nCenterPointAngle ))
    nCenterY = nStartY - (nArcRadius * math.sin( nCenterPointAngle ))
    oCirclePosition = makePoint( nCenterX - nArcRadius, nCenterY - nArcRadius )

    # Figure out what arc portion of the circle needs to be drawn.
    # Angles measures in 100'ths of a degree.
    if bTurnLeft:
        nCircleStartAngle = nStartAngle - 9000
        nCircleEndAngle = nCircleStartAngle + nArcAngle
    else:
        nCircleEndAngle = nStartAngle + 9000
        nCircleStartAngle = nCircleEndAngle - nArcAngle
 
    # Make the circle shape.
    oCircle = makeEllipseShape( oDrawDoc, oCirclePosition, oCircleSize )
    # Fill in its properties
    oCircle.FillStyle = uno.getConstantByName( "com.sun.star.drawing.FillStyle.NONE" )
    oCircle.CircleKind = uno.getConstantByName( "com.sun.star.drawing.CircleKind.ARC" )
    oCircle.CircleStartAngle = nCircleStartAngle
    oCircle.CircleEndAngle = nCircleEndAngle
    # Put it on the drawing
    oDrawPage.add( oCircle )

    # Figure out the ending turtle location and direction.
    if bTurnLeft:
        nEndAngle = nStartAngle - 9000 + nArcAngle
    else:
        nEndAngle = nStartAngle + 9000 - nArcAngle
    # Convert to radians
    nEndAngleRad = nEndAngle * nRadiansPerHundredthDegree
    # Ending Position
    nEndX = nCenterX + (nArcRadius * math.cos( nEndAngleRad ))
    nEndY = nCenterY - (nArcRadius * math.sin( nEndAngleRad ))
    # Ending angle
    if bTurnLeft:
        nEndAngle = normalizeOOoAngle( nEndAngle + 9000 )
    else:
        nEndAngle = normalizeOOoAngle( nEndAngle - 9000 )

    return oCircle, nEndX, nEndY, nEndAngle


def drawSpiralOfArcs( oDrawDoc, oDrawPage,
                      nStartX, nStartY, nStartAngle,
                      nArcAngle, nArcRadius, bTurnLeft,
                      nNumArcs,
                      nRadiusGrowthMultiplier=1.0, nRadiusGrowthAddIn=0 ):
    """Draw a spiral starting from a certian position and direction.
    The spiral turns either towards the left or towards the right.
    Parameters:
    nStartX and nStartY are the starting position of the "turtle".
    nStartAngle is the angle direction that the turtle is pointing,
     in 100'ths of a degree.
    A spiral is drawn by moving the turtle forward and turning either left or right as it moves.
    nArcAngle is the angle of the first arc of the spiral,
     in 100'ths of a degree.
    nArcRadius is the radius of the first arc of the spiral.
    bTurnLeft is True if the turtle turns to the left, or False to turn to the right.
    nNumArcs is the number of arcs which are drawn.
    nRadiusGrowthMultiplier - the radius of the arc is multiplied by this after each arc is drawn.
     Use a number such as 1.1 to cause the radius to grow geometrically as the spiral turns.
    nRadiusGrowthAddIn - this is added to the radius after each arc.
     Use a number, such as the original nArcRadius / number of arcs in a complete circle to cause the radius to grow arithmeteically as the spiral turns.
    Four values are returned.
    (1) the spiral shape, (2) the new X position, (3) new Y position, and (4) new angle.
    Use the last three return values as initial values to draw another arc
     which is connected to the ending position of the arc just drawn.
    """
    nX = nStartX
    nY = nStartY
    nAngle = nStartAngle

    oShapesToGroup = createUnoService( "com.sun.star.drawing.ShapeCollection" )
    for i in range( nNumArcs - 1 ):
        oArcShape, nX, nY, nAngle = drawArcPath( oDrawDoc, oDrawPage, nX, nY, nAngle, nArcAngle, nArcRadius, bTurnLeft )
        nArcRadius = nArcRadius * nRadiusGrowthMultiplier + nRadiusGrowthAddIn

        oShapesToGroup.add( oArcShape )
    oSpiralShape = oDrawPage.group( oShapesToGroup )
    return oSpiralShape, nX, nY, nAngle



#------------------------------------------------------------
#   Styles
#------------------------------------------------------------


def defineStyle( oDrawDoc, cStyleFamily, cStyleName, cParentStyleName=None ):
    """Add a new style to the style catalog if it is not already present.
    This returns the style object so that you can alter its properties.
    """

    oStyleFamily = oDrawDoc.getStyleFamilies().getByName( cStyleFamily )

    # Does the style already exist?
    if oStyleFamily.hasByName( cStyleName ):
        # then get it so we can return it.
        oStyle = oStyleFamily.getByName( cStyleName )
    else:
        # Create new style object.
        oStyle = oDrawDoc.createInstance( "com.sun.star.style.Style" )

        # Set its parent style
        if cParentStyleName != None:
            oStyle.setParentStyle( cParentStyleName )

        # Add the new style to the style family.
        oStyleFamily.insertByName( cStyleName, oStyle )

    return oStyle


def defineGraphicsStyle( oDrawDoc, cStyleName, cParentStyleName=None ):
    """Add a new style to the style catalog if it is not already present.
    This returns the style object so that you can alter its properties.
    """
    return defineStyle( oDrawDoc, "graphics", cStyleName, cParentStyleName )


def getStyle( oDrawDoc, cStyleFamily, cStyleName ):
    """Lookup and return a style from the document.
    """
    return oDrawDoc.getStyleFamilies().getByName( cStyleFamily ).getByName( cStyleName )


def getGraphicsStyle( oDrawDoc, cStyleName ):
    """Lookup and return a graphics style from the document.
    """
    return getStyle( oDrawDoc, "graphics", cStyleName )



#------------------------------------------------------------
#   General Utility functions
#------------------------------------------------------------

def normalizeOOoAngle( nAngleOOo ):
    """Given an angle in 100'ths of a degree,
    adjust it to be from 0 to 360 degrees."""
    if nAngleOOo < 0:
        nSign = -1
    else:
        nSign = 1
    nAngleOOo = nAngleOOo - (int( abs( nAngleOOo ) / 36000.0 ) * 36000 * nSign)
    if nAngleOOo < 0:
        nAngleOOo += 36000
    return nAngleOOo

def pathnameToUrl( cPathname ):
    """Convert a Windows or Linux pathname into an OOo URL."""
    if len( cPathname ) > 1:
        if cPathname[1:2] == ":":
            cPathname = "/" + cPathname[0] + "|" + cPathname[2:]
    cPathname = string.replace( cPathname, "\\", "/" )
    cPathname = "file://" + cPathname
    return cPathname 
