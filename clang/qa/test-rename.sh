#!/usr/bin/env bash

test_assert_fail()
{
    if "$@" 2>/dev/null; then
        echo "Error: assertion failure in ${test_name}."
        exit 1
    fi
}

test_assert_equal()
{
    if ! diff -u $1 $2; then
        echo "Error: assertion failure in ${test_name}."
        exit 1
    fi
}

declare_rename_test()
{
    test_name="Rename::${1}"
    test_input="qa/data/${2}"
    test_output="qa/data/${2}.new-rename"
    test_expected="qa/data/${2}.expected"
}

declare_rename_test "testFieldDecl" "rename-field-decl.cxx"
bin/rename -old-name=C::nX -new-name=m_nX $test_input --
test_assert_equal $test_expected $test_output

# Do the same as previously, but trigger the csv parser this time.
declare_rename_test "testFieldDeclCsv" "rename-field-decl.cxx"
bin/rename -csv=qa/data/rename-field-decl.csv $test_input --
test_assert_equal $test_expected $test_output

# Test that we fail on non-existing -csv parameter.
declare_rename_test "testFieldDeclCsvFail" "rename-field-decl.cxx"
test_assert_fail bin/rename -csv=qa/data/rename-field-decl.cvs $test_input --

# Test that the first column can't be empty.
declare_rename_test "testFieldDeclCsvFailCol1Empty" "rename-field-decl.cxx"
test_assert_fail bin/rename -csv=qa/data/rename-field-decl.csv-emptycol1 $test_input --

# Test that the second column can't be empty.
declare_rename_test "testFieldDeclCsvFailCol2Empty" "rename-field-decl.cxx"
test_assert_fail bin/rename -csv=qa/data/rename-field-decl.csv-emptycol2 $test_input --

# Test that rename fails without options.
declare_rename_test "testFieldDeclCsvFailNoopt" "rename-field-decl.cxx"
test_assert_fail bin/rename qa/data/rename-field-decl.cxx --

declare_rename_test "testVarDecl" "rename-var-decl.cxx"
bin/rename -old-name=C::aS -new-name=m_aS $test_input --
test_assert_equal $test_expected $test_output

declare_rename_test "testVarDeclClass" "rename-var-decl-class.cxx"
bin/rename -old-name=C -new-name=D $test_input --
test_assert_equal $test_expected $test_output

declare_rename_test "testCXXConstructorDecl" "rename-cxx-constructor-decl.cxx"
bin/rename -csv=qa/data/rename-cxx-constructor-decl.csv $test_input --
test_assert_equal $test_expected $test_output

declare_rename_test "testCXXConstructorDeclClass" "rename-cxx-constructor-decl-class.cxx"
bin/rename -old-name=C -new-name=D $test_input --
test_assert_equal $test_expected $test_output

declare_rename_test "testMemberExpr" "rename-member-expr.cxx"
bin/rename -old-name=C::nX -new-name=m_nX $test_input --
test_assert_equal $test_expected $test_output

declare_rename_test "testDeclRefExpr" "rename-decl-ref-expr.cxx"
bin/rename -old-name=C::aS -new-name=m_aS $test_input --
test_assert_equal $test_expected $test_output

declare_rename_test "testCXXMethodDecl" "rename-cxx-method-decl.cxx"
bin/rename -old-name=C::foo -new-name=bar $test_input --
test_assert_equal $test_expected $test_output

declare_rename_test "testCXXConstructExpr" "rename-cxx-constructor-expr.cxx"
bin/rename -old-name=C -new-name=D $test_input --
test_assert_equal $test_expected $test_output

declare_rename_test "testCXXStaticCastExpr" "rename-cxx-static-cast-expr.cxx"
bin/rename -old-name=C -new-name=D $test_input --
test_assert_equal $test_expected $test_output

# vi:set shiftwidth=4 expandtab:
