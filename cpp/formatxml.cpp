/*****************************************************************

Copyright (C) 2010 Lubos Lunak <l.lunak@suse.cz>

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL THE
AUTHORS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN
AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

******************************************************************/

/*

This is a tool that formats nicely an XML file (e.g. the .docx or .odt formats
are basically everything in a single line, which is pain to analyze).
Unlike 'xmllint --format', this works even if the XML is corrupt, and unlike
xmllint's --recover it does not alter the XML itself in any way (or at least
tries not to, complain if there's a problem).

If there are problems with the XML, there is also a comment inserted in the output
file that warns about it (so that the problem is easy to spot).

To compile (libQtCore from Qt4 is required, $QTDIR is the location, usually /usr):
g++ -Wall -I$QTDIR/include/QtCore -I$QTDIR/include formatxml.cpp -lQtCore -L$QTDIR/lib -o formatxml

The given file is written to stdout if it's redirected, otherwise it's written
to file with .format.xml appended.

*/

#include <assert.h>
#include <qfile.h>
#include <qstack.h>
#include <qstringlist.h>
#include <qtextstream.h>
#include <stdio.h>

enum TokenType
    {
    Error, // parse error or whatever
    OtherTag, // comments, <? ... ?>
    OpeningTag,
    ClosingTag,
    StandaloneTag, // <foo/>
    Text // whatever text outside of tags
    };

static QStringList readTokens( QTextStream& in )
    {
    QStringList ret;
    while( !in.atEnd())
        {
        QChar c;
        in >> c;
        if( c == '\n' ) // strip line leading whitespace (otherwise keep it, may be empty text between tags)
            {
            in.skipWhiteSpace();
            in >> c;
            }
        if( in.atEnd())
            break;
        if( c == '<' )
            {
            QString str = c;
            while( !in.atEnd())
                {
                in >> c;
                str.append( c );
                if( c == '>' )
                    break;
                }
            ret.append( str );
            }
        else
            {
            QString str = c;
            while( !in.atEnd())
                {
//                if( c == '\n' )
//                    break;
                in >> c;
                if( c == '<' || c == '>' )
                    {
                    in.seek( in.pos() - 1 ); // one char back
                    break;
                    }
                str.append( c );
                }
            ret.append( str );
            }
        }
    return ret;
    }

static QString tagName( const QString& token )
    {
    assert( token.length() >= 3 && token[ 0 ] == '<' );
    int start = ( token[ 1 ] == '/' ? 2 : 1 );
    int after = token.indexOf( ' ' );
    if( after == -1 )
        {
        if( token[ token.length() - 2 ] == '/' )
            after = token.length() - 2; // strip trailing />
        else
            after = token.length() - 1; // string trailing /
        }
    return token.mid( start, after - start );
    }

static TokenType analyzeToken( const QString& token )
    {
    if( token.isEmpty())
        return Error;
    if( token[ 0 ] == '<' )
        {
        if( token.length() >= 4 // <??>
            && ( token[ 1 ] == '?' || token[ 1 ] == '!' ))
            {
            if( token[ token.length() - 1 ] == '>' && token[ 1 ] == token[ token.length() - 2 ] )
                return OtherTag;
            else
                return Error;
            }
        if( token.length() >= 4 // <a/>
            && token[ token.length() - 1 ] == '>' && token[ token.length() - 2 ] == '/' )
            {
            return StandaloneTag;
            }
        if( token.length() >= 4 // </a>
            && token[ 1 ] == '/' && token[ token.length() - 1 ] == '>' )
            {
            return ClosingTag;
            }
        if( token.length() >= 3 // <a>
            && token[ token.length() - 1 ] == '>' )
            {
            return OpeningTag;
            }
        return Error;
        }
    return Text;
    }

static QString indent( int size )
    {
    return QString().fill( ' ', size );
    }

static void ensureNewLine( QTextStream& out, bool* needNewLine )
    {
    if( *needNewLine )
        {
        out << endl;
        *needNewLine = false;
        }
    }

static bool format( QTextStream& in, QTextStream& out )
    {
#define INDENT indent( stack.size() * 2 )
    QStack< QString > stack;
    QStringList tokens = readTokens( in );
    bool needNewLine = false;
    while( !tokens.isEmpty())
        {
        QString token = tokens.takeFirst();
#if 0
        static const char* const types[] = { "Error", "Other", "Opening", "Closing", "Standalone", "Text" };
        QTextStream( stderr ) << "TOKEN(" << types[ analyzeToken( token ) ] << "): " << token << endl;
#endif
        switch( analyzeToken( token ))
            {
            case OpeningTag:
                ensureNewLine( out, &needNewLine );
                out << INDENT << token;
                needNewLine = true;
                stack.push( tagName( token ));
                break;
            case ClosingTag:
                {
                QString tag = tagName( token );
                if( stack.isEmpty())
                    {
                    ensureNewLine( out, &needNewLine );
                    out << "<!-- ERROR: missing opening tag -->" << endl;
                    }
                else if( stack.top() != tag )
                    { // TODO or try to find it in the stack?
                    ensureNewLine( out, &needNewLine );
                    out << "<!-- ERROR: opening/closing tag mismatch -->" << endl;
                    }
                else
                    {
                    stack.pop();
                    }
                if( !needNewLine ) // not line continuation
                    out << INDENT;
                out << token << endl;
                needNewLine = false;
                break;
                }
            case StandaloneTag:
                ensureNewLine( out, &needNewLine );
                out << INDENT << token << endl;
                break;
            case OtherTag:
                ensureNewLine( out, &needNewLine );
                out << INDENT << token << endl;
                break;
            case Text:
                if( !needNewLine ) // not line continuation
                    out << INDENT;
                out << token;
                needNewLine = true;
                break;
            case Error:
                ensureNewLine( out, &needNewLine );
                out << "<!-- ERROR: cannot parse: " << token << "-->" << endl;
                break;
            }
        }
    if( needNewLine )
        out << endl;
    if( stack.size() == 0 )
        return true;
    out << "<!-- ERROR: missing closing tags -->" << endl;
    return false;
#undef INDENT        
    }

int main( int argc, char* argv[] )
    {
    if( argc != 2 )
        {
        QTextStream( stderr ) << "Usage: " << argv[ 0 ] << " <file>" << endl;
        return 2;
        }
    QFile fin( argv[ 1 ] );
    if( !fin.open( QIODevice::ReadOnly ))
        {
        QTextStream( stderr ) << "File " << argv[ 1 ] << " cannot be read" << endl;
        return 3;
        }
    QTextStream in( &fin );
    QFile fout;
    fout.open( stdout, QIODevice::WriteOnly );
    QTextStream out( &fout );
    in.setCodec( "UTF-8" );
    out.setCodec( "UTF-8" );
    return format( in, out ) ? 0 : 1;
    }
