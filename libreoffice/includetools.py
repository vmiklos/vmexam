#!/usr/bin/env python3
#
# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.
#

import os


stdCppHeaders = [
    "<cstdlib>",
    "<csignal>",
    "<csetjmp>",
    "<cstdarg>",
    "<typeinfo>",
    "<typeindex>",
    "<type_traits>",
    "<bitset>",
    "<functional>",
    "<utility>",
    "<ctime>",
    "<chrono>",
    "<cstddef>",
    "<initializer_list>",
    "<tuple>",
    "<new>",
    "<memory>",
    "<scoped_allocator>",
    "<climits>",
    "<cfloat>",
    "<cstdint>",
    "<cinttypes>",
    "<limits>",
    "<exception>",
    "<stdexcept>",
    "<cassert>",
    "<system_error>",
    "<cerrno>",
    "<cctype>",
    "<cwctype>",
    "<cstring>",
    "<cwchar>",
    "<cuchar>",
    "<string>",
    "<array>",
    "<vector>",
    "<deque>",
    "<list>",
    "<forward_list>",
    "<set>",
    "<map>",
    "<unordered_set>",
    "<unordered_map>",
    "<stack>",
    "<queue>",
    "<algorithm>",
    "<iterator>",
    "<cmath>",
    "<complex>",
    "<valarray>",
    "<random>",
    "<numeric>",
    "<ratio>",
    "<cfenv>",
    "<iosfwd>",
    "<ios>",
    "<istream>",
    "<ostream>",
    "<iostream>",
    "<fstream>",
    "<sstream>",
    "<strstream>",
    "<iomanip>",
    "<streambuf>",
    "<cstdio>",
    "<locale>",
    "<clocale>",
    "<codecvt>",
    "<regex>",
    "<atomic>",
    "<thread>",
    "<mutex>",
    "<shared_mutex>",
    "<future>",
    "<condition_variable>",
    "<ciso646>",
    "<ccomplex>",
    "<ctgmath>",
    "<cstdalign>",
    "<cstdbool>",
    "<assert.h>",
    "<complex.h>",
    "<ctype.h>",
    "<errno.h>",
    "<fenv.h>",
    "<float.h>",
    "<inttypes.h>",
    "<iso646.h>",
    "<limits.h>",
    "<locale.h>",
    "<math.h>",
    "<setjmp.h>",
    "<signal.h>",
    "<stdalign.h>",
    "<stdarg.h>",
    "<stdbool.h>",
    "<stddef.h>",
    "<stdint.h>",
    "<stdio.h>",
    "<stdlib.h>",
    "<string.h>",
    "<tgmath.h>",
    "<time.h>",
    "<uchar.h>",
    "<wchar.h>",
    "<wctype.h>"
]


def stripDirs(s):
    return os.path.basename(s)


def stripExt(s):
    return '.'.join(s.split('.')[:-1])


def isOwnHeader(iface, impl):
    ifaceName = stripExt(stripDirs(iface.strip('"<>')))
    implName = stripExt(stripDirs(impl))
    return ifaceName == implName

# vim:set shiftwidth=4 softtabstop=4 expandtab:
