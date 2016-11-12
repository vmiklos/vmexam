#!/usr/bin/env node

var fs = require('fs'),
    Path = require('path'),
    pathToUnicodeRegExpJs = Path.resolve(__dirname, 'lib/unicodeRegExp.js'),
    unicodeRegExpSrc = fs.readFileSync(pathToUnicodeRegExpJs, 'utf-8'),
    XRegExp = require('xregexp').XRegExp,
    characterClassNameByXRegExpPredicate = {
        L: 'letter',
        M: 'mark',
        N: 'number',
        P: 'punctuation',
        S: 'symbol',
        Z: 'separator',
        C: 'other'
    };

var letterOrNumberOrSymbolOrPunctuationRegExp = XRegExp('\\p{L}|\\p{N}|\\p{S}');

function isValidUnencodedCharacterInACharacterClass(ch) {
    return letterOrNumberOrSymbolOrPunctuationRegExp.test(ch) && !/[\{\}\[\]\^\$\\\/\|\-]/.test(ch);
}

Object.keys(characterClassNameByXRegExpPredicate).forEach(function (xRegExpPredicate) {
    var characterClassName = characterClassNameByXRegExpPredicate[xRegExpPredicate],
        srcLine = 'unicodeRegExp.' + characterClassName + ' = /' + XRegExp('\\p{' + xRegExpPredicate + '}').source.replace(/\\u([0-9a-f]{4})/gi, function ($0, hexChars) {
            var ch = String.fromCharCode(parseInt(hexChars, 16));
            return isValidUnencodedCharacterInACharacterClass(ch) ? ch : $0;
        }) + '/;';

    unicodeRegExpSrc = unicodeRegExpSrc.replace(new RegExp('unicodeRegExp\\.' + characterClassName + ' = [^\n]*;'), srcLine);
});

fs.writeFileSync(pathToUnicodeRegExpJs, unicodeRegExpSrc, 'utf-8');
