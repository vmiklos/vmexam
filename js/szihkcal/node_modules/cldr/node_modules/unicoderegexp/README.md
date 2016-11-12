unicoderegexp
=============

Various regular expressions for unicode character classes (letter,
punctuation, number, etc.) and helper functions for composing them.

Used by the [purify](https://github.com/One-com/purify) library.

The module exports a bunch of useful RegExps each with a single character class in them:

* `letter`
* `mark`
* `number`
* `punctuation`
* `symbol`
* `separator`
* `other`
* `visible`
* `printable`

```javascript
unicodeRegExp.visible.test("a"); // true
unicodeRegExp.visible.test(" "); // false
unicodeRegExp.visible.test("\u00a0"); // false -- a non-breaking space is not visible
```

To validate an entire string you need to build a new RegExp:

```javascript
var visibleStringRegExp = new RegExp('^' + unicodeRegExp.visible.source + '*$');
visibleStringRegExp.test("foobar"); // true
visibleStringRegExp.test("foo bar"); // false because of the space

unicodeRegExp.removeCharacterFromCharacterClassRegExp(/[æøå]/, 'æ'); // /[\u00f8\u00e5]/
unicodeRegExp.spliceCharacterClassRegExps(/[a-b]/, /[c-d]/); // /[a-bc-d]/
```

The info about which characters belong to which classes was taken from the
[XRegExp](http://xregexp.com/) library and its [Unicode plugin](http://xregexp.com/plugins/#unicode).
