#   HSBConversions        2003-08-21-01
#   by Danny Brewer
#
#   HSB to RGB color space conversion routines.
#   Copyright (c) 2003 Danny Brewer
#   Anyone may run this code.
#   If you wish to modify or distribute this code, then
#    you are granted a license to do so under the terms
#    of the Gnu Lesser General Public License.
#   See:  http://www.gnu.org/licenses/lgpl.html



def RGBtoHSB( nRed, nGreen, nBlue ):
    """RGB to HSB color space conversion routine.
    nRed, nGreen and nBlue are all numbers from 0 to 255.
    This routine returns three floating point numbers, nHue, nSaturation, nBrightness.
    nHue, nSaturation and nBrightness are all from 0.0 to 1.0.
    """
    nMin = min( nRed, nGreen, nBlue )
    nMax = max( nRed, nGreen, nBlue )

    if nMin == nMax:
        # Grayscale
        nHue = 0.0
        nSaturation = 0.0
        nBrightness = nMax
    else:
        if nRed == nMin:
            d = nGreen = nBlue
            h = 3.0
        elif nGreen == nMin:
            d = nBlue - nRed
            h = 5.0
        else:
            d = nRed - nGreen
            h = 1.0

        nHue = ( h - ( float( d ) / (nMax - nMin) ) ) / 6.0
        nSaturation = (nMax - nMin) / float( nMax )
        nBrightness = nMax / 255.0

    return nHue, nSaturation, nBrightness



def HSBtoRGB( nHue, nSaturation, nBrightness ):
    """HSB to RGB color space conversion routine.
    nHue, nSaturation and nBrightness are all from 0.0 to 1.0.
    This routine returns three integer numbers, nRed, nGreen, nBlue.
    nRed, nGreen and nBlue are all numbers from 0 to 255.
    """
    # Scale the brightness from a range of 0.0 thru 1.0
    #  to a range of 0.0 thru 255.0
    # Then truncate to an integer.
    nBrightness = int( min( nBrightness * 256.0, 255.0 ) )

    if nSaturation == 0.0:
        # Grayscale because there is no saturation
        nRed = nBrightness
        nGreen = nBrightness
        nBlue = nBrightness
    else:
        # Make hue angle be within a single rotation.
        # If the hue is > 1.0 or < 0.0, then it has
        #  "gone around the color wheel" too many times.
        #  For example, a value of 1.2 means that it has
        #  gone around the wheel 1.2 times, which is really
        #  the same ending angle as 0.2 trips around the wheel.
        # Scale it back to the 0.0 to 1.0 range.
        if nHue > 1.0:
            nHue = nHue - int( nHue )
        elif nHue < 0.0:
            nHue = abs( nHue )
            if nHue > 1.0:
                nHue = nHue - int( nHue )
            nHue = 1.0 - nHue
        # Rescale hue to a range of 0.0 thru 6.0
        nHue = nHue * 6.0
        # Separate hue into int and fractional parts
        iHue = int( nHue )
        fHue = nHue - iHue
        # Is hue even?
        if iHue % 2 == 0:
            fHue = 1.0 - fHue
        #
        m = nBrightness * (1.0 - nSaturation)
        n = nBrightness * (1.0 - (nSaturation * fHue))

        if iHue == 1:
            nRed = n
            nGreen = nBrightness
            nBlue = m
        elif iHue == 2:
            nRed = m
            nGreen = nBrightness
            nBlue = n
        elif iHue == 3:
            nRed = m
            nGreen = n
            nBlue = nBrightness
        elif iHue == 4:
            nRed = n
            nGreen = m
            nBlue = nBrightness
        elif iHue == 5:
            nRed = nBrightness
            nGreen = m
            nBlue = n
        else:
            nRed = nBrightness
            nGreen = n
            nBlue = m
   
    return nRed, nGreen, nBlue 
