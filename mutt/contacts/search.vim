"
" This Source Code Form is subject to the terms of the Mozilla Public
" License, v. 2.0. If a copy of the MPL was not distributed with this
" file, You can obtain one at http://mozilla.org/MPL/2.0/.
"
" VIM integration for gcontacts.
"
" Usage example:
" augroup Mutt
"     autocmd!
"     if filereadable($HOME . "/path/to/mutt/contacts/search.vim")
"             source $HOME/path/to/mutt/contacts/search.vim
"     endif
"     autocmd Filetype mail set omnifunc=MuttContactsCompletion
" augroup END

" Completion wrapper around gcontacts.
function! MuttContactsCompletion(findstart, base)
    if a:findstart == 1
        " In findstart mode, look for the beginning of the current identifier
        let l:line = getline('.')
        let l:start = col('.') - 1
        while l:start > 0 && l:line[l:start - 1] =~ '\i'
            let l:start -= 1
        endwhile
        return l:start
    endif

    let l:ret = []

    if a:base != ""
        let l:ret = systemlist($HOME . "/git/vmexam/mutt/contacts/search " .  shellescape(a:base))
    endif

    return l:ret
endfunction

" vim:set shiftwidth=4 softtabstop=4 expandtab:
