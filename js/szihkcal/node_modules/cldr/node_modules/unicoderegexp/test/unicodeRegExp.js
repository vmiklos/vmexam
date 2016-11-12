/*global describe,it*/

var unexpected = require('unexpected');
    unicodeRegExp = require('../lib/unicodeRegExp');

describe('unicodeRegExp', function () {
    var expect = unexpected.clone();

    expect.addAssertion('[not] to match all characters in', function (expect, subject, value) {
        for (var i = 0 ; i < value.length ; i += 1) {
            expect(value.charAt(i), '[not] to match', subject);
        }
    });

    describe('.letter', function () {
        it('should accept all Danish characters', function () {
            expect(unicodeRegExp.letter, 'to match all characters in', 'abcdefghijklmnopqrstuvwxyzæøå');
        });

        it('should reject numbers', function () {
            expect(unicodeRegExp.letter, 'not to match all characters in', '123456789');
        });
    });

    describe('.number', function () {
        it('should accept all Arabic numbers', function () {
            expect(unicodeRegExp.number, 'to match all characters in', '123456789');
        });

        it('should reject all Danish characters', function () {
            expect(unicodeRegExp.number, 'not to match all characters in', 'abcdefghijklmnopqrstuvwxyzæøå');
        });
    });

    describe('#removeCharacterFromCharacterClassRegExp()', function () {
        expect.addAssertion('[not] to be rewritten to', function (expect, subject, value) {
            expect(unicodeRegExp.removeCharacterFromCharacterClassRegExp(subject[0], subject[1]), 'to equal', value);
        });

        it('should handle an empty character class', function () {
            expect([new RegExp('[]'), 'a'], 'to be rewritten to', new RegExp("[]"));
        });

        it('should remove the only char', function () {
            expect([/[a]/, 'a'], 'to be rewritten to', new RegExp("[]"));
        });

        it('should remove the only char in \\u... syntax', function () {
            expect([/[\u0061]/, 'a'], 'to be rewritten to', new RegExp("[]"));
        });

        it('should remove single matching char, \\u syntax', function () {
            expect([/[\u0061]/, 'a'], 'to be rewritten to', new RegExp("[]"));
        });

        it('should remove single matching char, \\x syntax', function () {
            expect([/[\x61]/, 'a'], 'to be rewritten to', new RegExp("[]"));
        });

        it('should remove single non-matching char', function () {
            expect([/[a]/, 'b'], 'to be rewritten to', /[a]/);
        });

        it('should remove single non-matching char, \\x syntax', function () {
            expect([/[\x61]/, 'b'], 'to be rewritten to', /[a]/);
        });

        it('should remove multiple chars, remove first', function () {
            expect([/[abc]/, 'a'], 'to be rewritten to', /[bc]/);
        });

        it('should remove multiple chars, remove first, \\u syntax', function () {
            expect([/[\u0061\u0062\u0063]/, 'a'], 'to be rewritten to', /[bc]/);
        });

        it('should remove multiple chars, remove first, \\x syntax', function () {
            expect([/[\x61\x62\x63]/, 'a'], 'to be rewritten to', /[bc]/);
        });

        it('should remove multiple chars, remove second', function () {
            expect([/[abc]/, 'b'], 'to be rewritten to', /[ac]/);
        });

        it('should remove multiple chars, remove second, \\u syntax', function () {
            expect([/[\u0061\u0062\u0063]/, 'b'], 'to be rewritten to', /[ac]/);
        });

        it('should remove multiple chars, remove second, \\x syntax', function () {
            expect([/[\x61\x62\x63]/, 'b'], 'to be rewritten to', /[ac]/);
        });

        it('should remove single range, remove first char', function () {
            expect([/[a-z]/, 'a'], 'to be rewritten to', /[b-z]/);
        });

        it('should remove single range, remove first char, \\u syntax', function () {
            expect([/[\u0061-\u007a]/, 'a'], 'to be rewritten to', /[b-z]/);
        });

        it('should remove single range, remove first char, \\x syntax', function () {
            expect([/[\x61-\x7a]/, 'a'], 'to be rewritten to', /[b-z]/);
        });

        it('should remove single range, remove second char', function () {
            expect([/[a-z]/, 'b'], 'to be rewritten to', /[ac-z]/);
        });

        it('should remove single range, remove second char, \\u syntax', function () {
            expect([/[\u0061-\u007a]/, 'b'], 'to be rewritten to', /[ac-z]/);
        });

        it('should remove single range, remove second char, \\x syntax', function () {
            expect([/[\x61-\x7a]/, 'b'], 'to be rewritten to', /[ac-z]/);
        });

        it('should remove single range, remove last char but one', function () {
            expect([/[a-z]/, 'y'], 'to be rewritten to', /[a-xz]/);
        });

        it('should remove single range, remove last char but one, \\u syntax', function () {
            expect([/[\u0061-\u007a]/, 'y'], 'to be rewritten to', /[a-xz]/);
        });

        it('should remove single range, remove last char but one, \\x syntax', function () {
            expect([/[\x61-\x7a]/, 'y'], 'to be rewritten to', /[a-xz]/);
        });

        it('should remove single range, remove last char', function () {
            expect([/[a-z]/, 'z'], 'to be rewritten to', /[a-y]/);
        });

        it('should remove single range, remove last char, \\u syntax', function () {
            expect([/[\u0061-\u007a]/, 'z'], 'to be rewritten to', /[a-y]/);
        });

        it('should remove single range, remove last char, \\x syntax', function () {
            expect([/[\x61-\x7a]/, 'z'], 'to be rewritten to', /[a-y]/);
        });

        it('should remove multiple ranges, remove first char', function () {
            expect([/[0-9a-z]/, 'a'], 'to be rewritten to', /[0-9b-z]/);
        });

        it('should remove multiple ranges, remove second char', function () {
            expect([/[0-9a-z]/, 'b'], 'to be rewritten to', /[0-9ac-z]/);
        });

        it('should remove multiple ranges, remove last char but one', function () {
            expect([/[0-9a-z]/, 'y'], 'to be rewritten to', /[0-9a-xz]/);
        });

        it('should remove multiple ranges, remove last char', function () {
            expect([/[0-9a-z]/, 'z'], 'to be rewritten to', /[0-9a-y]/)
        });
    });

    describe('expandCldrUnicodeSetIdToCharacterClass', function () {
        expect.addAssertion('[not] to be expanded to character class satisfying', function (expect, subject, value) {
            expect(unicodeRegExp.expandCldrUnicodeSetIdToCharacterClass(subject).source, 'to equal', '[' + (value[1] ? '^' : '') + value[0].source.replace(/^\[|\]$/g, '') + ']');
        });

        it('to expand [:S:] to the "symbol" character class, negated', function () {
            expect('[:S:]', 'to be expanded to character class satisfying', [unicodeRegExp.symbol]);
        });

        it('to expand [:^S:] to the "symbol" character class, negated', function () {
            expect('[:^S:]', 'to be expanded to character class satisfying', [unicodeRegExp.symbol, true]);
        });

        it('to expand [:digit:] to the "symbol" character class, negated', function () {
            expect('[:digit:]', 'to be expanded to character class satisfying', [unicodeRegExp.number]);
        });

        it('to expand [:^digit:] to the "symbol" character class, negated', function () {
            expect('[:^digit:]', 'to be expanded to character class satisfying', [unicodeRegExp.number, true]);
        });
    });
});
