#!/usr/bin/env bash

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

declare_rename_test "testVarDecl" "rename-var-decl.cxx"
bin/rename -old-name=C::aS -new-name=m_aS $test_input --
test_assert_equal $test_expected $test_output

declare_rename_test "testVarDeclClass" "rename-var-decl-class.cxx"
bin/rename -old-name=C -new-name=D $test_input --
test_assert_equal $test_expected $test_output

# vi:set shiftwidth=4 expandtab:
