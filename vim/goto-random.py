import random
import vim


# Jumps to a random line inside the current buffer. Helpful if you have lots of
# testcases inside a single file and you want to minimize conflicts, i.e. just
# appending tests to the end of the file is a bad strategy.
def main():
    # Add an entry to the jump list.
    vim.command("normal! m'")

    # Jump to a line.
    line = random.choice(range(len(vim.current.buffer)))
    # cursor() is 1-based.
    vim.eval("cursor(" + str(line + 1) + ", 0)")

    # Move the cursor to the center of the screen.
    vim.command("normal! zz")

if __name__ == '__main__':
    main()

# vim: set shiftwidth=4 softtabstop=4 expandtab:
